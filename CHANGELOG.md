# Changelog

All notable changes to PDFOxide are documented here.

## [0.3.0] - 2026-01-10

### Added - PDF Creation
- **PDF Creation API** - Fluent `DocumentBuilder` for programmatic PDF generation
  - `Pdf::create()` / `DocumentBuilder::new()` entry points
  - Page sizing (Letter, A4, custom dimensions)
  - Text rendering with Base14 fonts and styling
  - Image embedding (JPEG/PNG) with positioning
- **Table Rendering** - `TableRenderer` for styled tables
  - Headers, borders, cell spans, alternating row colors
  - Column width control (fixed, percentage, auto)
  - Cell alignment and padding
- **Graphics API** - Advanced visual effects
  - Colors (RGB, CMYK, grayscale)
  - Linear and radial gradients
  - Tiling patterns with presets
  - Blend modes and transparency (ExtGState)
- **Page Templates** - Reusable page elements
  - Headers and footers with placeholders
  - Page numbering formats
  - Watermarks (text-based)
- **Barcode Generation** (requires `barcodes` feature)
  - QR codes with configurable size and error correction
  - Code128, EAN-13, UPC-A, Code39, ITF barcodes
  - Customizable colors and dimensions

### Added - PDF Editing
- **Editor API** - DOM-like editing with round-trip preservation
  - `DocumentEditor` for modifying existing PDFs
  - Content addition without breaking existing structure
  - Resource management for fonts and images
- **Annotation Support** - Full read/write for all types
  - Text markup: highlights, underlines, strikeouts, squiggly
  - Notes: sticky notes, comments, popups
  - Shapes: rectangles, circles, lines, polygons, polylines
  - Drawing: ink/freehand annotations
  - Stamps: standard and custom stamps
  - Special: file attachments, redactions, carets
- **Form Fields** - Interactive form creation
  - Text fields (single/multiline, password, comb)
  - Checkboxes with custom appearance
  - Radio button groups
  - Dropdown and list boxes
  - Push buttons with actions
  - Form flattening (convert fields to static content)
- **Link Annotations** - Navigation support
  - External URLs
  - Internal page navigation
  - Styled link appearance
- **Outline Builder** - Bookmark/TOC creation
  - Hierarchical structure
  - Page destinations
  - Styling (bold, italic, colors)
- **PDF Layers** - Optional Content Groups (OCG)
  - Create and manage content layers
  - Layer visibility controls

### Added - PDF Compliance & Validation
- **PDF/A Validation** - ISO 19005 compliance checking
  - PDF/A-1a, PDF/A-1b (ISO 19005-1)
  - PDF/A-2a, PDF/A-2b, PDF/A-2u (ISO 19005-2)
  - PDF/A-3a, PDF/A-3b (ISO 19005-3)
- **PDF/A Conversion** - Convert documents to archival format
  - Automatic font embedding
  - XMP metadata injection
  - ICC color profile conversion
- **PDF/X Validation** - ISO 15930 print production compliance
  - PDF/X-1a:2001, PDF/X-1a:2003
  - PDF/X-3:2002, PDF/X-3:2003
  - PDF/X-4, PDF/X-4p
  - PDF/X-5g, PDF/X-5n, PDF/X-5pg
  - PDF/X-6, PDF/X-6n, PDF/X-6p
  - 40+ specific error codes for violations
- **PDF/UA Validation** - ISO 14289 accessibility compliance
  - Tagged PDF structure validation
  - Language specification checks
  - Alt text requirements
  - Heading hierarchy validation
  - Table header validation
  - Form field accessibility
  - Reading order verification

### Added - Security & Encryption
- **Encryption on Write** - Password-protect PDFs when saving
  - AES-256 (V=5, R=6) - Modern 256-bit encryption (default)
  - AES-128 (V=4, R=4) - Modern 128-bit encryption
  - RC4-128 (V=2, R=3) - Legacy 128-bit encryption
  - RC4-40 (V=1, R=2) - Legacy 40-bit encryption
  - `Pdf::save_encrypted()` for simple password protection
  - `Pdf::save_with_encryption()` for full configuration
