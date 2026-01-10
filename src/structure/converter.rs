//! Converter from StructElem to StructureElement.
//!
//! This module bridges the PDF spec-level structure representation (StructElem)
//! with the unified content API (StructureElement) for round-trip operations.
//!
//! PDF Spec: ISO 32000-1:2008, Section 14.7-14.8 (Structure Trees and Tagged PDF)

use crate::elements::{ContentElement, StructureElement};
use crate::error::Result;
use crate::geometry::Rect;
use crate::object::Object;
use crate::structure::types::{StructChild, StructElem, StructType};
use std::collections::HashMap;

/// Converter from PDF structure elements to unified content elements.
///
/// This converter handles:
/// - Recursive structure element hierarchy conversion
/// - MCID (Marked Content ID) to content mapping
/// - Accessibility attribute extraction (alt text, language)
/// - Standard structure type mapping
pub struct StructureConverter {
    /// Map from MCID to extracted content elements
    mcid_map: HashMap<u32, Vec<ContentElement>>,
}

impl StructureConverter {
    /// Create a new converter with an MCID to content mapping.
    ///
    /// # Arguments
    ///
    /// * `mcid_map` - HashMap of MCID values to their extracted content elements
    pub fn new(mcid_map: HashMap<u32, Vec<ContentElement>>) -> Self {
        Self { mcid_map }
    }

    /// Convert a StructElem to a StructureElement.
    ///
    /// This recursively converts the entire hierarchy, populating children
    /// with actual content elements where MCIDs are referenced.
    ///
    /// # Arguments
    ///
    /// * `elem` - The structure element to convert
    ///
    /// # Returns
    ///
    /// A StructureElement with populated children
    ///
    /// # PDF Spec Compliance
    ///
    /// - ISO 32000-1:2008, Section 14.7.2 - Structure Hierarchy
    /// - ISO 32000-1:2008, Section 14.7.4 - Marked Content Identification
    /// - ISO 32000-1:2008, Section 14.9.3 - Accessibility Attributes
    pub fn convert_struct_elem(&self, elem: &StructElem) -> Result<StructureElement> {
        let mut children = Vec::new();

        // Process all children
        for child in &elem.children {
            match child {
                StructChild::StructElem(nested) => {
                    // Recursive structure element
                    let nested_structure = self.convert_struct_elem(nested)?;
                    children.push(ContentElement::Structure(nested_structure));
                },
                StructChild::MarkedContentRef { mcid, page: _ } => {
                    // Lookup content by MCID
                    if let Some(content_elements) = self.mcid_map.get(mcid) {
                        children.extend(content_elements.clone());
                    }
                    // If MCID not found, silently skip (per spec, missing MCIDs may occur)
                },
                StructChild::ObjectRef(_, _) => {
                    // Object references to other struct elements are deferred
                    // In a full implementation, would resolve indirect references
                },
            }
        }

        // Calculate bounding box from children
        let bbox = Self::calculate_bbox(&children);

        // Extract accessibility attributes
        let alt_text = Self::extract_alt_text(&elem.attributes);
        let language = Self::extract_language(&elem.attributes);

        Ok(StructureElement {
            structure_type: Self::format_struct_type(&elem.struct_type),
            bbox,
            children,
            reading_order: None, // Will be set from parent tree if available
            alt_text,
            language,
        })
    }

