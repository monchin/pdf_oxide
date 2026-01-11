//! Individual PDF/A validation functions.
//!
//! This module contains the validation logic for different aspects of PDF/A compliance.

use super::types::{
    ComplianceError, ComplianceWarning, ErrorCode, PdfALevel, ValidationResult, WarningCode,
};
use crate::document::PdfDocument;
use crate::error::Result;
use crate::object::Object;
use std::collections::HashMap;

/// Type alias for PDF dictionary.
type Dictionary = HashMap<String, Object>;

/// Helper to extract the catalog dictionary from a document.
///
/// Returns `None` if the catalog is not a dictionary, allowing callers
/// to handle this case appropriately.
fn get_catalog_dict(document: &mut PdfDocument) -> Result<Option<Dictionary>> {
    let catalog = document.catalog()?;
    match catalog {
        Object::Dictionary(d) => Ok(Some(d)),
        _ => Ok(None),
    }
}

/// Validate XMP metadata requirements.
pub fn validate_xmp_metadata(
    document: &mut PdfDocument,
    level: PdfALevel,
    result: &mut ValidationResult,
) -> Result<()> {
    // Get the catalog
    let catalog_dict = match get_catalog_dict(document)? {
        Some(d) => d,
        None => {
            result.add_error(ComplianceError::new(
                ErrorCode::MissingXmpMetadata,
                "Document catalog is invalid",
            ));
            return Ok(());
        },
    };

    // Check for /Metadata entry
    if !catalog_dict.contains_key("Metadata") {
        result.add_error(
            ComplianceError::new(
                ErrorCode::MissingXmpMetadata,
                "Document is missing XMP metadata stream",
            )
            .with_clause("6.7.2"),
        );
        return Ok(());
    }

    // TODO: Parse XMP metadata and check for PDF/A identification
    // For now, we just check that the Metadata entry exists

    // The XMP metadata should contain:
    // - pdfaid:part (1, 2, or 3)
    // - pdfaid:conformance (A, B, or U)

    // Suppress unused variable warning
    let _ = level;

    Ok(())
}

/// Validate font embedding requirements.
///
/// Note: This is a simplified version that checks the document catalog.
/// Full font validation requires the rendering feature for page resource access.
pub fn validate_fonts(
    document: &mut PdfDocument,
    level: PdfALevel,
    result: &mut ValidationResult,
) -> Result<()> {
    // Font validation requires accessing page resources, which needs the rendering feature.
    // For now, we'll just note that fonts should be checked.
    // The full validation will be available when rendering feature is enabled.

    result.add_warning(ComplianceWarning::new(
        WarningCode::PartialCheck,
        "Font embedding check requires rendering feature for full validation",
    ));

    // Suppress unused variable warning
    let _ = level;
    let _ = document;

    Ok(())
}

/// Validate color space requirements.
pub fn validate_colors(
    document: &mut PdfDocument,
    _level: PdfALevel,
    result: &mut ValidationResult,
) -> Result<()> {
    // Check for output intent
    let catalog_dict = match get_catalog_dict(document)? {
        Some(d) => d,
        None => return Ok(()),
    };

    let has_output_intent = catalog_dict.contains_key("OutputIntents");

    if !has_output_intent {
        // Without output intent, device colors may not be allowed
        result.add_warning(ComplianceWarning::new(
            WarningCode::MissingRecommendedMetadata,
            "No output intent specified; device-dependent colors may cause issues",
        ));
    }

    // TODO: Validate color spaces in content streams
    // This requires parsing content streams and checking:
    // - DeviceRGB, DeviceCMYK, DeviceGray usage
    // - ICC-based color spaces
    // - Spot colors

    Ok(())
}

/// Validate encryption (must be absent).
///
/// PDF/A documents must not be encrypted.
pub fn validate_encryption(
    document: &mut PdfDocument,
    _level: PdfALevel,
    result: &mut ValidationResult,
) -> Result<()> {
    // Check the trailer for /Encrypt entry
    let trailer = document.trailer();
    let is_encrypted = if let Object::Dictionary(trailer_dict) = trailer {
        trailer_dict.contains_key("Encrypt")
    } else {
        false
    };

    if is_encrypted {
        result.add_error(
            ComplianceError::new(
                ErrorCode::EncryptionNotAllowed,
                "PDF/A documents must not be encrypted",
            )
            .with_clause("6.1.4"),
        );
    }
    Ok(())
}

/// Validate transparency usage (PDF/A-1 restriction).
///
/// Note: Full transparency validation requires the rendering feature.
pub fn validate_transparency(
    document: &mut PdfDocument,
    level: PdfALevel,
    result: &mut ValidationResult,
) -> Result<()> {
    if level.allows_transparency() {
        return Ok(());
    }

    // Full transparency check requires page resources (rendering feature).
    // For now, add a warning about partial checking.
    result.add_warning(ComplianceWarning::new(
        WarningCode::PartialCheck,
        "Full transparency validation requires rendering feature",
    ));

    // Suppress unused variable warning
    let _ = document;

    Ok(())
}

