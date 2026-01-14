//! DOM-like API for PDF editing with strongly-typed elements.
//!
//! This module provides a hierarchical, DOM-like interface for editing PDF content.
//! Instead of working with generic content types, this API returns strongly-typed
//! wrappers (PdfText, PdfImage, etc.) that provide domain-specific methods.
//!
//! # Annotation Support
//!
//! Pages can have annotations attached. Use `PdfPage::annotations()` to read them
//! and `PdfPage::add_annotation()` to add new ones:
//!
//! ```ignore
//! use pdf_oxide::editor::DocumentEditor;
//! use pdf_oxide::writer::LinkAnnotation;
//! use pdf_oxide::geometry::Rect;
//!
//! let mut editor = DocumentEditor::open("document.pdf")?;
//! let mut page = editor.get_page(0)?;
//!
//! // Read existing annotations
//! for annot in page.annotations() {
//!     println!("Annotation: {:?}", annot.subtype());
//! }
//!
//! // Add a new link annotation
//! let link = LinkAnnotation::uri(
//!     Rect::new(100.0, 700.0, 50.0, 12.0),
//!     "https://example.com",
//! );
//! page.add_annotation(link);
//! ```

use crate::annotation_types::AnnotationSubtype;
use crate::annotations::Annotation as ReadAnnotation;
use crate::elements::{
    ContentElement, ImageContent, LineCap, LineJoin, PathContent, PathOperation, StructureElement,
    TableCellContent, TableContent, TextContent,
};
use crate::geometry::Rect;
use crate::layout::Color;
use crate::writer::Annotation as WriteAnnotation;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Unique element identifier (UUID-based).
///
/// This ID is used to reference elements for modification and navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementId(Uuid);

impl ElementId {
    /// Generate a new unique element ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ElementId {
    fn default() -> Self {
        Self::new()
    }
}

/// Path to an element in the content tree.
///
/// Represented as a sequence of child indices: [idx0, idx1, idx2, ...]
#[derive(Debug, Clone)]
pub struct ElementPath {
    /// Sequence of child indices traversing from root to element.
    pub path: Vec<usize>,
}

impl ElementPath {
    fn new() -> Self {
        Self { path: Vec::new() }
    }

    fn with_child(&self, idx: usize) -> Self {
        let mut path = self.path.clone();
        path.push(idx);
        Self { path }
    }
}

/// Strongly-typed text element with DOM capabilities.
#[derive(Debug, Clone)]
pub struct PdfText {
    pub id: ElementId,
    pub content: TextContent,
    pub path: ElementPath,
}

impl PdfText {
    pub fn id(&self) -> ElementId {
        self.id
    }

    pub fn text(&self) -> &str {
        &self.content.text
    }

    pub fn bbox(&self) -> Rect {
        self.content.bbox
    }

    pub fn font_name(&self) -> &str {
        &self.content.font.name
    }

    pub fn font_size(&self) -> f32 {
        self.content.font.size
    }

    pub fn is_bold(&self) -> bool {
        self.content.is_bold()
    }

    pub fn is_italic(&self) -> bool {
        self.content.is_italic()
    }

    pub fn color(&self) -> Color {
        self.content.style.color
    }

    /// Set text content (fluent API).
    pub fn set_text(&mut self, new_text: impl Into<String>) {
        self.content.text = new_text.into();
    }

    /// Modify style (fluent API).
    pub fn set_style(&mut self, style: crate::elements::TextStyle) {
        self.content.style = style;
    }

    // === Python-friendly aliases ===

    /// Get text content (alias for `text()`, Python-friendly).
    pub fn value(&self) -> &str {
        self.text()
    }

    /// Set text content (alias for `set_text()`, Python-friendly).
    pub fn set_value(&mut self, new_text: impl Into<String>) {
        self.set_text(new_text);
    }

    // === Text mutation helpers ===

    /// Append text to the current content.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut text = page.find_text_containing("Hello").first().unwrap();
    /// text.append(" World!");
    /// // text is now "Hello World!"
    /// ```
    pub fn append(&mut self, text: &str) {
        self.content.text.push_str(text);
    }

    /// Replace all occurrences of `old` with `new` in the text.
    ///
    /// Returns the number of replacements made.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut text = page.find_text_containing("foo").first().unwrap();
    /// let count = text.replace("foo", "bar");
    /// println!("Replaced {} occurrences", count);
    /// ```
    pub fn replace(&mut self, old: &str, new: &str) -> usize {
        let count = self.content.text.matches(old).count();
        self.content.text = self.content.text.replace(old, new);
        count
    }

    /// Clear the text content.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut text = page.find_text_containing("remove me").first().unwrap();
    /// text.clear();
    /// // text is now ""
    /// ```
    pub fn clear(&mut self) {
        self.content.text.clear();
    }

    /// Check if the text is empty.
    pub fn is_empty(&self) -> bool {
        self.content.text.is_empty()
    }

    /// Get the length of the text in bytes.
    pub fn len(&self) -> usize {
        self.content.text.len()
    }

    /// Check if the text contains a substring.
    pub fn contains(&self, needle: &str) -> bool {
        self.content.text.contains(needle)
    }

    /// Check if the text starts with a prefix.
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.content.text.starts_with(prefix)
    }

    /// Check if the text ends with a suffix.
    pub fn ends_with(&self, suffix: &str) -> bool {
        self.content.text.ends_with(suffix)
    }

    // === Transformation accessors (v0.3.1, Issue #27) ===

    /// Get the baseline origin point if available.
    pub fn origin(&self) -> Option<crate::geometry::Point> {
        self.content.origin
    }

    /// Get the rotation angle in degrees if available.
    pub fn rotation_degrees(&self) -> Option<f32> {
        self.content.rotation_degrees
    }

    /// Get the rotation angle in radians if available.
    pub fn rotation_radians(&self) -> Option<f32> {
        self.content.rotation_radians()
    }

    /// Get the transformation matrix [a, b, c, d, e, f] if available.
    pub fn matrix(&self) -> Option<[f32; 6]> {
        self.content.get_matrix()
    }

    /// Check if this text is rotated (non-zero rotation).
    pub fn is_rotated(&self) -> bool {
        self.content.is_rotated()
    }

    /// Set the transformation matrix.
    pub fn set_matrix(&mut self, matrix: [f32; 6]) {
        self.content.matrix = Some(matrix);
    }

    /// Set the origin point.
    pub fn set_origin(&mut self, origin: crate::geometry::Point) {
        self.content.origin = Some(origin);
    }

    /// Set the rotation angle in degrees.
    pub fn set_rotation(&mut self, degrees: f32) {
        self.content.rotation_degrees = Some(degrees);
    }
}

/// Strongly-typed image element with DOM capabilities.
#[derive(Debug, Clone)]
pub struct PdfImage {
    pub id: ElementId,
    pub content: ImageContent,
    pub path: ElementPath,
}

impl PdfImage {
    pub fn id(&self) -> ElementId {
        self.id
    }

    pub fn bbox(&self) -> Rect {
        self.content.bbox
    }

    pub fn format(&self) -> crate::elements::ImageFormat {
        self.content.format
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.content.width, self.content.height)
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.content.aspect_ratio()
    }

    pub fn is_grayscale(&self) -> bool {
        self.content.is_grayscale()
    }

    pub fn alt_text(&self) -> Option<&str> {
        self.content.alt_text.as_deref()
    }

    /// Set alternative text (fluent API).
    pub fn set_alt_text(&mut self, alt: impl Into<String>) {
        self.content.alt_text = Some(alt.into());
    }

    // DPI methods (v0.3.1)

    /// Get the resolution as (horizontal_dpi, vertical_dpi).
    pub fn resolution(&self) -> Option<(f32, f32)> {
        self.content.resolution()
    }

    /// Get the horizontal DPI.
    pub fn horizontal_dpi(&self) -> Option<f32> {
        self.content.get_horizontal_dpi()
    }

    /// Get the vertical DPI.
    pub fn vertical_dpi(&self) -> Option<f32> {
        self.content.get_vertical_dpi()
    }

    /// Check if this image is high resolution (>= 300 DPI).
    pub fn is_high_resolution(&self) -> bool {
        self.content.is_high_resolution()
    }

    /// Check if this image is low resolution (< 150 DPI).
    pub fn is_low_resolution(&self) -> bool {
        self.content.is_low_resolution()
    }

    /// Check if this image is medium resolution (>= 150 and < 300 DPI).
    pub fn is_medium_resolution(&self) -> bool {
        self.content.is_medium_resolution()
    }
}

