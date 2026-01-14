# Industry-Wide PDF Library Gap Analysis

This document compares pdf_oxide against major PDF libraries to identify feature gaps and industry-standard interfaces.

## Libraries Analyzed

| Library | Language | Type | Notable For |
|---------|----------|------|-------------|
| **pdfium-render** | Rust | FFI wrapper | Google PDFium bindings |
| **MuPDF** | C/C++ | Native | Fast, battle-tested |
| **iText** | Java | Native | Enterprise features |
| **Apache PDFBox** | Java | Native | Full-featured, mature |
| **pdf.js** | JavaScript | Native | Mozilla, browser rendering |
| **pypdf** | Python | Native | Pure Python |
| **pdfminer.six** | Python | Native | Layout analysis |
| **pdf-lib** | JavaScript | Native | PDF creation/editing |

---

## Text Character Properties

### Industry Standard: Character-Level Data

Every major PDF library provides these properties for each extracted character:

| Property | MuPDF | iText | PDFBox | pdf.js | pdfminer | pdfium-render | pdf_oxide |
|----------|-------|-------|--------|--------|----------|---------------|-----------|
| **Character value** | `c` | `getText()` | `getUnicode()` | `str` | `_text` | `unicode_char()` | ✅ `char` |
| **Origin X** | `origin.x` | baseline | `getX()` | `transform[4]` | via matrix | `origin_x()` | ❌ Missing |
| **Origin Y** | `origin.y` | baseline | `getY()` | `transform[5]` | via matrix | `origin_y()` | ❌ Missing |
| **Bounding box** | `quad` | getBBox | via metrics | w/h + transform | `bbox` | `tight_bounds()` | ✅ `bbox` |
| **Rotation angle** | via matrix | via matrix | `getDir()` | `transform` | `upright` | `angle_degrees()` | ❌ Missing |
| **Transform matrix** | `trm` | `getTextMatrix()` | `getTextMatrix()` | `transform` | `matrix` | `matrix()` | ❌ Missing |
| **Font name** | `font` | `getFont()` | `getFont()` | `fontName` | `fontname` | `font_name()` | ✅ `font_name` |
| **Font size** | `size` | `getFontSize()` | `getFontSize()` | via transform | `size` | `scaled_font_size()` | ✅ `font_size` |
| **Advance width** | `adv` | `getWidth()` | `getWidth()` | `width` | `adv` | via bounds | ❌ Missing |

### MuPDF `fz_stext_char` Structure
```c
struct fz_stext_char {
    int c;              // Unicode character
    fz_point origin;    // Origin point (baseline position)
    fz_quad quad;       // Quadrilateral bounding box (4 corners)
    float size;         // Font size
    fz_font *font;      // Font reference
};
```

### iText `TextRenderInfo` Methods
```java
getText()                    // Text string
getBaseline()               // LineSegment for baseline
getAscentLine()             // LineSegment for ascent
getDescentLine()            // LineSegment for descent
getFont()                   // PdfFont object
getCharacterRenderInfos()   // Individual glyph details
getRise()                   // Text rise value
getTextRenderMode()         // Fill, stroke, invisible, etc.
```

### Apache PDFBox `TextPosition` Properties
```java
getUnicode()          // Unicode string
getX(), getY()        // Position (page-rotation adjusted)
getXDirAdj()          // Direction-adjusted X
getYDirAdj()          // Direction-adjusted Y
getWidth()            // Character width
getHeight()           // Character height
getWidthDirAdj()      // Direction-adjusted width
getDir()              // Direction: 0, 90, 180, 270 degrees
getTextMatrix()       // Transformation matrix
getFont()             // PDFont object
getFontSize()         // Font size
getFontSizeInPt()     // Font size in points
getXScale()           // X scaling factor
getYScale()           // Y scaling factor
getWidthOfSpace()     // Width of space character
getIndividualWidths() // Widths array (for ligatures)
```

### pdf.js Text Item Structure
```javascript
{
    str: string,           // Text content
    dir: string,           // Direction ("ltr", "rtl")
    width: number,         // Width of text item
    height: number,        // Height of text item
    transform: [a,b,c,d,e,f], // 6-element transformation matrix
    fontName: string,      // Font identifier
    hasEOL: boolean        // End of line marker
}
```

### pdfminer.six `LTChar` Properties
```python
_text         # Character string
matrix        # Transformation matrix
fontname      # Font name
ncs           # Color space
graphicstate  # Graphic state (includes color)
adv           # Advance width
bbox          # Bounding box (x0, y0, x1, y1)
size          # Font size
upright       # Is character upright
```

---

## pdf_oxide Gap: TextChar Enhancement

