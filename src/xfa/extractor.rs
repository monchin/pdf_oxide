//! XFA data extraction from PDF documents.
//!
//! This module provides utilities to detect and extract XFA form data
//! from PDF documents.

use crate::decoders::decode_stream;
use crate::document::PdfDocument;
use crate::error::{Error, Result};
use crate::object::Object;

/// XFA data extractor.
///
/// Provides static methods to detect and extract XFA form data from PDF documents.
pub struct XfaExtractor;

impl XfaExtractor {
    /// Check if a PDF document contains an XFA form.
    ///
    /// XFA forms are indicated by the presence of an /XFA entry in the /AcroForm
    /// dictionary of the document catalog.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use pdf_oxide::PdfDocument;
    /// use pdf_oxide::xfa::XfaExtractor;
    ///
    /// let mut doc = PdfDocument::open("form.pdf")?;
    /// if XfaExtractor::has_xfa(&mut doc)? {
    ///     println!("Document contains XFA form");
    /// }
    /// ```
    pub fn has_xfa(doc: &mut PdfDocument) -> Result<bool> {
        let catalog = doc.catalog()?;
        let catalog_dict = match catalog.as_dict() {
            Some(d) => d,
            None => return Ok(false),
        };

        let acroform_obj = match catalog_dict.get("AcroForm") {
            Some(obj) => Self::resolve_object(doc, obj)?,
            None => return Ok(false),
        };

        let acroform_dict = match acroform_obj.as_dict() {
            Some(d) => d,
            None => return Ok(false),
        };

        Ok(acroform_dict.contains_key("XFA"))
    }

    /// Extract XFA data from a PDF document.
    ///
    /// XFA data in PDFs can be stored as:
    /// 1. A single stream containing the complete XDP document
    /// 2. An array of alternating name/stream pairs for different XFA packets
    ///
    /// This method returns the raw XFA data bytes.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use pdf_oxide::PdfDocument;
    /// use pdf_oxide::xfa::{XfaExtractor, XfaParser};
    ///
    /// let mut doc = PdfDocument::open("form.pdf")?;
    /// if let Ok(xfa_data) = XfaExtractor::extract_xfa(&mut doc) {
    ///     let mut parser = XfaParser::new();
    ///     let form = parser.parse(&xfa_data)?;
    ///     println!("Found {} fields", form.field_count());
    /// }
    /// ```
    pub fn extract_xfa(doc: &mut PdfDocument) -> Result<Vec<u8>> {
        let catalog = doc.catalog()?;
        let catalog_dict = catalog
            .as_dict()
            .ok_or_else(|| Error::InvalidPdf("Catalog is not a dictionary".to_string()))?;

        let acroform_obj = catalog_dict
            .get("AcroForm")
            .ok_or_else(|| Error::InvalidPdf("No AcroForm in document".to_string()))?;
        let acroform_obj = Self::resolve_object(doc, acroform_obj)?;

        let acroform_dict = acroform_obj
            .as_dict()
            .ok_or_else(|| Error::InvalidPdf("AcroForm is not a dictionary".to_string()))?;

        let xfa_obj = acroform_dict
            .get("XFA")
            .ok_or_else(|| Error::InvalidPdf("No XFA entry in AcroForm".to_string()))?;
        let xfa_obj = Self::resolve_object(doc, xfa_obj)?;

        match &xfa_obj {
            Object::Stream { dict, data } => {
                // Single stream containing complete XDP
                Self::decode_stream_data(dict, data)
            },
            Object::Array(arr) => {
                // Array of name/stream pairs
                // Combine all streams into single XFA document
                Self::extract_xfa_array(doc, arr)
            },
            _ => Err(Error::InvalidPdf("XFA entry is neither stream nor array".to_string())),
        }
    }

