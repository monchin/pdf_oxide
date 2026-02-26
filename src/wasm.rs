//! WebAssembly bindings for PDF Oxide.
//!
//! Provides a JavaScript/TypeScript API for PDF text extraction in browser
//! environments. Requires the `wasm` feature flag.
//!
//! # Example (JavaScript)
//!
//! ```javascript
//! import init, { WasmPdfDocument } from 'pdf_oxide';
//!
//! await init();
//! const response = await fetch('document.pdf');
//! const bytes = new Uint8Array(await response.arrayBuffer());
//! const doc = new WasmPdfDocument(bytes);
//! console.log(`Pages: ${doc.page_count()}`);
//! console.log(doc.extract_text(0));
//! doc.free();
//! ```

use wasm_bindgen::prelude::*;

use crate::converters::ConversionOptions;
use crate::document::PdfDocument;

/// A PDF document loaded from bytes for use in WebAssembly.
///
/// Create an instance by passing PDF file bytes to the constructor.
/// Call `.free()` when done to release memory.
#[wasm_bindgen]
pub struct WasmPdfDocument {
    inner: PdfDocument,
}

#[wasm_bindgen]
impl WasmPdfDocument {
    /// Load a PDF document from raw bytes.
    ///
    /// @param data - The PDF file contents as a Uint8Array
    /// @throws Error if the PDF is invalid or cannot be parsed
    #[wasm_bindgen(constructor)]
    pub fn new(data: &[u8]) -> Result<WasmPdfDocument, JsValue> {
        // Set up better panic messages in debug builds
        #[cfg(feature = "wasm")]
        console_error_panic_hook::set_once();

        let inner = PdfDocument::open_from_bytes(data.to_vec())
            .map_err(|e| JsValue::from_str(&format!("Failed to open PDF: {}", e)))?;

        Ok(WasmPdfDocument { inner })
    }

    /// Get the number of pages in the document.
    ///
    /// @returns The page count
    /// @throws Error if the page tree is invalid
    #[wasm_bindgen(js_name = "pageCount")]
    pub fn page_count(&mut self) -> Result<usize, JsValue> {
        self.inner
            .page_count()
            .map_err(|e| JsValue::from_str(&format!("Failed to get page count: {}", e)))
    }

    /// Extract plain text from a single page.
    ///
    /// @param page_index - Zero-based page number
    /// @returns The extracted text
    /// @throws Error if extraction fails or page index is out of range
    #[wasm_bindgen(js_name = "extractText")]
    pub fn extract_text(&mut self, page_index: usize) -> Result<String, JsValue> {
        self.inner
            .extract_text(page_index)
            .map_err(|e| JsValue::from_str(&format!("Failed to extract text: {}", e)))
    }

    /// Extract plain text from all pages, separated by form feed characters.
    ///
    /// @returns The full document text
    /// @throws Error if extraction fails
    #[wasm_bindgen(js_name = "extractAllText")]
    pub fn extract_all_text(&mut self) -> Result<String, JsValue> {
        self.inner
            .extract_all_text()
            .map_err(|e| JsValue::from_str(&format!("Failed to extract all text: {}", e)))
    }

    /// Convert a single page to Markdown.
    ///
    /// @param page_index - Zero-based page number
    /// @returns The page content as Markdown
    /// @throws Error if conversion fails or page index is out of range
    #[wasm_bindgen(js_name = "toMarkdown")]
    pub fn to_markdown(&mut self, page_index: usize) -> Result<String, JsValue> {
        let opts = ConversionOptions::default();
        self.inner
            .to_markdown(page_index, &opts)
            .map_err(|e| JsValue::from_str(&format!("Failed to convert to markdown: {}", e)))
    }
}