**Current pdf_oxide `TextChar`:**
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
}
```

**Proposed Enhancement (Industry Standard):**
```rust
pub struct TextChar {
    // Existing
    pub char: char,
    pub bbox: Rect,
    pub font_name: String,
    pub font_size: f32,
    pub font_weight: FontWeight,
    pub is_italic: bool,
    pub color: Color,
    pub mcid: Option<u32>,

    // NEW: Origin/Baseline (Issue #27)
    pub origin_x: f32,           // Baseline X position
    pub origin_y: f32,           // Baseline Y position

    // NEW: Rotation (Issue #27)
    pub rotation_degrees: f32,   // Rotation angle in degrees

    // NEW: Transformation Matrix
    pub matrix: Option<[f32; 6]>, // Full text matrix [a,b,c,d,e,f]

    // NEW: Extended metrics
    pub advance_width: f32,      // Horizontal advance
    pub ascent: Option<f32>,     // Font ascent
    pub descent: Option<f32>,    // Font descent

    // NEW: Quadrilateral bounds (rotated text)
    pub quad: Option<Quad>,      // 4-corner bounding box
}

pub struct Quad {
    pub upper_left: Point,
    pub upper_right: Point,
    pub lower_right: Point,
    pub lower_left: Point,
}
```

---

## Image Object Properties

### Industry Standard: Image Metadata

| Property | MuPDF | iText | PDFBox | pdfium-render | pdf_oxide |
|----------|-------|-------|--------|---------------|-----------|
| Width (pixels) | `w` | via object | ✅ | `width()` | ✅ `width` |
| Height (pixels) | `h` | via object | ✅ | `height()` | ✅ `height` |
| DPI (X) | `xres` | via object | ✅ | `horizontal_dpi()` | ❌ Missing |
| DPI (Y) | `yres` | via object | ✅ | `vertical_dpi()` | ❌ Missing |
| Color space | `colorspace` | ✅ | ✅ | `color_space()` | ❌ Missing |
| Bits per component | `bpc` | ✅ | ✅ | `bits_per_pixel()` | ❌ Missing |
| Number of components | `n` | ✅ | ✅ | via color space | ❌ Missing |
| Interpolation | `interpolate` | ✅ | ✅ | ✅ | ❌ Missing |
| Image mask | `imagemask` | ✅ | ✅ | ✅ | ❌ Missing |
| Orientation/rotation | `orientation` | ✅ | ✅ | ✅ | ❌ Missing |
| Transform matrix | via container | ✅ | ✅ | ✅ | ❌ Missing |
| Raw data | `fz_compressed_buffer` | ✅ | ✅ | `get_raw_image()` | ✅ |
| Decoded pixmap | `fz_get_pixmap_from_image()` | ✅ | ✅ | `get_processed_image()` | ❌ Missing |
| Filters (JPEG, Flate, etc.) | via buffer | ✅ | ✅ | `filters()` | ❌ Missing |

### MuPDF `fz_image` Properties
```c
struct fz_image {
    int w, h;              // Dimensions in pixels
    int bpc;               // Bits per component
    int n;                 // Number of components
    fz_colorspace *colorspace;
    int xres, yres;        // Resolution in DPI
    int interpolate;       // Interpolation flag
    int imagemask;         // Is this an image mask
    int orientation;       // Rotation/flip info
};
```

---

## Annotation Types

### Industry Standard Coverage

| Type | MuPDF | iText | PDFBox | pdfium-render | pdf_oxide |
|------|-------|-------|--------|---------------|-----------|
| Text (Note) | ✅ | ✅ | ✅ | ✅ | ❌ |
| Link | ✅ | ✅ | ✅ | ✅ | ✅ |
| FreeText | ✅ | ✅ | ✅ | ✅ | ❌ |
| Line | ✅ | ✅ | ✅ | ✅ | ❌ |
| Square | ✅ | ✅ | ✅ | ✅ | ❌ |
| Circle | ✅ | ✅ | ✅ | ✅ | ❌ |
| Polygon | ✅ | ✅ | ✅ | ✅ | ❌ |
| PolyLine | ✅ | ✅ | ✅ | ✅ | ❌ |
| Highlight | ✅ | ✅ | ✅ | ✅ | ✅ |
| Underline | ✅ | ✅ | ✅ | ✅ | ❌ |
| Squiggly | ✅ | ✅ | ✅ | ✅ | ❌ |
| StrikeOut | ✅ | ✅ | ✅ | ✅ | ❌ |
| Stamp | ✅ | ✅ | ✅ | ✅ | ❌ |
| Caret | ✅ | ✅ | ✅ | ✅ | ❌ |
| Ink | ✅ | ✅ | ✅ | ✅ | ❌ |
| Popup | ✅ | ✅ | ✅ | ✅ | ❌ |
| FileAttachment | ✅ | ✅ | ✅ | ✅ | ❌ |
| Sound | ✅ | ✅ | ✅ | ✅ | ❌ |
| Movie | ✅ | ✅ | ✅ | ✅ | ❌ |
| Widget (Forms) | ✅ | ✅ | ✅ | ✅ | Partial |
| Redact | ✅ | ✅ | ✅ | ✅ | ❌ |

### Annotation Properties (Common)
```
- type: Annotation type enum
- rect: Bounding rectangle
- contents: Text content
- author: Creator name
- creation_date: Creation timestamp
- modification_date: Last modified
- flags: Visibility, print, read-only, etc.
- border: Border style and width
- color: Annotation color
- opacity: Transparency
- appearance: Visual appearance streams
```

---

## Page Objects

### Industry Standard: Object Types

| Object Type | MuPDF | iText | PDFBox | pdfium-render | pdf_oxide |
|-------------|-------|-------|--------|---------------|-----------|
| Text objects | ✅ | ✅ | ✅ | ✅ | Partial |
| Image objects | ✅ | ✅ | ✅ | ✅ | ✅ |
| Path objects | ✅ | ✅ | ✅ | ✅ | ❌ |
| Shading objects | ✅ | ✅ | ✅ | ✅ | ❌ |
| Form XObjects | ✅ | ✅ | ✅ | ✅ | ❌ |

### Path Object Properties (Industry Standard)
```
- path_data: Moveto, lineto, curveto commands
- fill_color: Fill color
- stroke_color: Stroke color
- stroke_width: Line width
- line_cap: Cap style
- line_join: Join style
- dash_pattern: Dash array
- transform_matrix: Transformation
- blend_mode: Compositing mode
- opacity: Transparency
```

---

## Text Extraction Features

### Bounded/Regional Text Extraction

All major libraries support extracting text from a specific rectangular region:

| Library | Method |
|---------|--------|
| MuPDF | Device with clip rect |
| iText | `TextRegionEventFilter` |
| PDFBox | `PDFTextStripperByArea` |
| pdf.js | Filter by transform coordinates |
| pdfminer | `LTFigure.bbox` filtering |
| pdfium-render | `bounded()`, `inside_rect()` |
| **pdf_oxide** | ❌ **Missing** |

### Layout Analysis Features

| Feature | MuPDF | pdfminer | PDFBox | pdf_oxide |
|---------|-------|----------|--------|-----------|
| Text blocks | ✅ `fz_stext_block` | ✅ `LTTextBox` | ✅ | ✅ `TextBlock` |
| Text lines | ✅ `fz_stext_line` | ✅ `LTTextLine` | ✅ | ✅ `TextLine` |
| Characters | ✅ `fz_stext_char` | ✅ `LTChar` | ✅ | ✅ `TextChar` |
| Reading order | ✅ | ✅ `boxes_flow` | Partial | ✅ |
| Column detection | ✅ | ✅ | Partial | ✅ |
| Table detection | Partial | ❌ | Partial | ❌ |

---

## Summary: Priority Gaps to Address

### Critical (Issue #27)
1. **TextChar origin X/Y** - Baseline position for precise text placement
2. **TextChar rotation** - Angle in degrees/radians

### High Priority
3. **TextChar transformation matrix** - Full positioning info
4. **Bounded text extraction** - Extract text from rectangular region
5. **More annotation types** - At minimum: Underline, StrikeOut, FreeText, Ink

### Medium Priority
6. **Image DPI** - Resolution metadata
7. **Image color space** - RGB, CMYK, Grayscale, etc.
8. **Path objects** - Vector graphics access
9. **Form flattening** - Merge form content into page

### Lower Priority
10. **TextChar advance width** - For custom text layout
11. **TextChar ascent/descent** - Font metrics
12. **Quad bounds** - 4-corner bounding box for rotated text
13. **Image decoded pixmap** - Process image to bitmap
14. **Redaction annotations** - Permanent content removal

---

## Implementation Recommendation

For Issue #27 and pdfium-render migration support, minimum changes to `TextChar`:

```rust
// Minimal addition for Issue #27
pub struct TextChar {
    // ... existing fields ...

    /// Baseline X position (origin point)
    pub origin_x: f32,

    /// Baseline Y position (origin point)
    pub origin_y: f32,

    /// Rotation angle in degrees (0-360)
    pub rotation_degrees: f32,
}
```

These values should be extracted from the text matrix (`Tm`) during content stream parsing in the text extraction pipeline.