/// Validate document structure (for level A).
pub fn validate_structure(
    document: &mut PdfDocument,
    level: PdfALevel,
    result: &mut ValidationResult,
) -> Result<()> {
    if !level.requires_structure() {
        return Ok(());
    }

    let catalog_dict = match get_catalog_dict(document)? {
        Some(d) => d,
        None => return Ok(()),
    };

    // Check for MarkInfo with Marked = true
    let is_marked = if let Some(mark_info) = catalog_dict.get("MarkInfo") {
        let resolved_mark_info = document.resolve_references(mark_info, 1)?;
        if let Object::Dictionary(mi) = resolved_mark_info {
            matches!(mi.get("Marked"), Some(Object::Boolean(true)))
        } else {
            false
        }
    } else {
        false
    };

    if !is_marked {
        result.add_error(
            ComplianceError::new(
                ErrorCode::MissingDocumentStructure,
                "Document must be marked (Tagged PDF) for PDF/A level A conformance",
            )
            .with_clause("6.8"),
        );
    }

    // Check for StructTreeRoot
    if !catalog_dict.contains_key("StructTreeRoot") {
        result.add_error(
            ComplianceError::new(
                ErrorCode::MissingDocumentStructure,
                "Document must have a structure tree for PDF/A level A conformance",
            )
            .with_clause("6.8"),
        );
    }

    // Check for Lang (language specification)
    if !catalog_dict.contains_key("Lang") {
        result.add_error(
            ComplianceError::new(
                ErrorCode::MissingLanguage,
                "Document must specify a primary language for PDF/A level A conformance",
            )
            .with_clause("6.8.1"),
        );
    }

    Ok(())
}

/// Validate JavaScript (must be absent).
pub fn validate_javascript(
    document: &mut PdfDocument,
    _level: PdfALevel,
    result: &mut ValidationResult,
) -> Result<()> {
    let catalog_dict = match get_catalog_dict(document)? {
        Some(d) => d,
        None => return Ok(()),
    };

    // Check Names dictionary for JavaScript
    if let Some(names) = catalog_dict.get("Names") {
        let resolved_names = document.resolve_references(names, 1)?;
        if let Object::Dictionary(names_dict) = resolved_names {
            if names_dict.contains_key("JavaScript") {
                result.add_error(
                    ComplianceError::new(
                        ErrorCode::JavaScriptNotAllowed,
                        "JavaScript is not allowed in PDF/A documents",
                    )
                    .with_clause("6.6.1"),
                );
            }
        }
    }

    // Check for OpenAction with JavaScript
    if let Some(open_action) = catalog_dict.get("OpenAction") {
        let resolved_action = document.resolve_references(open_action, 1)?;
        if let Object::Dictionary(action) = resolved_action {
            if let Some(Object::Name(s)) = action.get("S") {
                if s == "JavaScript" {
                    result.add_error(
                        ComplianceError::new(
                            ErrorCode::JavaScriptNotAllowed,
                            "JavaScript OpenAction is not allowed in PDF/A documents",
                        )
                        .with_clause("6.6.1"),
                    );
                }
            }
        }
    }

    Ok(())
}

/// Validate embedded files (PDF/A-1/2 restriction, PDF/A-3 requirements).
pub fn validate_embedded_files(
    document: &mut PdfDocument,
    level: PdfALevel,
    result: &mut ValidationResult,
) -> Result<()> {
    let catalog_dict = match get_catalog_dict(document)? {
        Some(d) => d,
        None => return Ok(()),
    };

    // Check Names dictionary for EmbeddedFiles
    let has_embedded_files = if let Some(names) = catalog_dict.get("Names") {
        let resolved_names = document.resolve_references(names, 1)?;
        if let Object::Dictionary(names_dict) = resolved_names {
            names_dict.contains_key("EmbeddedFiles")
        } else {
            false
        }
    } else {
        false
    };

    if has_embedded_files {
        if !level.allows_embedded_files() {
            result.add_error(
                ComplianceError::new(
                    ErrorCode::EmbeddedFileNotAllowed,
                    format!("Embedded files are not allowed in {} (only PDF/A-3)", level),
                )
                .with_clause("6.9"),
            );
        } else {
            // For PDF/A-3, check that files have AF relationship
            // TODO: Validate AFRelationship entries
            result.add_warning(ComplianceWarning::new(
                WarningCode::MissingRecommendedMetadata,
                "Embedded files should have AFRelationship specified",
            ));
        }
    }

    Ok(())
}

/// Validate annotations.
///
/// Note: Full annotation validation requires page access. This checks catalog-level
/// info about annotations.
pub fn validate_annotations(
    document: &mut PdfDocument,
    _level: PdfALevel,
    result: &mut ValidationResult,
) -> Result<()> {
    // Full annotation validation requires page access which needs iterating through pages.
    // For now, add a note about partial checking.

    result.add_warning(ComplianceWarning::new(
        WarningCode::PartialCheck,
        "Full annotation validation requires page-level access",
    ));

    // Suppress unused variable warning
    let _ = document;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_updates() {
        let mut result = ValidationResult::new(PdfALevel::A2b);

        validate_encryption_mock(&mut result, false);
        assert!(!result.has_errors());

        validate_encryption_mock(&mut result, true);
        assert!(result.has_errors());
    }

    fn validate_encryption_mock(result: &mut ValidationResult, is_encrypted: bool) {
        if is_encrypted {
            result.add_error(ComplianceError::new(
                ErrorCode::EncryptionNotAllowed,
                "Document is encrypted",
            ));
        }
    }
}