    /// Calculate bounding box from children.
    ///
    /// Computes the minimal rectangle that encompasses all child elements.
    fn calculate_bbox(children: &[ContentElement]) -> Rect {
        if children.is_empty() {
            return Rect::new(0.0, 0.0, 0.0, 0.0);
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for child in children {
            let bbox = child.bbox();
            min_x = min_x.min(bbox.x);
            min_y = min_y.min(bbox.y);
            max_x = max_x.max(bbox.x + bbox.width);
            max_y = max_y.max(bbox.y + bbox.height);
        }

        if min_x == f32::MAX {
            Rect::new(0.0, 0.0, 0.0, 0.0)
        } else {
            Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
        }
    }

    /// Extract alternative text (alt text) from attributes.
    ///
    /// Per PDF Spec Section 14.9.3, alt text is stored in the `/Alt` attribute
    /// and provides a text description for accessibility.
    fn extract_alt_text(attributes: &HashMap<String, Object>) -> Option<String> {
        attributes.get("Alt").and_then(|obj| {
            if let Object::String(bytes) = obj {
                String::from_utf8(bytes.clone()).ok()
            } else {
                None
            }
        })
    }

    /// Extract language tag from attributes.
    ///
    /// Per PDF Spec Section 14.9.3, language tags are stored in the `/Lang` attribute
    /// as a string (e.g., "en-US", "fr", "de").
    fn extract_language(attributes: &HashMap<String, Object>) -> Option<String> {
        attributes.get("Lang").and_then(|obj| {
            if let Object::String(bytes) = obj {
                String::from_utf8(bytes.clone()).ok()
            } else {
                None
            }
        })
    }

    /// Format structure type for display.
    ///
    /// Converts StructType enum to human-readable string form.
    fn format_struct_type(struct_type: &StructType) -> String {
        match struct_type {
            StructType::Document => "Document".to_string(),
            StructType::Part => "Part".to_string(),
            StructType::Art => "Article".to_string(),
            StructType::Sect => "Section".to_string(),
            StructType::Div => "Division".to_string(),
            StructType::P => "P".to_string(),
            StructType::H => "H".to_string(),
            StructType::H1 => "H1".to_string(),
            StructType::H2 => "H2".to_string(),
            StructType::H3 => "H3".to_string(),
            StructType::H4 => "H4".to_string(),
            StructType::H5 => "H5".to_string(),
            StructType::H6 => "H6".to_string(),
            StructType::L => "List".to_string(),
            StructType::LI => "ListItem".to_string(),
            StructType::Lbl => "Label".to_string(),
            StructType::LBody => "ListBody".to_string(),
            StructType::Table => "Table".to_string(),
            StructType::THead => "TableHead".to_string(),
            StructType::TBody => "TableBody".to_string(),
            StructType::TFoot => "TableFoot".to_string(),
            StructType::TR => "TableRow".to_string(),
            StructType::TH => "TableHeader".to_string(),
            StructType::TD => "TableData".to_string(),
            StructType::Span => "Span".to_string(),
            StructType::Quote => "Quote".to_string(),
            StructType::Note => "Note".to_string(),
            StructType::Reference => "Reference".to_string(),
            StructType::BibEntry => "BibEntry".to_string(),
            StructType::Code => "Code".to_string(),
            StructType::Link => "Link".to_string(),
            StructType::Annot => "Annotation".to_string(),
            StructType::Figure => "Figure".to_string(),
            StructType::Formula => "Formula".to_string(),
            StructType::Form => "Form".to_string(),
            StructType::WB => "WordBreak".to_string(),
            StructType::Custom(name) => name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converter_creation() {
        let mcid_map = HashMap::new();
        let _converter = StructureConverter::new(mcid_map);
    }

    #[test]
    fn test_calculate_bbox_empty() {
        let bbox = StructureConverter::calculate_bbox(&[]);
        assert_eq!(bbox.x, 0.0);
        assert_eq!(bbox.y, 0.0);
        assert_eq!(bbox.width, 0.0);
        assert_eq!(bbox.height, 0.0);
    }

    #[test]
    fn test_extract_alt_text() {
        let mut attrs = HashMap::new();
        attrs.insert("Alt".to_string(), Object::String(b"Alt text".to_vec()));

        let alt_text = StructureConverter::extract_alt_text(&attrs);
        assert_eq!(alt_text, Some("Alt text".to_string()));
    }

    #[test]
    fn test_extract_language() {
        let mut attrs = HashMap::new();
        attrs.insert("Lang".to_string(), Object::String(b"en-US".to_vec()));

        let lang = StructureConverter::extract_language(&attrs);
        assert_eq!(lang, Some("en-US".to_string()));
    }

    #[test]
    fn test_format_struct_type() {
        assert_eq!(StructureConverter::format_struct_type(&StructType::Document), "Document");
        assert_eq!(StructureConverter::format_struct_type(&StructType::H1), "H1");
        assert_eq!(StructureConverter::format_struct_type(&StructType::P), "P");
        assert_eq!(StructureConverter::format_struct_type(&StructType::Table), "Table");
    }
}
