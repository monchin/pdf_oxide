//! Integration tests for the high-level PDF API.

use pdf_oxide::api::{Pdf, PdfBuilder, PdfConfig};
use pdf_oxide::writer::PageSize;
use tempfile::tempdir;

mod pdf_config_tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PdfConfig::default();
        assert_eq!(config.margin_left, 72.0);
        assert_eq!(config.margin_right, 72.0);
        assert_eq!(config.margin_top, 72.0);
        assert_eq!(config.margin_bottom, 72.0);
        assert_eq!(config.font_size, 12.0);
        assert_eq!(config.line_height, 1.5);
        assert!(config.title.is_none());
        assert!(config.author.is_none());
    }
}

mod pdf_builder_tests {
    use super::*;

    #[test]
    fn test_builder_creates_pdf() {
        // Test that builder can create a PDF
        let result = PdfBuilder::new().from_text("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_title() {
        // Test builder with title creates valid PDF
        let result = PdfBuilder::new().title("Test Title").from_text("Content");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_author() {
        let result = PdfBuilder::new().author("Test Author").from_text("Content");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_subject() {
        let result = PdfBuilder::new()
            .subject("Test Subject")
            .from_text("Content");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_keywords() {
        let result = PdfBuilder::new()
            .keywords("test, keywords")
            .from_text("Content");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_page_size() {
        let result = PdfBuilder::new()
            .page_size(PageSize::A4)
            .from_text("A4 content");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_margin() {
        let result = PdfBuilder::new().margin(50.0).from_text("Custom margin");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_margins() {
        let result = PdfBuilder::new()
            .margins(10.0, 20.0, 30.0, 40.0)
            .from_text("Custom margins");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_font_size() {
        let result = PdfBuilder::new()
            .font_size(14.0)
            .from_text("Custom font size");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_line_height() {
        let result = PdfBuilder::new()
            .line_height(1.8)
            .from_text("Custom line height");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_chain() {
        let result = PdfBuilder::new()
            .title("Title")
            .author("Author")
            .page_size(PageSize::Letter)
            .margin(72.0)
            .font_size(12.0)
            .from_text("Chained builder");

        assert!(result.is_ok());
        let pdf = result.unwrap();
        assert!(!pdf.as_bytes().is_empty());
    }
}

mod pdf_creation_tests {
    use super::*;

    #[test]
    fn test_from_text_simple() {
        let result = Pdf::from_text("Hello, World!");
        assert!(result.is_ok());

        let pdf = result.unwrap();
        assert!(!pdf.as_bytes().is_empty());
        assert!(pdf.as_bytes().starts_with(b"%PDF"));
    }

    #[test]
    fn test_from_text_multiline() {
        let result = Pdf::from_text("Line 1\nLine 2\nLine 3");
        assert!(result.is_ok());

        let pdf = result.unwrap();
        assert!(!pdf.as_bytes().is_empty());
    }

    #[test]
    fn test_from_markdown_heading() {
        let result = Pdf::from_markdown("# Heading 1\n\nSome text.");
        assert!(result.is_ok());

        let pdf = result.unwrap();
        assert!(!pdf.as_bytes().is_empty());
    }

    #[test]
    fn test_from_markdown_multiple_headings() {
        let markdown = r#"
# Chapter 1

Introduction text.

## Section 1.1

More content here.

### Subsection 1.1.1

Even more content.
"#;
        let result = Pdf::from_markdown(markdown);
        assert!(result.is_ok());
    }

    #[test]
    fn test_from_markdown_list() {
        let markdown = r#"
# Shopping List

- Apples
- Bananas
- Oranges
"#;
        let result = Pdf::from_markdown(markdown);
        assert!(result.is_ok());
    }

    #[test]
    fn test_from_markdown_blockquote() {
        let markdown = r#"
# Quote

> This is a quote.
> It spans multiple lines.
"#;
        let result = Pdf::from_markdown(markdown);
        assert!(result.is_ok());
    }

    #[test]
    fn test_from_markdown_code_block() {
        let markdown = r#"
# Code Example

```
fn main() {
    println!("Hello");
}
```
"#;
        let result = Pdf::from_markdown(markdown);
        assert!(result.is_ok());
    }

    #[test]
    fn test_from_html_simple() {
        let result = Pdf::from_html("<h1>Hello</h1><p>World</p>");
        assert!(result.is_ok());

        let pdf = result.unwrap();
        assert!(!pdf.as_bytes().is_empty());
    }

    #[test]
    fn test_from_html_formatting() {
        let html = "<h1>Title</h1><p>This is <b>bold</b> and <i>italic</i>.</p>";
        let result = Pdf::from_html(html);
        assert!(result.is_ok());
    }

    #[test]
    fn test_from_html_list() {
        let html = "<h1>List</h1><ul><li>Item 1</li><li>Item 2</li></ul>";
        let result = Pdf::from_html(html);
        assert!(result.is_ok());
    }

    #[test]
    fn test_into_bytes() {
        let pdf = Pdf::from_text("Test").unwrap();
        let bytes = pdf.into_bytes();

        assert!(!bytes.is_empty());
        assert!(bytes.starts_with(b"%PDF"));
    }

    #[test]
    fn test_config_access() {
        let pdf = PdfBuilder::new()
            .title("My Title")
            .from_text("Content")
            .unwrap();

        let config = pdf.config();
        assert_eq!(config.title, Some("My Title".to_string()));
    }
}

mod pdf_save_tests {
    use super::*;

    #[test]
    fn test_save_text_pdf() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("text.pdf");

        let mut pdf = Pdf::from_text("Hello, World!").unwrap();
        let result = pdf.save(&path);

        assert!(result.is_ok());
        assert!(path.exists());

        // Check file starts with PDF header
        let bytes = std::fs::read(&path).unwrap();
        assert!(bytes.starts_with(b"%PDF"));
    }

    #[test]
    fn test_save_markdown_pdf() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("markdown.pdf");

        let mut pdf = Pdf::from_markdown("# Hello\n\nWorld").unwrap();
        let result = pdf.save(&path);

        assert!(result.is_ok());
        assert!(path.exists());
    }

    #[test]
    fn test_save_html_pdf() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("html.pdf");

        let mut pdf = Pdf::from_html("<h1>Hello</h1>").unwrap();
        let result = pdf.save(&path);

        assert!(result.is_ok());
        assert!(path.exists());
    }

    #[test]
    fn test_save_with_metadata() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("metadata.pdf");

        let mut pdf = PdfBuilder::new()
            .title("Test Document")
            .author("Test Author")
            .subject("Testing")
            .from_text("Content")
            .unwrap();

        let result = pdf.save(&path);
        assert!(result.is_ok());
        assert!(path.exists());
    }
}

mod builder_to_pdf_tests {
    use super::*;

    #[test]
    fn test_builder_from_text() {
        let pdf = PdfBuilder::new()
            .title("Plain Text")
            .from_text("Plain text content")
            .unwrap();

        assert!(!pdf.as_bytes().is_empty());
    }

    #[test]
    fn test_builder_from_markdown() {
        let pdf = PdfBuilder::new()
            .title("Markdown Doc")
            .author("Author")
            .page_size(PageSize::A4)
            .from_markdown("# Title\n\nContent")
            .unwrap();

        assert!(!pdf.as_bytes().is_empty());
    }

    #[test]
    fn test_builder_from_html() {
        let pdf = PdfBuilder::new()
            .title("HTML Doc")
            .margin(50.0)
            .from_html("<h1>Title</h1><p>Content</p>")
            .unwrap();

        assert!(!pdf.as_bytes().is_empty());
    }

    #[test]
    fn test_builder_custom_fonts() {
        let pdf = PdfBuilder::new()
            .font_size(14.0)
            .line_height(1.6)
            .from_text("Custom font settings")
            .unwrap();

        assert!(!pdf.as_bytes().is_empty());
    }

    #[test]
    fn test_builder_custom_margins() {
        let pdf = PdfBuilder::new()
            .margins(36.0, 36.0, 72.0, 72.0)
            .from_text("Custom margins")
            .unwrap();

        assert!(!pdf.as_bytes().is_empty());
    }
}

mod integration_tests {
    use super::*;
    use pdf_oxide::editor::{DocumentEditor, EditableDocument};

    #[test]
    fn test_create_and_open() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("roundtrip.pdf");

        // Create PDF
        let mut pdf = Pdf::from_markdown("# Test Document\n\nContent here.").unwrap();
        pdf.save(&path).unwrap();

        // Open with editor
        let editor = DocumentEditor::open(&path);
        assert!(editor.is_ok());

        let mut editor = editor.unwrap();
        assert!(editor.page_count().unwrap() >= 1);
    }

    #[test]
    fn test_full_workflow() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("workflow.pdf");

        // Build with full options
        let mut pdf = PdfBuilder::new()
            .title("Complete Document")
            .author("Integration Test")
            .subject("Testing all features")
            .keywords("test, integration, pdf")
            .page_size(PageSize::Letter)
            .margin(72.0)
            .font_size(12.0)
            .line_height(1.5)
            .from_markdown(
                r#"
# Introduction

This is a complete test document.

## Features

- Headings
- Lists
- Text formatting

## Code

```
let x = 42;
```

> And a quote for good measure.
"#,
            )
            .unwrap();

        // Save
        pdf.save(&path).unwrap();

        // Verify
        assert!(path.exists());
        let bytes = std::fs::read(&path).unwrap();
        assert!(bytes.starts_with(b"%PDF"));
        assert!(bytes.len() > 100);
    }
}
