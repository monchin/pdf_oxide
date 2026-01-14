# pdfium-render vs pdf_oxide Gap Analysis

This document analyzes feature gaps between [pdfium-render](https://github.com/ajrcarey/pdfium-render) and pdf_oxide to guide future development priorities.

## Overview

| Aspect | pdfium-render | pdf_oxide |
|--------|---------------|-----------|
| Backend | PDFium (Google's C++ library via FFI) | Pure Rust implementation |
| License | Apache 2.0 / MIT | MIT |
| WASM Support | Yes | Planned |
| Dependencies | Requires PDFium binary | Pure Rust, no external deps |

## Text Extraction

### PdfPageTextChar (Character-level)

| Feature | pdfium-render | pdf_oxide | Gap | Priority |
|---------|---------------|-----------|-----|----------|
| Character value | `PdfPageTextChar::unicode_char()` | `TextChar::char` | ✅ None | - |
| Bounding box (tight) | `tight_bounds()` | `TextChar::bbox` | ✅ None | - |
| Bounding box (loose) | `loose_bounds()` | ❌ Missing | **Gap** | Medium |
| Origin X (baseline) | `origin_x()` | ❌ Missing | **Gap** | High |
| Origin Y (baseline) | `origin_y()` | ❌ Missing | **Gap** | High |
| Rotation angle | `angle_degrees()`, `angle_radians()` | ❌ Missing | **Gap** | High |
| Has descender | `has_descender()` | ❌ Missing | **Gap** | Low |
| Transformation matrix | `matrix()` | ❌ Missing | **Gap** | Medium |
| Font size (scaled) | `scaled_font_size()` | `TextChar::font_size` | ✅ Partial | - |
| Font size (unscaled) | `unscaled_font_size()` | ❌ Missing | **Gap** | Low |
| Fill color | `fill_color()` | `TextChar::color` | ✅ Partial | - |
| Stroke color | `stroke_color()` | ❌ Missing | **Gap** | Low |
| Font name | `font_name()` | `TextChar::font_name` | ✅ None | - |
| Font weight | `font_weight()` | `TextChar::font_weight` | ✅ None | - |
| Is italic | Via font flags | `TextChar::is_italic` | ✅ None | - |

**Issue #27** specifically requests: origin X/Y and rotation angle.

### PdfPageText (Page-level text)

| Feature | pdfium-render | pdf_oxide | Gap |
|---------|---------------|-----------|-----|
| Get all text | `all()` | `page.extract_text()` | ✅ None |
| Character count | `len()` | Via iterator | ✅ None |
| Get char by index | `get()` | Via iterator | ✅ None |
| Character iterator | `chars()` | `page.text_chars()` | ✅ None |
| Segment iterator | `segments()` | `page.text_lines()` | ✅ Similar |
| Bounded text extraction | `bounded()` | ❌ Missing | **Gap** |
| Text from rect | `inside_rect()` | ❌ Missing | **Gap** |

## Image Extraction

### PdfPageImageObject

| Feature | pdfium-render | pdf_oxide | Gap |
|---------|---------------|-----------|-----|
| Width/Height (pixels) | `width()`, `height()` | `image.width`, `image.height` | ✅ None |
| Horizontal DPI | `horizontal_dpi()` | ❌ Missing | **Gap** |
| Vertical DPI | `vertical_dpi()` | ❌ Missing | **Gap** |
| Color space | `color_space()` | ❌ Missing | **Gap** |
| Bits per pixel | `bits_per_pixel()` | ❌ Missing | **Gap** |
| Filters applied | `filters()` | ❌ Missing | **Gap** |
| Raw image data | `get_raw_image()` | ✅ Available | ✅ None |
| Decoded bitmap | `get_processed_image()` | ❌ Missing | **Gap** |
| Export to DynamicImage | `as_image()` | ❌ Missing | **Gap** |
| Markers (EXIF, etc.) | `get_raw_metadata()` | ❌ Missing | **Gap** |

## Annotations

### Supported Annotation Types

| Type | pdfium-render | pdf_oxide | Gap |
|------|---------------|-----------|-----|
| Circle | ✅ | ❌ | **Gap** |
| Free Text | ✅ | ❌ | **Gap** |
| Highlight | ✅ | ✅ | ✅ None |
| Ink | ✅ | ❌ | **Gap** |
| Line | ✅ | ❌ | **Gap** |
| Link | ✅ | ✅ | ✅ None |
| Popup | ✅ | ❌ | **Gap** |
| Square | ✅ | ❌ | **Gap** |
| Squiggly | ✅ | ❌ | **Gap** |
| Stamp | ✅ | ❌ | **Gap** |
| Strikeout | ✅ | ❌ | **Gap** |
| Text (Note) | ✅ | ❌ | **Gap** |
| Underline | ✅ | ❌ | **Gap** |
| Widget (Forms) | ✅ | Partial | **Gap** |
| XFA Widget | ✅ | ❌ | **Gap** |

### Annotation Operations

| Operation | pdfium-render | pdf_oxide | Gap |
|-----------|---------------|-----------|-----|
| Read annotation content | ✅ | ✅ | ✅ None |
| Create new annotations | ✅ | Partial (highlight only) | **Gap** |
| Delete annotations | ✅ | ❌ | **Gap** |
| Modify annotations | ✅ | ❌ | **Gap** |
| Get annotation bounds | ✅ | ✅ | ✅ None |
| Annotation appearance | ✅ | ❌ | **Gap** |

## Form Fields (AcroForms)

### Supported Field Types

| Type | pdfium-render | pdf_oxide | Gap |
|------|---------------|-----------|-----|
| Text Field | ✅ Read/Write | ✅ Read/Write | ✅ None |
| Checkbox | ✅ Read/Write | ✅ Read/Write | ✅ None |
| Radio Button | ✅ Read/Write | ✅ Read/Write | ✅ None |
| Combo Box | ✅ Read/Write | ✅ Read/Write | ✅ None |
| List Box | ✅ Read/Write | ✅ Read/Write | ✅ None |
| Push Button | ✅ | ✅ | ✅ None |
| Signature | ✅ | ✅ | ✅ None |
| XFA Forms | ✅ | ❌ | **Gap** |

### Form Operations

| Operation | pdfium-render | pdf_oxide | Gap |
|-----------|---------------|-----------|-----|
| Read field values | ✅ | ✅ | ✅ None |
| Fill field values | ✅ | ✅ | ✅ None |
| Get field options | ✅ | ✅ | ✅ None |
| Flatten forms | ✅ | ❌ | **Gap** |
| Create new fields | ✅ | ❌ | **Gap** |
| Remove fields | ✅ | ❌ | **Gap** |
| Field validation | ✅ | ❌ | **Gap** |

## Page Rendering

| Feature | pdfium-render | pdf_oxide | Gap |
|---------|---------------|-----------|-----|
| Render to image | ✅ | ✅ | ✅ None |
| Custom DPI | ✅ | ✅ | ✅ None |
| PNG output | ✅ | ✅ | ✅ None |
| JPEG output | ✅ | ✅ | ✅ None |
| Color formats (RGB, BGR, RGBA, etc.) | ✅ Multiple | Partial | **Gap** |
| Grayscale rendering | ✅ | ❌ | **Gap** |
| Transparent background | ✅ | ✅ | ✅ None |
| Anti-aliasing control | ✅ | ❌ | **Gap** |
| Render specific region | ✅ | ❌ | **Gap** |
| Render to raw buffer | ✅ | ❌ | **Gap** |
| Tile-based rendering | ✅ | ❌ | **Gap** |

## Page Objects

### Object Types

| Type | pdfium-render | pdf_oxide | Gap |
|------|---------------|-----------|-----|
| Text objects | ✅ Full access | Partial | **Gap** |
| Image objects | ✅ Full access | ✅ | ✅ None |
| Path objects | ✅ Full access | ❌ | **Gap** |
| Shading objects | ✅ | ❌ | **Gap** |
| Form XObjects | ✅ | ❌ | **Gap** |

### Object Operations

| Operation | pdfium-render | pdf_oxide | Gap |
|-----------|---------------|-----------|-----|
| Get object bounds | ✅ | Partial | **Gap** |
| Get object matrix | ✅ | ❌ | **Gap** |
| Create objects | ✅ | ❌ | **Gap** |
| Delete objects | ✅ | ❌ | **Gap** |
| Modify objects | ✅ | ❌ | **Gap** |
| Object z-order | ✅ | ❌ | **Gap** |

## Document Operations

| Feature | pdfium-render | pdf_oxide | Gap |
|---------|---------------|-----------|-----|
| Open from file | ✅ | ✅ | ✅ None |
| Open from bytes | ✅ | ✅ | ✅ None |
| Password protected | ✅ | ✅ | ✅ None |
| Page count | ✅ | ✅ | ✅ None |
| Get metadata | ✅ | ✅ | ✅ None |
| Set metadata | ✅ | ✅ | ✅ None |
| Get bookmarks | ✅ | ✅ | ✅ None |
| Create bookmarks | ✅ | ❌ | **Gap** |
| Page manipulation | ✅ | ✅ | ✅ None |
| Linearization | ✅ | ❌ | **Gap** |

## Priority Roadmap

### High Priority (Issue #27 + Common Use Cases)

1. **TextChar origin (X/Y baseline position)** - Issue #27
2. **TextChar rotation angle** - Issue #27
3. **Bounded text extraction** - Common OCR/layout use case
4. **More annotation types** - Strikeout, underline, text notes

### Medium Priority

5. **TextChar loose bounds** - Better layout analysis
6. **TextChar transformation matrix** - Full positioning info
7. **Image DPI and color space** - Image quality assessment
8. **Form flattening** - Common document finalization
9. **Path objects** - Vector graphics access

### Lower Priority

10. **XFA Forms** - Legacy technology
11. **Object creation/modification** - Advanced editing
12. **Tile-based rendering** - Large document handling
13. **Linearization** - Web optimization

## Implementation Notes

### For Issue #27 (TextChar origin and rotation)

The current `TextChar` struct needs these additions:

```rust
pub struct TextChar {
    pub char: char,
    pub bbox: Rect,
    pub font_name: String,
    pub font_size: f32,
    pub font_weight: FontWeight,
    pub is_italic: bool,
    pub color: Color,
    pub mcid: Option<u32>,
    // NEW FIELDS:
    pub origin_x: f32,      // Baseline X position
    pub origin_y: f32,      // Baseline Y position
    pub rotation: f32,      // Rotation angle in degrees
}
```

The origin and rotation should be extracted from the text rendering matrix (Tm) during content stream parsing.

### Architecture Difference

**pdfium-render**: Wraps Google's PDFium C++ library via FFI. Benefits from PDFium's mature, battle-tested implementation. Drawback: requires distributing PDFium binary.

**pdf_oxide**: Pure Rust implementation. Benefits: no external dependencies, better WASM support, full control. Drawback: must implement everything from scratch based on PDF spec.

## Conclusion

pdf_oxide has strong coverage for:
- Basic text extraction
- Form reading/writing
- Page rendering
- Document manipulation
- Digital signatures
- PDF/A compliance

Main gaps compared to pdfium-render:
- Character-level positioning metadata (Issue #27)
- Advanced annotation types
- Low-level page object manipulation
- Some image metadata

The pure Rust approach gives pdf_oxide advantages in deployment simplicity and WASM support, while pdfium-render has more mature low-level access through PDFium.
