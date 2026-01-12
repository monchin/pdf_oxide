# Getting Started with PDFOxide (Python)

PDFOxide is the complete PDF toolkit. One library for extracting, creating, and editing PDFs with a unified API. Built on a Rust core for maximum performance.

## Installation

```bash
pip install pdf_oxide
```

## Quick Start - The Unified `Pdf` API

The `Pdf` class is your main entry point for all PDF operations:

```python
from pdf_oxide import Pdf

# Create from Markdown
pdf = Pdf.from_markdown("# Hello World\n\nThis is a PDF.")
pdf.save("output.pdf")
```

## Creating PDFs

### From Markdown

```python
from pdf_oxide import Pdf

pdf = Pdf.from_markdown("""
# Report Title

## Introduction

This is **bold** and *italic* text.

- Item 1
- Item 2
- Item 3

## Code Example

```python
print("Hello, World!")
```
""")
pdf.save("report.pdf")
```

### From HTML

```python
from pdf_oxide import Pdf

pdf = Pdf.from_html("""
<h1>Invoice</h1>
<p>Thank you for your purchase.</p>
<table>
    <tr><th>Item</th><th>Price</th></tr>
    <tr><td>Widget</td><td>$10.00</td></tr>
</table>
""")
pdf.save("invoice.pdf")
```

### From Plain Text

```python
from pdf_oxide import Pdf

pdf = Pdf.from_text("Simple plain text document.\n\nWith paragraphs.")
pdf.save("notes.pdf")
```

### From Images

```python
from pdf_oxide import Pdf

# Single image
pdf = Pdf.from_image("photo.jpg")
pdf.save("photo.pdf")

# Multiple images (one per page)
album = Pdf.from_images(["page1.jpg", "page2.png", "page3.jpg"])
album.save("album.pdf")
```

## Opening and Reading PDFs

```python
from pdf_oxide import PdfDocument

# Open existing PDF
doc = PdfDocument("document.pdf")

# Extract text from page 0
text = doc.extract_text(0)
print(f"Text: {text}")

# Convert to Markdown
markdown = doc.to_markdown(0)
print(f"Markdown:\n{markdown}")

# Get page count
print(f"Pages: {doc.page_count}")
```

## Builder Pattern for Advanced Creation

For full control over PDF creation, use `PdfBuilder`:

```python
from pdf_oxide import PdfBuilder, PageSize

pdf = (PdfBuilder()
    .title("Annual Report 2025")
    .author("Company Inc.")
    .subject("Financial Summary")
    .page_size(PageSize.A4)
    .margins(72.0, 72.0, 72.0, 72.0)  # 1 inch margins
    .font_size(11.0)
    .from_markdown("# Annual Report\n\n..."))

pdf.save("annual-report.pdf")
```

## Encryption and Security

### Password Protection

```python
from pdf_oxide import Pdf

pdf = Pdf.from_markdown("# Confidential Document")

# Simple password protection (AES-256)
pdf.save_encrypted("secure.pdf", "user-password", "owner-password")
```

## Text Extraction Options

### Basic Extraction

```python
from pdf_oxide import PdfDocument

doc = PdfDocument("paper.pdf")
text = doc.extract_text(0)
```

### With Options

```python
from pdf_oxide import PdfDocument, ConversionOptions

doc = PdfDocument("paper.pdf")
options = ConversionOptions(
    detect_headings=True,
    detect_lists=True,
    embed_images=True
)
markdown = doc.to_markdown(0, options)
```

### Extract All Pages

```python
from pdf_oxide import PdfDocument

doc = PdfDocument("book.pdf")

# Extract text from all pages
all_text = doc.extract_text_all()

# Convert entire document to Markdown
all_markdown = doc.to_markdown_all()
```

## Office Document Conversion

Convert DOCX, XLSX, and PPTX files to PDF:

```python
from pdf_oxide import OfficeConverter

# Auto-detect format
converter = OfficeConverter()

# Convert Word document
converter.convert("report.docx", "report.pdf")

# Convert Excel spreadsheet
converter.convert("data.xlsx", "data.pdf")

# Convert PowerPoint presentation
converter.convert("slides.pptx", "slides.pdf")
```

## Working with Images

### Extract Images from PDF

```python
from pdf_oxide import PdfDocument

doc = PdfDocument("document.pdf")
images = doc.extract_images(0)

for i, img in enumerate(images):
    img.save(f"image_{i}.png")
```

### Embed Images in Output

```python
from pdf_oxide import PdfDocument, ConversionOptions

doc = PdfDocument("paper.pdf")
options = ConversionOptions(embed_images=True)

# Images embedded as base64 data URIs
html = doc.to_html(0, options)
```

## Performance Tips

1. **Reuse document objects** - Opening a PDF has overhead, reuse the object for multiple operations
2. **Use specific page extraction** - `extract_text(page_num)` is faster than `extract_text_all()` if you only need some pages
3. **Disable features you don't need** - Use `ConversionOptions` to skip heading detection, image extraction, etc.

```python
from pdf_oxide import PdfDocument, ConversionOptions

doc = PdfDocument("large.pdf")

# Fast extraction - minimal processing
options = ConversionOptions(
    detect_headings=False,
    detect_lists=False,
    embed_images=False
)
text = doc.to_markdown(0, options)
```

## Error Handling

```python
from pdf_oxide import PdfDocument, PdfError

try:
    doc = PdfDocument("document.pdf")
    text = doc.extract_text(0)
except PdfError as e:
    print(f"PDF error: {e}")
except FileNotFoundError:
    print("File not found")
```

## Examples

See the [examples/](../examples/) directory for complete working examples.

### Quick Script Examples

**Extract text from all PDFs in a folder:**

```python
from pdf_oxide import PdfDocument
from pathlib import Path

for pdf_path in Path("documents").glob("*.pdf"):
    doc = PdfDocument(str(pdf_path))
    text = doc.extract_text_all()

    output_path = pdf_path.with_suffix(".txt")
    output_path.write_text(text)
    print(f"Extracted: {pdf_path.name}")
```

**Batch convert Markdown to PDF:**

```python
from pdf_oxide import Pdf
from pathlib import Path

for md_path in Path("notes").glob("*.md"):
    content = md_path.read_text()
    pdf = Pdf.from_markdown(content)

    output_path = md_path.with_suffix(".pdf")
    pdf.save(str(output_path))
    print(f"Created: {output_path.name}")
```

## Next Steps

- [API Reference](https://docs.rs/pdf_oxide) - Full API documentation
- [PDF Creation Guide](PDF_CREATION_GUIDE.md) - Advanced creation options
- [GitHub Issues](https://github.com/yfedoseev/pdf_oxide/issues) - Report bugs or request features