- **Permission Controls** - Granular access restrictions
  - Print, copy, modify, annotate permissions
  - Form fill and accessibility extraction controls
- **Digital Signatures** (foundation, requires `signatures` feature)
  - ByteRange calculation for signature placeholders
  - PKCS#7/CMS signature structure support
  - X.509 certificate parsing
  - Signature verification framework

### Added - Document Features
- **Page Labels** - Custom page numbering
  - Roman numerals, letters, decimal formats
  - Prefix support (e.g., "A-1", "B-2")
  - `PageLabelsBuilder` for creation
  - Extract existing labels from documents
- **XMP Metadata** - Extensible metadata support
  - Dublin Core properties (title, creator, description)
  - PDF properties (producer, keywords)
  - Custom namespace support
  - Full read/write capability
- **Embedded Files** - File attachments
  - Attach files to PDF documents
  - MIME type and description support
  - Relationship specification (Source, Data, etc.)
- **Linearization** - Web-optimized PDFs
  - Fast web view support
  - Streaming delivery optimization

### Added - Search & Analysis
- **Text Search** - Pattern-based document search
  - Regex pattern support
  - Case-sensitive/insensitive options
  - Position tracking with page/coordinates
  - Whole word matching
- **Page Rendering** (requires `rendering` feature)
  - Render pages to PNG/JPEG images
  - Configurable DPI and scale
  - Pure Rust via tiny-skia (no external dependencies)
- **Debug Visualization** (requires `rendering` feature)
  - Visualize text bounding boxes
  - Element highlighting for debugging
  - Export annotated page images

### Added - Document Conversion
- **Office to PDF** (requires `office` feature)
  - **DOCX**: Word documents with paragraphs, headings, lists, formatting
  - **XLSX**: Excel spreadsheets via calamine (sheets, cells, tables)
  - **PPTX**: PowerPoint presentations (slides, titles, text boxes)
  - `OfficeConverter` with auto-detection
  - `OfficeConfig` for page size, margins, fonts
  - Python bindings: `OfficeConverter.from_docx()`, `from_xlsx()`, `from_pptx()`

### Added - Python Bindings
- `Pdf` class for PDF creation
- `Color`, `BlendMode`, `ExtGState` for graphics
- `LinearGradient`, `RadialGradient` for gradients
- `LineCap`, `LineJoin`, `PatternPresets` for styling
- `save_encrypted()` method with permission flags
- `OfficeConverter` class for Office document conversion

### Changed
- Description updated to "The Complete PDF Toolkit: extract, create, and edit PDFs"
- Python module docstring updated for v0.3.0 features
- Branding updated with Extract/Create/Edit pillars

## [0.2.6] - 2026-01-09

### Added
- **TagSuspect/MarkInfo support** (ISO 32000-1 Section 14.7.1)
  - Parse MarkInfo dictionary from document catalog (`marked`, `suspects`, `user_properties`)
  - `PdfDocument::mark_info()` method to retrieve MarkInfo
  - Automatic fallback to geometric ordering when structure tree is marked as suspect
- **Word Break /WB structure element** (Section 14.8.4.4)
  - Support for explicit word boundaries in CJK text
  - `StructType::WB` variant and `is_word_break()` helper
  - Word break markers emitted during structure tree traversal
- **Predefined CMap support for CJK fonts** (Section 9.7.5.2)
  - Adobe-GB1 (Simplified Chinese) - ~500 common character mappings
  - Adobe-Japan1 (Japanese) - Hiragana, Katakana, Kanji mappings
  - Adobe-CNS1 (Traditional Chinese) - Bopomofo and CJK mappings
  - Adobe-Korea1 (Korean) - Hangul and Hanja mappings
  - Fallback identity mapping for common Unicode ranges