    /// Extract XFA packets from an array format.
    ///
    /// Returns a vector of (name, data) pairs for each XFA packet.
    pub fn extract_xfa_packets(doc: &mut PdfDocument) -> Result<Vec<(String, Vec<u8>)>> {
        let catalog = doc.catalog()?;
        let catalog_dict = catalog
            .as_dict()
            .ok_or_else(|| Error::InvalidPdf("Catalog is not a dictionary".to_string()))?;

        let acroform_obj = catalog_dict
            .get("AcroForm")
            .ok_or_else(|| Error::InvalidPdf("No AcroForm in document".to_string()))?;
        let acroform_obj = Self::resolve_object(doc, acroform_obj)?;

        let acroform_dict = acroform_obj
            .as_dict()
            .ok_or_else(|| Error::InvalidPdf("AcroForm is not a dictionary".to_string()))?;

        let xfa_obj = acroform_dict
            .get("XFA")
            .ok_or_else(|| Error::InvalidPdf("No XFA entry in AcroForm".to_string()))?;
        let xfa_obj = Self::resolve_object(doc, xfa_obj)?;

        match &xfa_obj {
            Object::Stream { dict, data } => {
                // Single stream - return as single "xdp" packet
                let decoded = Self::decode_stream_data(dict, data)?;
                Ok(vec![("xdp".to_string(), decoded)])
            },
            Object::Array(arr) => {
                // Array of name/stream pairs
                Self::extract_xfa_packets_from_array(doc, arr)
            },
            _ => Err(Error::InvalidPdf("XFA entry is neither stream nor array".to_string())),
        }
    }

    /// Resolve an indirect reference to its object.
    fn resolve_object(doc: &mut PdfDocument, obj: &Object) -> Result<Object> {
        if let Some(ref_val) = obj.as_reference() {
            doc.load_object(ref_val)
        } else {
            Ok(obj.clone())
        }
    }

    /// Decode a stream's data using its filter chain.
    fn decode_stream_data(
        dict: &std::collections::HashMap<String, Object>,
        data: &[u8],
    ) -> Result<Vec<u8>> {
        // Get filters from dictionary
        let filters = Self::get_filters(dict);
        if filters.is_empty() {
            // No filters, return raw data
            Ok(data.to_vec())
        } else {
            decode_stream(data, &filters)
        }
    }

    /// Extract filter names from stream dictionary.
    fn get_filters(dict: &std::collections::HashMap<String, Object>) -> Vec<String> {
        let filter_obj = dict.get("Filter");
        match filter_obj {
            Some(Object::Name(n)) => vec![n.clone()],
            Some(Object::Array(arr)) => arr
                .iter()
                .filter_map(|o| {
                    if let Object::Name(n) = o {
                        Some(n.clone())
                    } else {
                        None
                    }
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    /// Extract and combine XFA data from an array of packets.
    fn extract_xfa_array(doc: &mut PdfDocument, arr: &[Object]) -> Result<Vec<u8>> {
        let mut combined = Vec::new();

        // Array is [name1, stream1, name2, stream2, ...]
        let mut i = 0;
        while i < arr.len() {
            // Skip the name, get the stream
            if i + 1 < arr.len() {
                let stream_obj = Self::resolve_object(doc, &arr[i + 1])?;
                if let Object::Stream { dict, data } = &stream_obj {
                    let decoded = Self::decode_stream_data(dict, data)?;
                    combined.extend_from_slice(&decoded);
                }
            }
            i += 2;
        }

        Ok(combined)
    }

    /// Extract packets from XFA array with names preserved.
    fn extract_xfa_packets_from_array(
        doc: &mut PdfDocument,
        arr: &[Object],
    ) -> Result<Vec<(String, Vec<u8>)>> {
        let mut packets = Vec::new();

        // Array is [name1, stream1, name2, stream2, ...]
        let mut i = 0;
        while i < arr.len() {
            if i + 1 < arr.len() {
                // Get the name
                let name = match &arr[i] {
                    Object::Name(n) => n.clone(),
                    Object::String(s) => String::from_utf8_lossy(s).to_string(),
                    _ => format!("packet_{}", i / 2),
                };

                // Get the stream data
                let stream_obj = Self::resolve_object(doc, &arr[i + 1])?;
                if let Object::Stream { dict, data } = &stream_obj {
                    let decoded = Self::decode_stream_data(dict, data)?;
                    packets.push((name, decoded));
                }
            }
            i += 2;
        }

        Ok(packets)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_xfa_extractor_no_xfa() {
        // Create a minimal PDF document without XFA
        // For now, this is a placeholder test
        // Real tests would require a test PDF fixture
    }
}