/// Strongly-typed path/graphics element with DOM capabilities.
#[derive(Debug, Clone)]
pub struct PdfPath {
    /// Element ID for DOM tracking.
    pub id: ElementId,
    /// Underlying path content.
    pub content: PathContent,
    /// Path in the content tree.
    pub path: ElementPath,
}

impl PdfPath {
    /// Get the element ID.
    pub fn id(&self) -> ElementId {
        self.id
    }

    /// Get the bounding box.
    pub fn bbox(&self) -> Rect {
        self.content.bbox
    }

    /// Get the path operations.
    pub fn operations(&self) -> &[PathOperation] {
        &self.content.operations
    }

    /// Get the stroke color.
    pub fn stroke_color(&self) -> Option<Color> {
        self.content.stroke_color
    }

    /// Get the fill color.
    pub fn fill_color(&self) -> Option<Color> {
        self.content.fill_color
    }

    /// Get the stroke width.
    pub fn stroke_width(&self) -> f32 {
        self.content.stroke_width
    }

    /// Set the stroke color.
    pub fn set_stroke_color(&mut self, color: Option<Color>) {
        self.content.stroke_color = color;
    }

    /// Set the fill color.
    pub fn set_fill_color(&mut self, color: Option<Color>) {
        self.content.fill_color = color;
    }

    /// Set the stroke width.
    pub fn set_stroke_width(&mut self, width: f32) {
        self.content.stroke_width = width;
    }

    /// Check if this path has a stroke.
    pub fn has_stroke(&self) -> bool {
        self.content.has_stroke()
    }

    /// Check if this path has a fill.
    pub fn has_fill(&self) -> bool {
        self.content.has_fill()
    }

    /// Get the line cap style.
    pub fn line_cap(&self) -> LineCap {
        self.content.line_cap
    }

    /// Get the line join style.
    pub fn line_join(&self) -> LineJoin {
        self.content.line_join
    }

    /// Check if this is a closed path.
    pub fn is_closed(&self) -> bool {
        self.content
            .operations
            .iter()
            .any(|op| matches!(op, PathOperation::ClosePath))
    }

    /// Convert this path to an SVG path string.
    ///
    /// Returns a complete SVG element that can be embedded in an SVG document.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let svg = path.to_svg();
    /// // Returns something like:
    /// // <path d="M 10 10 L 100 10 L 100 100 Z" stroke="black" fill="none"/>
    /// ```
    pub fn to_svg(&self) -> String {
        let mut d = String::new();

        for op in &self.content.operations {
            match op {
                PathOperation::MoveTo(x, y) => {
                    d.push_str(&format!("M {} {} ", x, y));
                },
                PathOperation::LineTo(x, y) => {
                    d.push_str(&format!("L {} {} ", x, y));
                },
                PathOperation::CurveTo(x1, y1, x2, y2, x3, y3) => {
                    d.push_str(&format!("C {} {} {} {} {} {} ", x1, y1, x2, y2, x3, y3));
                },
                PathOperation::Rectangle(x, y, w, h) => {
                    // SVG doesn't have a rectangle path command, so we expand it
                    d.push_str(&format!(
                        "M {} {} L {} {} L {} {} L {} {} Z ",
                        x,
                        y,
                        x + w,
                        y,
                        x + w,
                        y + h,
                        x,
                        y + h
                    ));
                },
                PathOperation::ClosePath => {
                    d.push_str("Z ");
                },
            }
        }

        let d = d.trim_end();

        // Build stroke attribute
        let stroke = if self.has_stroke() {
            if let Some(color) = self.stroke_color() {
                format!(
                    "stroke=\"rgb({},{},{})\" stroke-width=\"{}\"",
                    (color.r * 255.0) as u8,
                    (color.g * 255.0) as u8,
                    (color.b * 255.0) as u8,
                    self.stroke_width()
                )
            } else {
                "stroke=\"black\"".to_string()
            }
        } else {
            "stroke=\"none\"".to_string()
        };

        // Build fill attribute
        let fill = if self.has_fill() {
            if let Some(color) = self.fill_color() {
                format!(
                    "fill=\"rgb({},{},{})\"",
                    (color.r * 255.0) as u8,
                    (color.g * 255.0) as u8,
                    (color.b * 255.0) as u8
                )
            } else {
                "fill=\"black\"".to_string()
            }
        } else {
            "fill=\"none\"".to_string()
        };

        // Build line cap attribute
        let line_cap_attr = match self.line_cap() {
            LineCap::Butt => "",
            LineCap::Round => " stroke-linecap=\"round\"",
            LineCap::Square => " stroke-linecap=\"square\"",
        };

        // Build line join attribute
        let line_join_attr = match self.line_join() {
            LineJoin::Miter => "",
            LineJoin::Round => " stroke-linejoin=\"round\"",
            LineJoin::Bevel => " stroke-linejoin=\"bevel\"",
        };

        format!("<path d=\"{}\" {} {}{}{}/>", d, stroke, fill, line_cap_attr, line_join_attr)
    }

    /// Convert this path to an SVG document.
    ///
    /// Returns a complete SVG document with viewport set to the path's bounding box.
    pub fn to_svg_document(&self) -> String {
        let bbox = self.bbox();
        let path_element = self.to_svg();

        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="{} {} {} {}" width="{}" height="{}">
  {}
</svg>"#,
            bbox.x, bbox.y, bbox.width, bbox.height, bbox.width, bbox.height, path_element
        )
    }
}

/// Strongly-typed table element with DOM capabilities.
#[derive(Debug, Clone)]
pub struct PdfTable {
    /// Element ID for DOM tracking.
    pub id: ElementId,
    /// Underlying table content.
    pub content: TableContent,
    /// Path in the content tree.
    pub path: ElementPath,
}

impl PdfTable {
    /// Get the element ID.
    pub fn id(&self) -> ElementId {
        self.id
    }

    /// Get the bounding box.
    pub fn bbox(&self) -> Rect {
        self.content.bbox
    }

    /// Get the number of rows.
    pub fn row_count(&self) -> usize {
        self.content.row_count()
    }

    /// Get the number of columns.
    pub fn column_count(&self) -> usize {
        self.content.column_count()
    }

    /// Check if the table has a header row.
    pub fn has_header(&self) -> bool {
        self.content.has_header()
    }

    /// Get a cell at the specified row and column.
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&TableCellContent> {
        self.content.get_cell(row, col)
    }

    /// Get the table caption.
    pub fn caption(&self) -> Option<&str> {
        self.content.caption.as_deref()
    }

    /// Set the text of a cell at the specified row and column.
    /// Returns true if the cell was found and updated.
    pub fn set_cell_text(&mut self, row: usize, col: usize, text: impl Into<String>) -> bool {
        if let Some(row_content) = self.content.rows.get_mut(row) {
            if let Some(cell) = row_content.cells.get_mut(col) {
                cell.text = text.into();
                return true;
            }
        }
        false
    }

    /// Set the table caption.
    pub fn set_caption(&mut self, caption: impl Into<String>) {
        self.content.caption = Some(caption.into());
    }

    /// Get the detection confidence (if table was detected via heuristics).
    pub fn detection_confidence(&self) -> f32 {
        self.content.detection_confidence()
    }