- **Abbreviation expansion /E support** (Section 14.9.5)
  - Parse `/E` entry from marked content properties
  - `expansion` field on `StructElem` for structure-level abbreviations
- **Object reference resolution utility**
  - `PdfDocument::resolve_references()` for recursive reference handling in complex PDF structures
- **Type 0 /W array parsing** for CIDFont glyph widths
  - Proper spacing for CJK text using CIDFont width specifications
- **ActualText verification tests** - comprehensive test coverage for PDF Spec Section 14.9.4

### Fixed
- **Soft hyphen handling** (U+00AD) - now correctly treated as valid continuation hyphen for word reconstruction

### Changed
- **Enhanced artifact filtering** with subtype support
  - `ArtifactType::Pagination` with subtypes: Header, Footer, Watermark, PageNumber
  - `ArtifactType::Layout` and `ArtifactType::Background` classification
- `OrderedContent.mcid` changed to `Option<u32>` to support word break markers

## [0.2.5] - 2026-01-09

### Added
- **Image embedding**: Both HTML and Markdown now support embedded base64 images when `embed_images=true` (default)
  - HTML: `<img src="data:image/png;base64,...">`
  - Markdown: `![alt](data:image/png;base64,...)` (works in Obsidian, Typora, VS Code, Jupyter)
- **Image file export**: Set `embed_images=false` + `image_output_dir` to save images as files with relative path references
- New `embed_images` option in `ConversionOptions` to control embedding behavior
- `PdfImage::to_base64_data_uri()` method for converting images to data URIs
- `PdfImage::to_png_bytes()` method for in-memory PNG encoding
- Python bindings: new `embed_images` parameter for `to_html`, `to_markdown`, and `*_all` methods

## [0.2.4] - 2026-01-09

### Fixed
- CTM (Current Transformation Matrix) now correctly applied to text positions per PDF Spec ISO 32000-1:2008 Section 9.4.4 (#11)

### Added
- Structure tree: `/Alt` (alternate description) parsing for accessibility text on formulas and figures
- Structure tree: `/Pg` (page reference) resolution - correctly maps structure elements to page numbers
- `FormulaRenderer` module for extracting formula regions as base64 images from rendered pages
- `ConversionOptions`: new fields `render_formulas`, `page_images`, `page_dimensions` for formula image embedding
- Regression tests for CTM transformation

## [0.2.3] - 2026-01-07

### Fixed
- BT/ET matrix reset per PDF spec Section 9.4.1 (PR #10 by @drahnr)
- Geometric spacing detection in markdown converter (#5)
- Verbose extractor logs changed from info to trace (#7)
- docs.rs build failure (excluded tesseract-rs)

### Added
- `apply_intelligent_text_processing()` method for ligature expansion, hyphenation reconstruction, and OCR cleanup (#6)

### Changed
- Removed unused tesseract-rs dependency

## [0.2.2] - 2025-12-15

### Changed
- Optimized crate keywords for better discoverability

## [0.2.1] - 2025-12-15

### Fixed
- Encrypted stream decoding improvements (#3)
- CI/CD pipeline fixes

## [0.1.4] - 2025-12-12

### Fixed
- Encrypted stream decoding (#2)
- Documentation and doctest fixes

## [0.1.3] - 2025-12-12

### Fixed
- Encrypted stream decoding refinements

## [0.1.2] - 2025-11-27

### Added
- Python 3.13 support
- GitHub sponsor configuration

## [0.1.1] - 2025-11-26

### Added
- Cross-platform binary builds (Linux, macOS, Windows)

## [0.1.0] - 2025-11-06

### Added
- Initial release
- PDF text extraction with spec-compliant Unicode mapping
- Intelligent reading order detection
- Python bindings via PyO3
- Support for encrypted PDFs
- Form field extraction
- Image extraction
