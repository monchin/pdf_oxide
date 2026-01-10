//! High-level PDF API for simple document creation and manipulation.
//!
//! This module provides a unified, easy-to-use API for common PDF operations:
//! - Creating PDFs from Markdown, HTML, or plain text
//! - Opening and editing existing PDFs with DOM-like navigation
//! - Querying and modifying document content
//! - Converting between formats
//!
//! ## Quick Start - Creating PDFs
//!
//! ```ignore
//! use pdf_oxide::api::Pdf;
//!
//! // Create from Markdown
//! let mut pdf = Pdf::from_markdown("# Hello World\n\nThis is a PDF.")?;
//! pdf.save("output.pdf")?;
//!
//! // Create from HTML
//! let mut pdf = Pdf::from_html("<h1>Hello</h1><p>World</p>")?;
//! pdf.save("output.pdf")?;
//! ```
//!
//! ## Opening and Editing PDFs with DOM Access
//!
//! ```ignore
//! use pdf_oxide::api::Pdf;
//!
//! // Open existing PDF
//! let mut doc = Pdf::open("input.pdf")?;
//!
//! // Get a page for DOM-like navigation
//! let page = doc.page(0)?;
//!
//! // Query elements
//! for text in page.find_text_containing("Hello") {
//!     println!("Found: {} at {:?}", text.text(), text.bbox());
//! }
//!
//! // Modify content
//! let mut page = doc.page(0)?;
//! let texts = page.find_text_containing("old");
//! for t in &texts {
//!     page.set_text(t.id(), "new")?;
//! }
//! doc.save_page(page)?;
//!
//! // Navigate DOM tree
//! let page = doc.page(0)?;
//! for element in page.children() {
//!     match element {
//!         PdfElement::Text(t) => println!("Text: {}", t.text()),
//!         PdfElement::Image(i) => println!("Image: {}x{}", i.width(), i.height()),
//!         _ => {}
//!     }
//! }
//!
//! // Save modifications
//! doc.save("output.pdf")?;
//! ```
//!
//! ## Builder Pattern
//!
//! For more control over PDF creation:
//!
//! ```ignore
//! use pdf_oxide::api::PdfBuilder;
//! use pdf_oxide::writer::PageSize;
//!
//! let mut pdf = PdfBuilder::new()
//!     .title("My Document")
//!     .author("John Doe")
//!     .page_size(PageSize::A4)
//!     .margins(72.0, 72.0, 72.0, 72.0)
//!     .from_markdown("# Content")?;
//! pdf.save("output.pdf")?;
//! ```

mod pdf_builder;

pub use pdf_builder::{Pdf, PdfBuilder, PdfConfig};

// Re-export DOM types for convenience
pub use crate::editor::{
    AnnotationId, AnnotationWrapper, ElementId, ImageInfo, PdfElement, PdfImage, PdfPage, PdfPath,
    PdfStructure, PdfTable, PdfText,
};

// Re-export encryption types for password protection
pub use crate::editor::{EncryptionAlgorithm, EncryptionConfig, Permissions};

// Re-export annotation types for creating annotations
pub use crate::writer::{
    Annotation, CaretAnnotation, FileAttachmentAnnotation, FreeTextAnnotation, HighlightMode,
    InkAnnotation, LineAnnotation, LinkAction, LinkAnnotation, PolygonAnnotation, PopupAnnotation,
    RedactAnnotation, ShapeAnnotation, StampAnnotation, StampType, TextAnnotation,
    TextMarkupAnnotation, WatermarkAnnotation,
};

// Re-export geometry types
pub use crate::geometry::Rect;

// Re-export element content types for adding new elements
pub use crate::elements::{ImageContent, PathContent, TableContent, TextContent};