    /// Check if table came from structure tree (Tagged PDF).
    pub fn is_from_structure_tree(&self) -> bool {
        self.content.is_from_structure_tree()
    }
}

/// Strongly-typed structure element with DOM capabilities.
#[derive(Debug, Clone)]
pub struct PdfStructure {
    id: ElementId,
    content: StructureElement,
    path: ElementPath,
}

impl PdfStructure {
    pub fn id(&self) -> ElementId {
        self.id
    }

    pub fn structure_type(&self) -> &str {
        &self.content.structure_type
    }

    pub fn bbox(&self) -> Rect {
        self.content.bbox
    }
}

/// Enum wrapper for mixed query results that can contain multiple element types.
#[derive(Debug, Clone)]
pub enum PdfElement {
    Text(PdfText),
    Image(PdfImage),
    Path(PdfPath),
    Table(PdfTable),
    Structure(PdfStructure),
}

impl PdfElement {
    pub fn as_text(&self) -> Option<&PdfText> {
        match self {
            PdfElement::Text(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_image(&self) -> Option<&PdfImage> {
        match self {
            PdfElement::Image(i) => Some(i),
            _ => None,
        }
    }

    pub fn as_path(&self) -> Option<&PdfPath> {
        match self {
            PdfElement::Path(p) => Some(p),
            _ => None,
        }
    }

    pub fn as_table(&self) -> Option<&PdfTable> {
        match self {
            PdfElement::Table(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_structure(&self) -> Option<&PdfStructure> {
        match self {
            PdfElement::Structure(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_text(&self) -> bool {
        matches!(self, PdfElement::Text(_))
    }

    pub fn is_image(&self) -> bool {
        matches!(self, PdfElement::Image(_))
    }

    pub fn is_path(&self) -> bool {
        matches!(self, PdfElement::Path(_))
    }

    pub fn is_table(&self) -> bool {
        matches!(self, PdfElement::Table(_))
    }

    pub fn is_structure(&self) -> bool {
        matches!(self, PdfElement::Structure(_))
    }

    pub fn bbox(&self) -> Rect {
        match self {
            PdfElement::Text(t) => t.bbox(),
            PdfElement::Image(i) => i.bbox(),
            PdfElement::Path(p) => p.bbox(),
            PdfElement::Table(t) => t.bbox(),
            PdfElement::Structure(s) => s.bbox(),
        }
    }

    pub fn id(&self) -> ElementId {
        match self {
            PdfElement::Text(t) => t.id(),
            PdfElement::Image(i) => i.id(),
            PdfElement::Path(p) => p.id(),
            PdfElement::Table(t) => t.id(),
            PdfElement::Structure(s) => s.id(),
        }
    }
}

// =============================================================================
// Annotation Support
// =============================================================================

/// Unique annotation identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnnotationId(Uuid);

impl AnnotationId {
    /// Generate a new unique annotation ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AnnotationId {
    fn default() -> Self {
        Self::new()
    }
}

/// Wrapper for annotations that provides a unified interface for both
/// existing (read from PDF) and new (to be written) annotations.
///
/// This wrapper tracks whether the annotation has been modified since loading.
#[derive(Debug, Clone)]
pub struct AnnotationWrapper {
    /// Unique ID for this annotation in the editing session
    id: AnnotationId,
    /// Original annotation data (if read from PDF)
    original: Option<ReadAnnotation>,
    /// New/modified annotation (for writing)
    writer_annotation: Option<WriteAnnotation>,
    /// Whether this annotation has been modified
    modified: bool,
}

impl AnnotationWrapper {
    /// Create a wrapper from an existing annotation (read from PDF).
    pub fn from_read(annotation: ReadAnnotation) -> Self {
        Self {
            id: AnnotationId::new(),
            original: Some(annotation),
            writer_annotation: None,
            modified: false,
        }
    }

    /// Create a wrapper from a new annotation (to be written).
    pub fn from_write<A: Into<WriteAnnotation>>(annotation: A) -> Self {
        Self {
            id: AnnotationId::new(),
            original: None,
            writer_annotation: Some(annotation.into()),
            modified: true,
        }
    }

    /// Get the annotation ID.
    pub fn id(&self) -> AnnotationId {
        self.id
    }

    /// Get the annotation subtype.
    pub fn subtype(&self) -> AnnotationSubtype {
        if let Some(ref original) = self.original {
            original.subtype_enum
        } else if let Some(ref writer) = self.writer_annotation {
            Self::writer_annotation_subtype(writer)
        } else {
            AnnotationSubtype::Unknown
        }
    }

    /// Get the bounding rectangle.
    pub fn rect(&self) -> Rect {
        if let Some(ref writer) = self.writer_annotation {
            writer.rect()
        } else if let Some(ref original) = self.original {
            if let Some([x1, y1, x2, y2]) = original.rect {
                Rect::new(x1 as f32, y1 as f32, (x2 - x1) as f32, (y2 - y1) as f32)
            } else {
                Rect::new(0.0, 0.0, 0.0, 0.0)
            }
        } else {
            Rect::new(0.0, 0.0, 0.0, 0.0)
        }
    }

    /// Get the annotation contents/text.
    pub fn contents(&self) -> Option<&str> {
        if let Some(ref original) = self.original {
            original.contents.as_deref()
        } else {
            None
        }
    }

    /// Get the annotation color as RGB (0.0-1.0 for each component).
    pub fn color(&self) -> Option<(f32, f32, f32)> {
        if let Some(ref original) = self.original {
            if let Some(ref color) = original.color {
                if color.len() >= 3 {
                    return Some((color[0] as f32, color[1] as f32, color[2] as f32));
                }
            }
        }
        None
    }

    /// Check if this annotation has been modified.
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Check if this is a new annotation (not loaded from PDF).
    pub fn is_new(&self) -> bool {
        self.original.is_none()
    }

    /// Get the writer annotation (for building PDF output).
    pub fn writer_annotation(&self) -> Option<&WriteAnnotation> {
        self.writer_annotation.as_ref()
    }

    /// Get the original read annotation.
    pub fn original(&self) -> Option<&ReadAnnotation> {
        self.original.as_ref()
    }

    /// Set the contents/text of the annotation.
    pub fn set_contents(&mut self, text: impl Into<String>) {
        if let Some(ref mut original) = self.original {
            original.contents = Some(text.into());
            self.modified = true;
        }
    }

    /// Set the rectangle bounds of the annotation.
    pub fn set_rect(&mut self, rect: Rect) {
        if let Some(ref mut original) = self.original {
            original.rect = Some([
                rect.x as f64,
                rect.y as f64,
                (rect.x + rect.width) as f64,
                (rect.y + rect.height) as f64,
            ]);
            self.modified = true;
        }
    }

    /// Set the color of the annotation (RGB, 0.0-1.0).
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        if let Some(ref mut original) = self.original {
            original.color = Some(vec![r as f64, g as f64, b as f64]);
            self.modified = true;
        }
    }

    /// Get the raw PDF dictionary from the original annotation (for round-trip preservation).
    ///
    /// This returns the complete original dictionary, enabling faithful preservation
    /// of properties that aren't explicitly parsed (appearance streams, popup references,
    /// vendor-specific extensions, etc.).
    pub fn raw_dict(&self) -> Option<&std::collections::HashMap<String, crate::object::Object>> {
        self.original.as_ref().and_then(|o| o.raw_dict.as_ref())
    }

    /// Helper to determine subtype from writer annotation.
    fn writer_annotation_subtype(annotation: &WriteAnnotation) -> AnnotationSubtype {
        match annotation {
            WriteAnnotation::Link(_) => AnnotationSubtype::Link,
            WriteAnnotation::TextMarkup(m) => {
                use crate::annotation_types::TextMarkupType;
                match m.markup_type {
                    TextMarkupType::Highlight => AnnotationSubtype::Highlight,
                    TextMarkupType::Underline => AnnotationSubtype::Underline,
                    TextMarkupType::StrikeOut => AnnotationSubtype::StrikeOut,
                    TextMarkupType::Squiggly => AnnotationSubtype::Squiggly,
                }
            },
            WriteAnnotation::Text(_) => AnnotationSubtype::Text,
            WriteAnnotation::FreeText(_) => AnnotationSubtype::FreeText,
            WriteAnnotation::Line(_) => AnnotationSubtype::Line,
            WriteAnnotation::Shape(s) => {
                use crate::writer::ShapeType;
                match s.shape_type {
                    ShapeType::Square => AnnotationSubtype::Square,
                    ShapeType::Circle => AnnotationSubtype::Circle,
                }
            },
            WriteAnnotation::Polygon(p) => {
                use crate::writer::PolygonType;
                match p.polygon_type {
                    PolygonType::Polygon => AnnotationSubtype::Polygon,
                    PolygonType::PolyLine => AnnotationSubtype::PolyLine,
                }
            },
            WriteAnnotation::Ink(_) => AnnotationSubtype::Ink,
            WriteAnnotation::Stamp(_) => AnnotationSubtype::Stamp,
            WriteAnnotation::Popup(_) => AnnotationSubtype::Popup,
            WriteAnnotation::Caret(_) => AnnotationSubtype::Caret,
            WriteAnnotation::FileAttachment(_) => AnnotationSubtype::FileAttachment,
            WriteAnnotation::Redact(_) => AnnotationSubtype::Redact,
            WriteAnnotation::Watermark(_) => AnnotationSubtype::Watermark,
            WriteAnnotation::Sound(_) => AnnotationSubtype::Sound,
            WriteAnnotation::Movie(_) => AnnotationSubtype::Movie,
            WriteAnnotation::Screen(_) => AnnotationSubtype::Screen,
            WriteAnnotation::ThreeD(_) => AnnotationSubtype::ThreeD,
            WriteAnnotation::RichMedia(_) => AnnotationSubtype::RichMedia,
        }
    }
}

/// Page with DOM-like editing capabilities.
#[derive(Clone)]
pub struct PdfPage {
    pub page_index: usize,
    pub root: StructureElement,
    element_map: HashMap<ElementId, ElementPath>,
    dirty_elements: HashSet<ElementId>,
    pub width: f32,
    pub height: f32,
    /// Annotations on this page
    annotations: Vec<AnnotationWrapper>,
    /// Track if annotations have been modified
    annotations_modified: bool,
}

impl PdfPage {
    /// Create a new PdfPage from a StructureElement.
    pub fn from_structure(
        page_index: usize,
        root: StructureElement,
        width: f32,
        height: f32,
    ) -> Self {
        let mut page = Self {
            page_index,
            root,
            element_map: HashMap::new(),
            dirty_elements: HashSet::new(),
            width,
            height,
            annotations: Vec::new(),
            annotations_modified: false,
        };
        page.rebuild_element_map();
        page
    }

    /// Create a PdfPage with pre-loaded annotations.
    pub fn from_structure_with_annotations(
        page_index: usize,
        root: StructureElement,
        width: f32,
        height: f32,
        annotations: Vec<AnnotationWrapper>,
    ) -> Self {
        let mut page = Self {
            page_index,
            root,
            element_map: HashMap::new(),
            dirty_elements: HashSet::new(),
            width,
            height,
            annotations,
            annotations_modified: false,
        };
        page.rebuild_element_map();
        page
    }

    /// Rebuild the element ID-to-path mapping.
    fn rebuild_element_map(&mut self) {
        self.element_map.clear();
        let children = self.root.children.clone();
        self.traverse_and_map(&children, ElementPath::new());
    }

    /// Traverse the tree and map element IDs to paths.
    fn traverse_and_map(&mut self, children: &[ContentElement], path: ElementPath) {
        for (idx, child) in children.iter().enumerate() {
            let child_path = path.with_child(idx);
            let id = ElementId::new();
            self.element_map.insert(id, child_path.clone());

            // Recursively traverse structure elements
            if let ContentElement::Structure(s) = child {
                self.traverse_and_map(&s.children, child_path);
            }
        }
    }

    /// Get the root element as a PdfElement.
    pub fn root(&self) -> PdfElement {
        let id = ElementId::new();
        PdfElement::Structure(PdfStructure {
            id,
            content: self.root.clone(),
            path: ElementPath::new(),
        })
    }

    /// Get all top-level children as strongly-typed elements.
    pub fn children(&self) -> Vec<PdfElement> {
        self.root
            .children
            .iter()
            .enumerate()
            .map(|(idx, child)| {
                let path = ElementPath::new().with_child(idx);
                let id = self.get_id_for_path(&path);
                self.wrap_element(id, path, child)
            })
            .collect()
    }

    /// Wrap a ContentElement with ID and path information.
    fn wrap_element(
        &self,
        id: ElementId,
        path: ElementPath,
        element: &ContentElement,
    ) -> PdfElement {
        match element {
            ContentElement::Text(t) => PdfElement::Text(PdfText {
                id,
                content: t.clone(),
                path,
            }),
            ContentElement::Image(i) => PdfElement::Image(PdfImage {
                id,
                content: i.clone(),
                path,
            }),
            ContentElement::Path(p) => PdfElement::Path(PdfPath {
                id,
                content: p.clone(),
                path,
            }),
            ContentElement::Table(t) => PdfElement::Table(PdfTable {
                id,
                content: t.clone(),
                path,
            }),
            ContentElement::Structure(s) => PdfElement::Structure(PdfStructure {
                id,
                content: s.clone(),
                path,
            }),
        }
    }

    /// Find text containing the specified needle string.
    pub fn find_text_containing(&self, needle: &str) -> Vec<PdfText> {
        self.find_text(|t| t.text().contains(needle))
    }

    /// Find text elements matching the predicate.
    pub fn find_text<F>(&self, predicate: F) -> Vec<PdfText>
    where
        F: Fn(&PdfText) -> bool,
    {
        let mut results = Vec::new();
        self.collect_text_recursive(
            &self.root.children,
            ElementPath::new(),
            &predicate,
            &mut results,
        );
        results
    }

    /// Recursively collect text elements matching predicate.
    fn collect_text_recursive<F>(
        &self,
        children: &[ContentElement],
        path: ElementPath,
        predicate: &F,
        results: &mut Vec<PdfText>,
    ) where
        F: Fn(&PdfText) -> bool,
    {
        for (idx, child) in children.iter().enumerate() {
            let child_path = path.with_child(idx);
            match child {
                ContentElement::Text(t) => {
                    let id = self.get_id_for_path(&child_path);
                    let pdf_text = PdfText {
                        id,
                        content: t.clone(),
                        path: child_path,
                    };
                    if predicate(&pdf_text) {
                        results.push(pdf_text);
                    }
                },
                ContentElement::Structure(s) => {
                    self.collect_text_recursive(&s.children, child_path, predicate, results);
                },
                _ => {},
            }
        }
    }

    /// Find all images on the page.
    pub fn find_images(&self) -> Vec<PdfImage> {
        self.find_images_internal(ElementPath::new())
    }

    /// Find images in a specific region.
    pub fn find_images_in_region(&self, region: Rect) -> Vec<PdfImage> {
        self.find_images()
            .into_iter()
            .filter(|img| {
                let bbox = img.bbox();
                // Check if image intersects with region
                bbox.x < region.x + region.width
                    && bbox.x + bbox.width > region.x
                    && bbox.y < region.y + region.height
                    && bbox.y + bbox.height > region.y
            })
            .collect()
    }

    /// Recursively collect image elements.
    fn find_images_internal(&self, path: ElementPath) -> Vec<PdfImage> {
        let mut results = Vec::new();
        self.collect_images_recursive(&self.root.children, path, &mut results);
        results
    }

    fn collect_images_recursive(
        &self,
        children: &[ContentElement],
        path: ElementPath,
        results: &mut Vec<PdfImage>,
    ) {
        for (idx, child) in children.iter().enumerate() {
            let child_path = path.with_child(idx);
            match child {
                ContentElement::Image(i) => {
                    let id = self.get_id_for_path(&child_path);
                    results.push(PdfImage {
                        id,
                        content: i.clone(),
                        path: child_path,
                    });
                },
                ContentElement::Structure(s) => {
                    self.collect_images_recursive(&s.children, child_path, results);
                },
                _ => {},
            }
        }
    }

    /// Find elements in a specific region.
    pub fn find_in_region(&self, region: Rect) -> Vec<PdfElement> {
        let mut results = Vec::new();
        self.collect_in_region_recursive(
            &self.root.children,
            ElementPath::new(),
            region,
            &mut results,
        );
        results
    }

    fn collect_in_region_recursive(
        &self,
        children: &[ContentElement],
        path: ElementPath,
        region: Rect,
        results: &mut Vec<PdfElement>,
    ) {
        for (idx, child) in children.iter().enumerate() {
            let child_path = path.with_child(idx);
            let bbox = child.bbox();

            // Check if element intersects with region
            if bbox.x < region.x + region.width
                && bbox.x + bbox.width > region.x
                && bbox.y < region.y + region.height
                && bbox.y + bbox.height > region.y
            {
                let id = self.get_id_for_path(&child_path);
                let element = self.wrap_element(id, child_path.clone(), child);
                results.push(element);
            }

            // Recurse into structures
            if let ContentElement::Structure(s) = child {
                self.collect_in_region_recursive(&s.children, child_path, region, results);
            }
        }
    }

    /// Modify text element by ID.
    pub fn modify_text<F>(&mut self, id: ElementId, f: F) -> crate::error::Result<()>
    where
        F: FnOnce(&mut TextContent),
    {
        if let Some(path) = self.element_map.get(&id).cloned() {
            self.modify_text_by_path(&path, f)?;
            self.dirty_elements.insert(id);
        }
        Ok(())
    }

    /// Set text content by ID.
    pub fn set_text(
        &mut self,
        id: ElementId,
        new_text: impl Into<String>,
    ) -> crate::error::Result<()> {
        self.modify_text(id, |t| {
            t.text = new_text.into();
        })
    }

    /// Modify text by path.
    fn modify_text_by_path<F>(&mut self, path: &ElementPath, f: F) -> crate::error::Result<()>
    where
        F: FnOnce(&mut TextContent),
    {
        let mut current = &mut self.root.children;

        for (i, &idx) in path.path.iter().enumerate() {
            if idx >= current.len() {
                return Ok(());
            }

            if i == path.path.len() - 1 {
                // Last index - modify the text element
                if let ContentElement::Text(ref mut text) = current[idx] {
                    f(text);
                }
                return Ok(());
            }

            // Navigate deeper
            if let ContentElement::Structure(ref mut structure) = current[idx] {
                current = &mut structure.children;
            } else {
                return Ok(());
            }
        }

        Ok(())
    }

    /// Set image alt text by ID.
    pub fn set_image_alt_text(
        &mut self,
        id: ElementId,
        alt: impl Into<String>,
    ) -> crate::error::Result<()> {
        if let Some(path) = self.element_map.get(&id).cloned() {
            self.modify_image_by_path(&path, |img| {
                img.alt_text = Some(alt.into());
            })?;
            self.dirty_elements.insert(id);
        }
        Ok(())
    }

    /// Modify image by path.
    fn modify_image_by_path<F>(&mut self, path: &ElementPath, f: F) -> crate::error::Result<()>
    where
        F: FnOnce(&mut ImageContent),
    {
        let mut current = &mut self.root.children;

        for (i, &idx) in path.path.iter().enumerate() {
            if idx >= current.len() {
                return Ok(());
            }

            if i == path.path.len() - 1 {
                // Last index - modify the image element
                if let ContentElement::Image(ref mut image) = current[idx] {
                    f(image);
                }
                return Ok(());
            }

            // Navigate deeper
            if let ContentElement::Structure(ref mut structure) = current[idx] {
                current = &mut structure.children;
            } else {
                return Ok(());
            }
        }

        Ok(())
    }

    /// Get element by ID (if it's still in the map).
    pub fn get_element(&self, id: ElementId) -> Option<PdfElement> {
        self.element_map.get(&id).and_then(|path| {
            self.get_element_by_path(path)
                .map(|elem| self.wrap_element(id, path.clone(), elem))
        })
    }

    /// Get element by path.
    fn get_element_by_path(&self, path: &ElementPath) -> Option<&ContentElement> {
        let mut current = &self.root.children;

        for &idx in &path.path {
            if idx >= current.len() {
                return None;
            }

            if let ContentElement::Structure(ref structure) = current[idx] {
                current = &structure.children;
            } else if path.path.last() == Some(&idx) {
                return Some(&current[idx]);
            } else {
                return None;
            }
        }

        None
    }

    /// Get parent element by ID.
    ///
    /// Returns `None` if the element is at the root level (top-level children of the page)
    /// or if the element is not found.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let page = doc.page(0)?;
    /// if let Some(text) = page.find_text_containing("Hello").first() {
    ///     if let Some(parent) = page.get_parent(text.id()) {
    ///         println!("Parent element: {:?}", parent);
    ///     }
    /// }
    /// ```
    pub fn get_parent(&self, id: ElementId) -> Option<PdfElement> {
        let path = self.element_map.get(&id)?;

        // If path has no elements, the element is the root itself
        // If path has only one element, the parent is the root (which we represent as None
        // for simplicity, as root is a synthetic container)
        if path.path.len() <= 1 {
            return None;
        }

        // Parent path is the element's path minus the last index
        let mut parent_path = path.clone();
        parent_path.path.pop();

        // Traverse to the parent element
        self.get_element_by_path(&parent_path).map(|elem| {
            let parent_id = self.get_id_for_path(&parent_path);
            self.wrap_element(parent_id, parent_path, elem)
        })
    }

    /// Get the index of an element within its parent's children.
    ///
    /// Returns `None` if the element is not found.
    pub fn get_element_index(&self, id: ElementId) -> Option<usize> {
        let path = self.element_map.get(&id)?;
        path.path.last().copied()
    }

    /// Get siblings of an element (excluding the element itself).
    ///
    /// Returns all elements that share the same parent, excluding the element with the given ID.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let page = doc.page(0)?;
    /// if let Some(text) = page.find_text_containing("Hello").first() {
    ///     for sibling in page.get_siblings(text.id()) {
    ///         println!("Sibling: {:?}", sibling);
    ///     }
    /// }
    /// ```
    pub fn get_siblings(&self, id: ElementId) -> Vec<PdfElement> {
        let Some(path) = self.element_map.get(&id) else {
            return Vec::new();
        };

        // Get the parent's children
        let parent_children = if path.path.len() <= 1 {
            // Parent is root, get root children
            &self.root.children
        } else {
            // Get parent element and its children
            let mut parent_path = path.clone();
            parent_path.path.pop();

            match self.get_element_by_path(&parent_path) {
                Some(ContentElement::Structure(s)) => &s.children,
                _ => return Vec::new(),
            }
        };

        // Build the base path (parent's path)
        let base_path = if path.path.len() <= 1 {
            ElementPath::new()
        } else {
            let mut p = path.clone();
            p.path.pop();
            p
        };

        // Get the current element's index
        let current_idx = path.path.last().copied().unwrap_or(0);

        // Return all siblings except the current element
        parent_children
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != current_idx)
            .map(|(idx, child)| {
                let child_path = base_path.with_child(idx);
                let child_id = self.get_id_for_path(&child_path);
                self.wrap_element(child_id, child_path, child)
            })
            .collect()
    }

    /// Get the next sibling of an element.
    ///
    /// Returns `None` if the element is the last child or not found.
    pub fn get_next_sibling(&self, id: ElementId) -> Option<PdfElement> {
        let path = self.element_map.get(&id)?;

        // Get the parent's children
        let parent_children = if path.path.len() <= 1 {
            &self.root.children
        } else {
            let mut parent_path = path.clone();
            parent_path.path.pop();
            match self.get_element_by_path(&parent_path) {
                Some(ContentElement::Structure(s)) => &s.children,
                _ => return None,
            }
        };

        let current_idx = path.path.last().copied()?;
        let next_idx = current_idx + 1;

        if next_idx >= parent_children.len() {
            return None;
        }

        // Build the sibling path
        let base_path = if path.path.len() <= 1 {
            ElementPath::new()
        } else {
            let mut p = path.clone();
            p.path.pop();
            p
        };

        let sibling_path = base_path.with_child(next_idx);
        let sibling_id = self.get_id_for_path(&sibling_path);
        Some(self.wrap_element(sibling_id, sibling_path, &parent_children[next_idx]))
    }

    /// Get the previous sibling of an element.
    ///
    /// Returns `None` if the element is the first child or not found.
    pub fn get_prev_sibling(&self, id: ElementId) -> Option<PdfElement> {
        let path = self.element_map.get(&id)?;

        // Get the parent's children
        let parent_children = if path.path.len() <= 1 {
            &self.root.children
        } else {
            let mut parent_path = path.clone();
            parent_path.path.pop();
            match self.get_element_by_path(&parent_path) {
                Some(ContentElement::Structure(s)) => &s.children,
                _ => return None,
            }
        };

        let current_idx = path.path.last().copied()?;

        if current_idx == 0 {
            return None;
        }

        let prev_idx = current_idx - 1;

        // Build the sibling path
        let base_path = if path.path.len() <= 1 {
            ElementPath::new()
        } else {
            let mut p = path.clone();
            p.path.pop();
            p
        };

        let sibling_path = base_path.with_child(prev_idx);
        let sibling_id = self.get_id_for_path(&sibling_path);
        Some(self.wrap_element(sibling_id, sibling_path, &parent_children[prev_idx]))
    }

    /// Get children of a structure element by ID.
    pub fn get_children(&self, id: ElementId) -> Vec<PdfElement> {
        if let Some(PdfElement::Structure(structure)) = self.get_element(id) {
            return structure
                .content
                .children
                .iter()
                .enumerate()
                .map(|(idx, child)| {
                    let path = ElementPath::new().with_child(idx);
                    let child_id = self.get_id_for_path(&path);
                    self.wrap_element(child_id, path, child)
                })
                .collect();
        }
        Vec::new()
    }

    /// Set an element at a specific path in the tree (internal use for fluent API).
    fn set_element_at_path(
        &mut self,
        path: &ElementPath,
        element: ContentElement,
    ) -> crate::error::Result<()> {
        let mut current = &mut self.root.children;

        for (i, &idx) in path.path.iter().enumerate() {
            if idx >= current.len() {
                return Ok(());
            }

            if i == path.path.len() - 1 {
                // Last index - replace the element
                current[idx] = element;
                return Ok(());
            }

            // Navigate deeper
            if let ContentElement::Structure(ref mut structure) = current[idx] {
                current = &mut structure.children;
            } else {
                return Ok(());
            }
        }

        Ok(())
    }

    /// Get the correct ElementId for a path by looking it up in the element_map.
    /// Creates a reverse lookup from path to ID.
    fn get_id_for_path(&self, path: &ElementPath) -> ElementId {
        self.element_map
            .iter()
            .find_map(|(id, stored_path)| {
                if stored_path.path == path.path {
                    Some(*id)
                } else {
                    None
                }
            })
            .unwrap_or_else(ElementId::new)
    }

    // === Add/Remove Element Methods ===

    /// Add a text element to the page.
    ///
    /// The element is added as a direct child of the root structure.
    /// Returns the ElementId of the newly added element.
    pub fn add_text(&mut self, content: TextContent) -> ElementId {
        let id = ElementId::new();
        let idx = self.root.children.len();
        let path = ElementPath::new().with_child(idx);

        self.root.children.push(ContentElement::Text(content));
        self.element_map.insert(id, path);
        self.dirty_elements.insert(id);
        id
    }

    /// Add an image element to the page.
    ///
    /// The element is added as a direct child of the root structure.
    /// Returns the ElementId of the newly added element.
    pub fn add_image(&mut self, content: ImageContent) -> ElementId {
        let id = ElementId::new();
        let idx = self.root.children.len();
        let path = ElementPath::new().with_child(idx);

        self.root.children.push(ContentElement::Image(content));
        self.element_map.insert(id, path);
        self.dirty_elements.insert(id);
        id
    }

    /// Add a path/graphics element to the page.
    ///
    /// The element is added as a direct child of the root structure.
    /// Returns the ElementId of the newly added element.
    pub fn add_path(&mut self, content: PathContent) -> ElementId {
        let id = ElementId::new();
        let idx = self.root.children.len();
        let path = ElementPath::new().with_child(idx);

        self.root.children.push(ContentElement::Path(content));
        self.element_map.insert(id, path);
        self.dirty_elements.insert(id);
        id
    }

    /// Add a table element to the page.
    ///
    /// The element is added as a direct child of the root structure.
    /// Returns the ElementId of the newly added element.
    pub fn add_table(&mut self, content: TableContent) -> ElementId {
        let id = ElementId::new();
        let idx = self.root.children.len();
        let path = ElementPath::new().with_child(idx);

        self.root.children.push(ContentElement::Table(content));
        self.element_map.insert(id, path);
        self.dirty_elements.insert(id);
        id
    }

    /// Remove an element from the page by ID.
    ///
    /// Returns true if the element was found and removed, false otherwise.
    /// Note: This only removes top-level elements. Nested elements within
    /// structures cannot be removed this way.
    pub fn remove_element(&mut self, id: ElementId) -> bool {
        if let Some(path) = self.element_map.remove(&id) {
            // Only handle top-level elements (path length = 1)
            if path.path.len() == 1 {
                let idx = path.path[0];
                if idx < self.root.children.len() {
                    self.root.children.remove(idx);
                    self.dirty_elements.remove(&id);
                    // Rebuild element map since indices have shifted
                    self.rebuild_element_map();
                    return true;
                }
            }
        }
        false
    }

    // === Find Path/Table Methods ===

    /// Find all paths on the page.
    pub fn find_paths(&self) -> Vec<PdfPath> {
        let mut results = Vec::new();
        self.collect_paths_recursive(&self.root.children, ElementPath::new(), &mut results);
        results
    }

    /// Recursively collect path elements.
    fn collect_paths_recursive(
        &self,
        children: &[ContentElement],
        path: ElementPath,
        results: &mut Vec<PdfPath>,
    ) {
        for (idx, child) in children.iter().enumerate() {
            let child_path = path.with_child(idx);
            match child {
                ContentElement::Path(p) => {
                    let id = self.get_id_for_path(&child_path);
                    results.push(PdfPath {
                        id,
                        content: p.clone(),
                        path: child_path,
                    });
                },
                ContentElement::Structure(s) => {
                    self.collect_paths_recursive(&s.children, child_path, results);
                },
                _ => {},
            }
        }
    }

    /// Find all tables on the page.
    pub fn find_tables(&self) -> Vec<PdfTable> {
        let mut results = Vec::new();
        self.collect_tables_recursive(&self.root.children, ElementPath::new(), &mut results);
        results
    }

    /// Recursively collect table elements.
    fn collect_tables_recursive(
        &self,
        children: &[ContentElement],
        path: ElementPath,
        results: &mut Vec<PdfTable>,
    ) {
        for (idx, child) in children.iter().enumerate() {
            let child_path = path.with_child(idx);
            match child {
                ContentElement::Table(t) => {
                    let id = self.get_id_for_path(&child_path);
                    results.push(PdfTable {
                        id,
                        content: t.clone(),
                        path: child_path,
                    });
                },
                ContentElement::Structure(s) => {
                    self.collect_tables_recursive(&s.children, child_path, results);
                },
                _ => {},
            }
        }
    }

    // === Annotation Methods ===

    /// Get all annotations on this page.
    pub fn annotations(&self) -> &[AnnotationWrapper] {
        &self.annotations
    }

    /// Get annotation by index.
    pub fn annotation(&self, index: usize) -> Option<&AnnotationWrapper> {
        self.annotations.get(index)
    }

    /// Get mutable reference to all annotations.
    pub fn annotations_mut(&mut self) -> &mut [AnnotationWrapper] {
        self.annotations_modified = true;
        &mut self.annotations
    }

    /// Get mutable reference to annotation by index.
    pub fn annotation_mut(&mut self, index: usize) -> Option<&mut AnnotationWrapper> {
        self.annotations_modified = true;
        self.annotations.get_mut(index)
    }

    /// Add a new annotation to the page.
    ///
    /// Returns the AnnotationId of the newly added annotation.
    pub fn add_annotation<A: Into<WriteAnnotation>>(&mut self, annotation: A) -> AnnotationId {
        let wrapper = AnnotationWrapper::from_write(annotation);
        let id = wrapper.id();
        self.annotations.push(wrapper);
        self.annotations_modified = true;
        id
    }

    /// Remove an annotation by index.
    ///
    /// Returns the removed annotation wrapper if the index was valid.
    pub fn remove_annotation(&mut self, index: usize) -> Option<AnnotationWrapper> {
        if index < self.annotations.len() {
            self.annotations_modified = true;
            Some(self.annotations.remove(index))
        } else {
            None
        }
    }

    /// Remove an annotation by its ID.
    ///
    /// Returns the removed annotation wrapper if found.
    pub fn remove_annotation_by_id(&mut self, id: AnnotationId) -> Option<AnnotationWrapper> {
        if let Some(pos) = self.annotations.iter().position(|a| a.id() == id) {
            self.annotations_modified = true;
            Some(self.annotations.remove(pos))
        } else {
            None
        }
    }

    /// Find annotation by its ID.
    pub fn find_annotation(&self, id: AnnotationId) -> Option<&AnnotationWrapper> {
        self.annotations.iter().find(|a| a.id() == id)
    }

    /// Find mutable annotation by its ID.
    pub fn find_annotation_mut(&mut self, id: AnnotationId) -> Option<&mut AnnotationWrapper> {
        self.annotations_modified = true;
        self.annotations.iter_mut().find(|a| a.id() == id)
    }

    /// Check if annotations have been modified.
    pub fn has_annotations_modified(&self) -> bool {
        self.annotations_modified
    }

    /// Get the number of annotations on this page.
    pub fn annotation_count(&self) -> usize {
        self.annotations.len()
    }

    /// Find annotations in a specific region.
    pub fn find_annotations_in_region(&self, region: Rect) -> Vec<&AnnotationWrapper> {
        self.annotations
            .iter()
            .filter(|annot| {
                let rect = annot.rect();
                // Check if annotation intersects with region
                rect.x < region.x + region.width
                    && rect.x + rect.width > region.x
                    && rect.y < region.y + region.height
                    && rect.y + rect.height > region.y
            })
            .collect()
    }

    /// Find annotations by subtype.
    pub fn find_annotations_by_type(&self, subtype: AnnotationSubtype) -> Vec<&AnnotationWrapper> {
        self.annotations
            .iter()
            .filter(|annot| annot.subtype() == subtype)
            .collect()
    }
}

/// Fluent page editor for chainable operations (XMLDocument-style API).
///
/// Enables chaining operations like:
/// ```ignore
/// doc.edit_page(0)?
///    .find_text("Hello")?
///    .for_each(|mut text| text.set_text("Hi"))?
///    .done()?;
/// ```
pub struct PageEditor {
    pub page: PdfPage,
}

impl PageEditor {
    /// Find text elements containing a needle string.
    pub fn find_text_containing(
        self,
        needle: &str,
    ) -> crate::error::Result<TextElementCollectionEditor> {
        let elements = self.page.find_text_containing(needle);
        let element_ids = elements.iter().map(|e| e.id()).collect();
        Ok(TextElementCollectionEditor {
            page: self.page,
            element_ids,
        })
    }

    /// Find text elements matching a predicate.
    pub fn find_text<F>(self, predicate: F) -> crate::error::Result<TextElementCollectionEditor>
    where
        F: Fn(&PdfText) -> bool,
    {
        let elements = self.page.find_text(predicate);
        let element_ids = elements.iter().map(|e| e.id()).collect();
        Ok(TextElementCollectionEditor {
            page: self.page,
            element_ids,
        })
    }

    /// Find all images on the page.
    pub fn find_images(self) -> crate::error::Result<ImageElementCollectionEditor> {
        let elements = self.page.find_images();
        let element_ids = elements.iter().map(|e| e.id()).collect();
        Ok(ImageElementCollectionEditor {
            page: self.page,
            element_ids,
        })
    }

    /// Find all path/graphics elements on the page.
    pub fn find_paths(self) -> crate::error::Result<PathElementCollectionEditor> {
        let elements = self.page.find_paths();
        let element_ids = elements.iter().map(|e| e.id()).collect();
        Ok(PathElementCollectionEditor {
            page: self.page,
            element_ids,
        })
    }

    /// Find all table elements on the page.
    pub fn find_tables(self) -> crate::error::Result<TableElementCollectionEditor> {
        let elements = self.page.find_tables();
        let element_ids = elements.iter().map(|e| e.id()).collect();
        Ok(TableElementCollectionEditor {
            page: self.page,
            element_ids,
        })
    }

    /// Finish editing and return the modified page.
    pub fn done(self) -> crate::error::Result<PdfPage> {
        Ok(self.page)
    }
}

/// Fluent text element collection editor.
pub struct TextElementCollectionEditor {
    pub page: PdfPage,
    pub element_ids: Vec<ElementId>,
}

impl TextElementCollectionEditor {
    /// Apply a function to each text element.
    pub fn for_each<F>(mut self, mut f: F) -> crate::error::Result<Self>
    where
        F: FnMut(&mut PdfText) -> crate::error::Result<()>,
    {
        for &id in self.element_ids.iter() {
            // Get the current element from the page
            if let Some(PdfElement::Text(mut text)) = self.page.get_element(id) {
                // Call the user's closure on the mutable element
                f(&mut text)?;

                // Sync the modifications back to the page using the stored path
                self.page
                    .set_element_at_path(&text.path, ContentElement::Text(text.content))?;
            }
        }
        Ok(self)
    }

    /// Finish editing and return the modified page.
    pub fn done(self) -> crate::error::Result<PdfPage> {
        Ok(self.page)
    }
}

/// Fluent image element collection editor.
pub struct ImageElementCollectionEditor {
    pub page: PdfPage,
    pub element_ids: Vec<ElementId>,
}

impl ImageElementCollectionEditor {
    /// Apply a function to each image element.
    pub fn for_each<F>(mut self, mut f: F) -> crate::error::Result<Self>
    where
        F: FnMut(&mut PdfImage) -> crate::error::Result<()>,
    {
        for &id in self.element_ids.iter() {
            // Get the current element from the page
            if let Some(PdfElement::Image(mut image)) = self.page.get_element(id) {
                // Call the user's closure on the mutable element
                f(&mut image)?;

                // Sync the modifications back to the page using the stored path
                self.page
                    .set_element_at_path(&image.path, ContentElement::Image(image.content))?;
            }
        }
        Ok(self)
    }

    /// Finish editing and return the modified page.
    pub fn done(self) -> crate::error::Result<PdfPage> {
        Ok(self.page)
    }
}

/// Fluent path/graphics element collection editor.
pub struct PathElementCollectionEditor {
    /// The page being edited.
    pub page: PdfPage,
    /// IDs of the path elements in this collection.
    pub element_ids: Vec<ElementId>,
}

impl PathElementCollectionEditor {
    /// Apply a function to each path element.
    pub fn for_each<F>(mut self, mut f: F) -> crate::error::Result<Self>
    where
        F: FnMut(&mut PdfPath) -> crate::error::Result<()>,
    {
        for &id in self.element_ids.iter() {
            // Get the current element from the page
            if let Some(PdfElement::Path(mut path)) = self.page.get_element(id) {
                // Call the user's closure on the mutable element
                f(&mut path)?;

                // Sync the modifications back to the page using the stored path
                self.page
                    .set_element_at_path(&path.path, ContentElement::Path(path.content))?;
            }
        }
        Ok(self)
    }

    /// Finish editing and return the modified page.
    pub fn done(self) -> crate::error::Result<PdfPage> {
        Ok(self.page)
    }
}

/// Fluent table element collection editor.
pub struct TableElementCollectionEditor {
    /// The page being edited.
    pub page: PdfPage,
    /// IDs of the table elements in this collection.
    pub element_ids: Vec<ElementId>,
}

impl TableElementCollectionEditor {
    /// Apply a function to each table element.
    pub fn for_each<F>(mut self, mut f: F) -> crate::error::Result<Self>
    where
        F: FnMut(&mut PdfTable) -> crate::error::Result<()>,
    {
        for &id in self.element_ids.iter() {
            // Get the current element from the page
            if let Some(PdfElement::Table(mut table)) = self.page.get_element(id) {
                // Call the user's closure on the mutable element
                f(&mut table)?;

                // Sync the modifications back to the page using the stored path
                self.page
                    .set_element_at_path(&table.path, ContentElement::Table(table.content))?;
            }
        }
        Ok(self)
    }

    /// Finish editing and return the modified page.
    pub fn done(self) -> crate::error::Result<PdfPage> {
        Ok(self.page)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::{FontSpec, TextStyle};

    fn create_test_text(text: &str, x: f32, y: f32) -> TextContent {
        TextContent {
            text: text.to_string(),
            bbox: Rect::new(x, y, 100.0, 12.0),
            font: FontSpec {
                name: "Helvetica".to_string(),
                size: 12.0,
            },
            style: TextStyle::default(),
            reading_order: None,
            origin: None,
            rotation_degrees: None,
            matrix: None,
        }
    }

    fn create_test_page_with_texts() -> PdfPage {
        let text1 = create_test_text("Hello", 10.0, 700.0);
        let text2 = create_test_text("World", 10.0, 680.0);
        let text3 = create_test_text("Foo", 10.0, 660.0);

        let root = StructureElement {
            structure_type: "Document".to_string(),
            bbox: Rect::new(0.0, 0.0, 612.0, 792.0),
            children: vec![
                ContentElement::Text(text1),
                ContentElement::Text(text2),
                ContentElement::Text(text3),
            ],
            reading_order: Some(0),
            alt_text: None,
            language: None,
        };

        PdfPage::from_structure(0, root, 612.0, 792.0)
    }

    #[test]
    fn test_element_id_generation() {
        let id1 = ElementId::new();
        let id2 = ElementId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_pdf_page_creation() {
        let root = StructureElement {
            structure_type: "Document".to_string(),
            bbox: Rect::new(0.0, 0.0, 612.0, 792.0),
            children: Vec::new(),
            reading_order: Some(0),
            alt_text: None,
            language: None,
        };

        let page = PdfPage::from_structure(0, root, 612.0, 792.0);
        assert_eq!(page.page_index, 0);
        assert_eq!(page.width, 612.0);
        assert_eq!(page.height, 792.0);
    }

    #[test]
    fn test_children_returns_all_top_level_elements() {
        let page = create_test_page_with_texts();
        let children = page.children();
        assert_eq!(children.len(), 3);
    }

    #[test]
    fn test_find_text_containing() {
        let page = create_test_page_with_texts();
        let results = page.find_text_containing("Hello");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].text(), "Hello");
    }

    #[test]
    fn test_get_element_index() {
        let page = create_test_page_with_texts();
        let children = page.children();

        // First element should be at index 0
        let first_id = children[0].id();
        assert_eq!(page.get_element_index(first_id), Some(0));

        // Second element should be at index 1
        let second_id = children[1].id();
        assert_eq!(page.get_element_index(second_id), Some(1));
    }

    #[test]
    fn test_get_siblings() {
        let page = create_test_page_with_texts();
        let children = page.children();

        // Get siblings of the second element (should be first and third)
        let second_id = children[1].id();
        let siblings = page.get_siblings(second_id);
        assert_eq!(siblings.len(), 2);
    }

    #[test]
    fn test_get_next_sibling() {
        let page = create_test_page_with_texts();
        let children = page.children();

        // First element should have a next sibling
        let first_id = children[0].id();
        let next = page.get_next_sibling(first_id);
        assert!(next.is_some());

        // Last element should not have a next sibling
        let last_id = children[2].id();
        let next = page.get_next_sibling(last_id);
        assert!(next.is_none());
    }

    #[test]
    fn test_get_prev_sibling() {
        let page = create_test_page_with_texts();
        let children = page.children();

        // First element should not have a previous sibling
        let first_id = children[0].id();
        let prev = page.get_prev_sibling(first_id);
        assert!(prev.is_none());

        // Last element should have a previous sibling
        let last_id = children[2].id();
        let prev = page.get_prev_sibling(last_id);
        assert!(prev.is_some());
    }

    #[test]
    fn test_get_parent_at_root_level() {
        let page = create_test_page_with_texts();
        let children = page.children();

        // Top-level elements should return None for parent
        let first_id = children[0].id();
        assert!(page.get_parent(first_id).is_none());
    }

    #[test]
    fn test_pdf_text_value_alias() {
        let mut text = PdfText {
            id: ElementId::new(),
            content: create_test_text("Hello", 0.0, 0.0),
            path: ElementPath::new(),
        };

        // Test value() alias
        assert_eq!(text.value(), "Hello");
        assert_eq!(text.text(), text.value());

        // Test set_value() alias
        text.set_value("Goodbye");
        assert_eq!(text.value(), "Goodbye");
    }

    #[test]
    fn test_pdf_text_append() {
        let mut text = PdfText {
            id: ElementId::new(),
            content: create_test_text("Hello", 0.0, 0.0),
            path: ElementPath::new(),
        };

        text.append(" World");
        assert_eq!(text.text(), "Hello World");
    }

    #[test]
    fn test_pdf_text_replace() {
        let mut text = PdfText {
            id: ElementId::new(),
            content: create_test_text("Hello Hello Hello", 0.0, 0.0),
            path: ElementPath::new(),
        };

        let count = text.replace("Hello", "Hi");
        assert_eq!(count, 3);
        assert_eq!(text.text(), "Hi Hi Hi");
    }

    #[test]
    fn test_pdf_text_clear() {
        let mut text = PdfText {
            id: ElementId::new(),
            content: create_test_text("Hello", 0.0, 0.0),
            path: ElementPath::new(),
        };

        text.clear();
        assert!(text.is_empty());
        assert_eq!(text.len(), 0);
    }

    #[test]
    fn test_pdf_text_contains() {
        let text = PdfText {
            id: ElementId::new(),
            content: create_test_text("Hello World", 0.0, 0.0),
            path: ElementPath::new(),
        };

        assert!(text.contains("World"));
        assert!(!text.contains("Foo"));
    }

    #[test]
    fn test_pdf_text_starts_with_ends_with() {
        let text = PdfText {
            id: ElementId::new(),
            content: create_test_text("Hello World", 0.0, 0.0),
            path: ElementPath::new(),
        };

        assert!(text.starts_with("Hello"));
        assert!(text.ends_with("World"));
        assert!(!text.starts_with("World"));
        assert!(!text.ends_with("Hello"));
    }
}
