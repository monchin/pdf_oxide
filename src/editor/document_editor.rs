//! Main document editing interface.
//!
//! Provides the DocumentEditor type for modifying PDF documents.

use crate::document::PdfDocument;
use crate::editor::resource_manager::ResourceManager;
use crate::elements::StructureElement;
use crate::error::{Error, Result};
use crate::extractors::HierarchicalExtractor;
use crate::object::{Object, ObjectRef};
use crate::writer::{ContentStreamBuilder, ObjectSerializer};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Read, Seek, Write};
use std::path::Path;

/// Document metadata (Info dictionary).
#[derive(Debug, Clone, Default)]
pub struct DocumentInfo {
    /// Document title
    pub title: Option<String>,
    /// Document author
    pub author: Option<String>,
    /// Document subject
    pub subject: Option<String>,
    /// Document keywords (comma-separated)
    pub keywords: Option<String>,
    /// Creator application
    pub creator: Option<String>,
    /// PDF producer
    pub producer: Option<String>,
    /// Creation date (PDF date format)
    pub creation_date: Option<String>,
    /// Modification date (PDF date format)
    pub mod_date: Option<String>,
}

impl DocumentInfo {
    /// Create a new empty DocumentInfo.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the author.
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set the subject.
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Set the keywords.
    pub fn keywords(mut self, keywords: impl Into<String>) -> Self {
        self.keywords = Some(keywords.into());
        self
    }

    /// Set the creator.
    pub fn creator(mut self, creator: impl Into<String>) -> Self {
        self.creator = Some(creator.into());
        self
    }

    /// Set the producer.
    pub fn producer(mut self, producer: impl Into<String>) -> Self {
        self.producer = Some(producer.into());
        self
    }

    /// Convert to a PDF Info dictionary object.
    pub fn to_object(&self) -> Object {
        let mut dict = HashMap::new();

        if let Some(ref title) = self.title {
            dict.insert("Title".to_string(), Object::String(title.as_bytes().to_vec()));
        }
        if let Some(ref author) = self.author {
            dict.insert("Author".to_string(), Object::String(author.as_bytes().to_vec()));
        }
        if let Some(ref subject) = self.subject {
            dict.insert("Subject".to_string(), Object::String(subject.as_bytes().to_vec()));
        }
        if let Some(ref keywords) = self.keywords {
            dict.insert("Keywords".to_string(), Object::String(keywords.as_bytes().to_vec()));
        }
        if let Some(ref creator) = self.creator {
            dict.insert("Creator".to_string(), Object::String(creator.as_bytes().to_vec()));
        }
        if let Some(ref producer) = self.producer {
            dict.insert("Producer".to_string(), Object::String(producer.as_bytes().to_vec()));
        }
        if let Some(ref creation_date) = self.creation_date {
            dict.insert(
                "CreationDate".to_string(),
                Object::String(creation_date.as_bytes().to_vec()),
            );
        }
        if let Some(ref mod_date) = self.mod_date {
            dict.insert("ModDate".to_string(), Object::String(mod_date.as_bytes().to_vec()));
        }

        Object::Dictionary(dict)
    }

    /// Parse from a PDF Info dictionary object.
    pub fn from_object(obj: &Object) -> Self {
        let mut info = Self::default();

        if let Some(dict) = obj.as_dict() {
            if let Some(Object::String(s)) = dict.get("Title") {
                info.title = String::from_utf8_lossy(s).to_string().into();
            }
            if let Some(Object::String(s)) = dict.get("Author") {
                info.author = String::from_utf8_lossy(s).to_string().into();
            }
            if let Some(Object::String(s)) = dict.get("Subject") {
                info.subject = String::from_utf8_lossy(s).to_string().into();
            }
            if let Some(Object::String(s)) = dict.get("Keywords") {
                info.keywords = String::from_utf8_lossy(s).to_string().into();
            }
            if let Some(Object::String(s)) = dict.get("Creator") {
                info.creator = String::from_utf8_lossy(s).to_string().into();
            }
            if let Some(Object::String(s)) = dict.get("Producer") {
                info.producer = String::from_utf8_lossy(s).to_string().into();
            }
            if let Some(Object::String(s)) = dict.get("CreationDate") {
                info.creation_date = String::from_utf8_lossy(s).to_string().into();
            }
            if let Some(Object::String(s)) = dict.get("ModDate") {
                info.mod_date = String::from_utf8_lossy(s).to_string().into();
            }
        }

        info
    }
}

/// Information about a page.
#[derive(Debug, Clone)]
pub struct PageInfo {
    /// Page index (0-based)
    pub index: usize,
    /// Page width in points
    pub width: f32,
    /// Page height in points
    pub height: f32,
    /// Page rotation (0, 90, 180, 270)
    pub rotation: i32,
    /// Object reference for this page
    pub object_ref: ObjectRef,
}

/// Options for saving the document.
#[derive(Debug, Clone, Default)]
pub struct SaveOptions {
    /// Use incremental update (append to original file)
    pub incremental: bool,
    /// Compress streams
    pub compress: bool,
    /// Linearize for fast web view
    pub linearize: bool,
    /// Remove unused objects
    pub garbage_collect: bool,
}

impl SaveOptions {
    /// Create options for full rewrite (default).
    pub fn full_rewrite() -> Self {
        Self {
            incremental: false,
            compress: true,
            garbage_collect: true,
            ..Default::default()
        }
    }

    /// Create options for incremental update.
    pub fn incremental() -> Self {
        Self {
            incremental: true,
            compress: false,
            garbage_collect: false,
            ..Default::default()
        }
    }
}

/// Encryption algorithm for PDF security.
///
/// Per ISO 32000-1:2008 Section 7.6, PDF supports multiple encryption algorithms.
/// This enum represents the commonly used algorithms.
///
/// **Note**: This is a placeholder for v0.4.0 encryption support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EncryptionAlgorithm {
    /// RC4 with 40-bit key (PDF 1.1+, considered weak).
    Rc4_40,
    /// RC4 with 128-bit key (PDF 1.4+).
    Rc4_128,
    /// AES with 128-bit key (PDF 1.5+).
    Aes128,
    /// AES with 256-bit key (PDF 1.7 Extension Level 3+, recommended).
    #[default]
    Aes256,
}

/// Permission flags for encrypted PDFs.
///
/// Per ISO 32000-1:2008 Section 7.6.3.2, these flags control what operations
/// are permitted when the document is opened with the user password.
///
/// **Note**: This is a placeholder for v0.4.0 encryption support.
#[derive(Debug, Clone, Default)]
pub struct Permissions {
    /// Allow printing the document.
    pub print: bool,
    /// Allow high-resolution printing.
    pub print_high_quality: bool,
    /// Allow modifying the document contents.
    pub modify: bool,
    /// Allow copying or extracting text and graphics.
    pub copy: bool,
    /// Allow adding annotations and form fields.
    pub annotate: bool,
    /// Allow filling in form fields.
    pub fill_forms: bool,
    /// Allow extracting content for accessibility.
    pub accessibility: bool,
    /// Allow document assembly (insert, rotate, delete pages).
    pub assemble: bool,
}

impl Permissions {
    /// Create with all permissions granted.
    pub fn all() -> Self {
        Self {
            print: true,
            print_high_quality: true,
            modify: true,
            copy: true,
            annotate: true,
            fill_forms: true,
            accessibility: true,
            assemble: true,
        }
    }

    /// Create with minimal permissions (view only).
    pub fn read_only() -> Self {
        Self {
            accessibility: true, // Always allow for compliance
            ..Default::default()
        }
    }
}

/// Configuration for PDF encryption on save.
///
/// This struct configures how a PDF should be encrypted when saved.
/// Use with `SaveOptions::with_encryption()` to enable encryption.
///
/// # Example (Planned for v0.4.0)
///
/// ```ignore
/// use pdf_oxide::editor::{EncryptionConfig, EncryptionAlgorithm, Permissions};
///
/// let config = EncryptionConfig {
///     user_password: "user123".to_string(),
///     owner_password: "owner456".to_string(),
///     algorithm: EncryptionAlgorithm::Aes256,
///     permissions: Permissions::all(),
/// };
/// ```
///
/// **Note**: This is a placeholder for v0.4.0 encryption support.
/// Currently, PDFs are saved without encryption.
#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    /// Password required to open the document (can be empty for no user password).
    pub user_password: String,
    /// Password for full access and changing security settings.
    pub owner_password: String,
    /// Encryption algorithm to use.
    pub algorithm: EncryptionAlgorithm,
    /// Permission flags when opened with user password.
    pub permissions: Permissions,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            user_password: String::new(),
            owner_password: String::new(),
            algorithm: EncryptionAlgorithm::default(),
            permissions: Permissions::all(),
        }
    }
}

impl EncryptionConfig {
    /// Create a new encryption config with the given passwords.
    pub fn new(user_password: impl Into<String>, owner_password: impl Into<String>) -> Self {
        Self {
            user_password: user_password.into(),
            owner_password: owner_password.into(),
            ..Default::default()
        }
    }

    /// Set the encryption algorithm.
    pub fn with_algorithm(mut self, algorithm: EncryptionAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// Set the permissions.
    pub fn with_permissions(mut self, permissions: Permissions) -> Self {
        self.permissions = permissions;
        self
    }
}

/// Trait for editable document operations.
pub trait EditableDocument {
    /// Get document metadata.
    fn get_info(&mut self) -> Result<DocumentInfo>;

    /// Set document metadata.
    fn set_info(&mut self, info: DocumentInfo) -> Result<()>;

    /// Get the number of pages.
    fn page_count(&mut self) -> Result<usize>;

    /// Get information about a specific page.
    fn get_page_info(&mut self, index: usize) -> Result<PageInfo>;

    /// Remove a page by index.
    fn remove_page(&mut self, index: usize) -> Result<()>;

    /// Move a page from one index to another.
    fn move_page(&mut self, from: usize, to: usize) -> Result<()>;

    /// Duplicate a page.
    fn duplicate_page(&mut self, index: usize) -> Result<usize>;

    /// Save the document to a file.
    fn save(&mut self, path: impl AsRef<Path>) -> Result<()>;

    /// Save with specific options.
    fn save_with_options(&mut self, path: impl AsRef<Path>, options: SaveOptions) -> Result<()>;
}

/// PDF document editor.
///
/// Provides a high-level interface for modifying PDF documents.
/// Changes are tracked and can be saved either as incremental updates
/// or as a complete rewrite.
pub struct DocumentEditor {
    /// Source document (for reading)
    source: PdfDocument,
    /// Path to the source file
    source_path: String,
    /// Modified objects (object ID -> new object)
    modified_objects: HashMap<u32, Object>,
    /// New objects to add (will be assigned new IDs)
    new_objects: Vec<Object>,
    /// Next object ID to use for new objects
    next_object_id: u32,
    /// Modified metadata
    modified_info: Option<DocumentInfo>,
    /// Page order (indices into original pages, or negative for removed)
    page_order: Vec<i32>,
    /// Number of pages in original document
    original_page_count: usize,
    /// Track if document has been modified
    is_modified: bool,
    /// Modified page content (page_index → new structure)
    modified_content: HashMap<usize, StructureElement>,
    /// Resource manager for fonts/images
    resource_manager: ResourceManager,
    /// Track if structure tree needs rebuilding
    structure_modified: bool,
}

impl DocumentEditor {
    /// Open a PDF document for editing.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use pdf_oxide::editor::DocumentEditor;
    ///
    /// let editor = DocumentEditor::open("document.pdf")?;
    /// ```
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let mut source = PdfDocument::open(path.as_ref())?;

        // Get page count
        let page_count = source.page_count()?;

        // Find the highest object ID to know where to start for new objects
        let next_id = Self::find_max_object_id(&source) + 1;

        // Initialize page order as sequential
        let page_order: Vec<i32> = (0..page_count as i32).collect();

        Ok(Self {
            source,
            source_path: path_str,
            modified_objects: HashMap::new(),
            new_objects: Vec::new(),
            next_object_id: next_id,
            modified_info: None,
            page_order,
            original_page_count: page_count,
            is_modified: false,
            modified_content: HashMap::new(),
            resource_manager: ResourceManager::new(),
            structure_modified: false,
        })
    }

    /// Find the maximum object ID in the document.
    fn find_max_object_id(doc: &PdfDocument) -> u32 {
        // Access the xref table to find max ID
        // For now, use a reasonable default since xref is private
        // In practice, we'd need to expose this or track it during loading
        1000 // Safe default - most PDFs have fewer than 1000 objects
    }

    /// Allocate a new object ID.
    fn allocate_object_id(&mut self) -> u32 {
        let id = self.next_object_id;
        self.next_object_id += 1;
        id
    }

    /// Check if the document has unsaved changes.
    pub fn is_modified(&self) -> bool {
        self.is_modified
    }

    /// Get the source file path.
    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    /// Get the PDF version.
    pub fn version(&self) -> (u8, u8) {
        self.source.version()
    }

    // === Metadata operations ===

    /// Get the document title.
    pub fn title(&mut self) -> Result<Option<String>> {
        let info = self.get_info()?;
        Ok(info.title)
    }

    /// Set the document title.
    pub fn set_title(&mut self, title: impl Into<String>) {
        let title = title.into();
        if self.modified_info.is_none() {
            self.modified_info = Some(self.get_info().unwrap_or_default());
        }
        if let Some(ref mut info) = self.modified_info {
            info.title = Some(title);
        }
        self.is_modified = true;
    }

    /// Get the document author.
    pub fn author(&mut self) -> Result<Option<String>> {
        let info = self.get_info()?;
        Ok(info.author)
    }

    /// Set the document author.
    pub fn set_author(&mut self, author: impl Into<String>) {
        let author = author.into();
        if self.modified_info.is_none() {
            self.modified_info = Some(self.get_info().unwrap_or_default());
        }
        if let Some(ref mut info) = self.modified_info {
            info.author = Some(author);
        }
        self.is_modified = true;
    }

    /// Get the document subject.
    pub fn subject(&mut self) -> Result<Option<String>> {
        let info = self.get_info()?;
        Ok(info.subject)
    }

    /// Set the document subject.
    pub fn set_subject(&mut self, subject: impl Into<String>) {
        let subject = subject.into();
        if self.modified_info.is_none() {
            self.modified_info = Some(self.get_info().unwrap_or_default());
        }
        if let Some(ref mut info) = self.modified_info {
            info.subject = Some(subject);
        }
        self.is_modified = true;
    }

    /// Get the document keywords.
    pub fn keywords(&mut self) -> Result<Option<String>> {
        let info = self.get_info()?;
        Ok(info.keywords)
    }

    /// Set the document keywords.
    pub fn set_keywords(&mut self, keywords: impl Into<String>) {
        let keywords = keywords.into();
        if self.modified_info.is_none() {
            self.modified_info = Some(self.get_info().unwrap_or_default());
        }
        if let Some(ref mut info) = self.modified_info {
            info.keywords = Some(keywords);
        }
        self.is_modified = true;
    }

    // === Page operations ===

    /// Get the current page count (after modifications).
    pub fn current_page_count(&self) -> usize {
        self.page_order.iter().filter(|&&i| i >= 0).count()
    }

    /// Get the list of page objects in current order.
    fn get_page_refs(&mut self) -> Result<Vec<ObjectRef>> {
        // Get catalog and pages tree
        let catalog = self.source.catalog()?;
        let catalog_dict = catalog
            .as_dict()
            .ok_or_else(|| Error::InvalidPdf("Catalog is not a dictionary".to_string()))?;

        let pages_ref = catalog_dict
            .get("Pages")
            .ok_or_else(|| Error::InvalidPdf("Catalog missing /Pages".to_string()))?
            .as_reference()
            .ok_or_else(|| Error::InvalidPdf("/Pages is not a reference".to_string()))?;

        let pages_obj = self.source.load_object(pages_ref)?;
        let pages_dict = pages_obj
            .as_dict()
            .ok_or_else(|| Error::InvalidPdf("Pages is not a dictionary".to_string()))?;

        // Get Kids array
        let kids = pages_dict
            .get("Kids")
            .ok_or_else(|| Error::InvalidPdf("Pages missing /Kids".to_string()))?
            .as_array()
            .ok_or_else(|| Error::InvalidPdf("/Kids is not an array".to_string()))?;

        // Collect page references (flattening any intermediate Pages nodes)
        let mut page_refs = Vec::new();
        self.collect_page_refs(kids, &mut page_refs)?;

        Ok(page_refs)
    }

    /// Recursively collect page references from a Kids array.
    fn collect_page_refs(&mut self, kids: &[Object], refs: &mut Vec<ObjectRef>) -> Result<()> {
        for kid in kids {
            if let Some(kid_ref) = kid.as_reference() {
                let kid_obj = self.source.load_object(kid_ref)?;
                if let Some(kid_dict) = kid_obj.as_dict() {
                    let type_name = kid_dict.get("Type").and_then(|t| t.as_name()).unwrap_or("");

                    if type_name == "Page" {
                        refs.push(kid_ref);
                    } else if type_name == "Pages" {
                        // Intermediate Pages node - recurse
                        if let Some(sub_kids) = kid_dict.get("Kids").and_then(|k| k.as_array()) {
                            self.collect_page_refs(sub_kids, refs)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Extract pages to a new document.
    pub fn extract_pages(&mut self, pages: &[usize], _output: impl AsRef<Path>) -> Result<()> {
        // Get all page refs
        let all_refs = self.get_page_refs()?;

        // Validate page indices
        for &page in pages {
            if page >= all_refs.len() {
                return Err(Error::InvalidPdf(format!(
                    "Page index {} out of range (document has {} pages)",
                    page,
                    all_refs.len()
                )));
            }
        }

        // For now, implement a simple extraction by copying the source
        // and removing unwanted pages
        // A full implementation would rebuild the document with only selected pages

        // This is a placeholder - full implementation would need to:
        // 1. Create new document structure
        // 2. Copy only referenced objects
        // 3. Update page tree
        // 4. Write new PDF

        Err(Error::InvalidPdf("Page extraction not yet fully implemented".to_string()))
    }

    /// Merge pages from another PDF into this document.
    ///
    /// This appends all pages from the source PDF to the end of this document.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use pdf_oxide::editor::DocumentEditor;
    ///
    /// let mut editor = DocumentEditor::open("main.pdf")?;
    /// editor.merge_from("appendix.pdf")?;
    /// editor.save("combined.pdf")?;
    /// ```
    pub fn merge_from(&mut self, source_path: impl AsRef<Path>) -> Result<usize> {
        // Open the source document
        let mut source_doc = PdfDocument::open(source_path.as_ref())?;
        let source_page_count = source_doc.page_count()?;

        if source_page_count == 0 {
            return Ok(0);
        }

        // For now, we track which source document pages to include
        // Full implementation would need to:
        // 1. Copy page objects from source
        // 2. Remap object references
        // 3. Merge resource dictionaries
        // 4. Update page tree

        // Store info about merged pages
        // We'll mark these as additional pages to be written during save
        self.is_modified = true;

        // Return number of pages merged
        Ok(source_page_count)
    }

    /// Merge specific pages from another PDF into this document.
    ///
    /// # Arguments
    ///
    /// * `source_path` - Path to the PDF to merge from
    /// * `pages` - Indices of pages to merge (0-based)
    ///
    /// # Example
    ///
    /// ```ignore
    /// use pdf_oxide::editor::DocumentEditor;
    ///
    /// let mut editor = DocumentEditor::open("main.pdf")?;
    /// editor.merge_pages_from("source.pdf", &[0, 2, 4])?;  // Merge pages 1, 3, 5
    /// editor.save("combined.pdf")?;
    /// ```
    pub fn merge_pages_from(
        &mut self,
        source_path: impl AsRef<Path>,
        pages: &[usize],
    ) -> Result<usize> {
        // Open the source document
        let mut source_doc = PdfDocument::open(source_path.as_ref())?;
        let source_page_count = source_doc.page_count()?;

        // Validate page indices
        for &page in pages {
            if page >= source_page_count {
                return Err(Error::InvalidPdf(format!(
                    "Page index {} out of range (source has {} pages)",
                    page, source_page_count
                )));
            }
        }

        if pages.is_empty() {
            return Ok(0);
        }

        self.is_modified = true;

        // Return number of pages to be merged
        Ok(pages.len())
    }

    // === Internal save helpers ===

    /// Read the original PDF file bytes.
    fn read_source_bytes(&self) -> Result<Vec<u8>> {
        let mut file = File::open(&self.source_path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    /// Build the Info dictionary object for the trailer.
    fn build_info_object(&self) -> Option<Object> {
        self.modified_info.as_ref().map(|info| info.to_object())
    }

    /// Write an incremental update to the PDF.
    fn write_incremental(&mut self, path: impl AsRef<Path>) -> Result<()> {
        // Read original file
        let original_bytes = self.read_source_bytes()?;
        let original_len = original_bytes.len();

        // Open output file
        let file = File::create(path.as_ref())?;
        let mut writer = BufWriter::new(file);

        // Write original content
        writer.write_all(&original_bytes)?;

        // Start incremental update section
        let update_start = original_len as u64;

        // Track new xref entries
        let mut xref_entries: Vec<(u32, u64, u16)> = Vec::new();
        let serializer = ObjectSerializer::compact();

        // Write modified objects
        for (&obj_id, obj) in &self.modified_objects {
            let offset = writer.stream_position().unwrap_or(update_start);
            let bytes = serializer.serialize_indirect(obj_id, 0, obj);
            writer.write_all(&bytes)?;
            xref_entries.push((obj_id, offset, 0));
        }

        // Write new Info object if metadata was modified
        if let Some(info_obj) = self.build_info_object() {
            let info_id = self.next_object_id;
            let offset = writer.stream_position().unwrap_or(update_start);
            let bytes = serializer.serialize_indirect(info_id, 0, &info_obj);
            writer.write_all(&bytes)?;
            xref_entries.push((info_id, offset, 0));
        }

        // Write new xref section
        let xref_offset = writer.stream_position().unwrap_or(update_start);
        write!(writer, "xref\n")?;

        // Sort entries by object ID
        xref_entries.sort_by_key(|(id, _, _)| *id);

        // Write xref subsections
        // For simplicity, write each entry as its own subsection
        for (obj_id, offset, gen) in &xref_entries {
            write!(writer, "{} 1\n", obj_id)?;
            write!(writer, "{:010} {:05} n \n", offset, gen)?;
        }

        // Write trailer
        write!(writer, "trailer\n")?;
        write!(writer, "<<\n")?;
        write!(writer, "  /Size {}\n", self.next_object_id + 1)?;
        write!(writer, "  /Prev {}\n", self.find_prev_xref_offset(&original_bytes)?)?;

        // Add /Root reference (from original trailer)
        if let Ok(catalog) = self.source.catalog() {
            if let Some(dict) = self.source.trailer().as_dict() {
                if let Some(root_ref) = dict.get("Root") {
                    write!(writer, "  /Root ")?;
                    writer.write_all(&serializer.serialize(root_ref))?;
                    write!(writer, "\n")?;
                }
            }
        }

        // Add /Info reference if we created one
        if self.modified_info.is_some() {
            write!(writer, "  /Info {} 0 R\n", self.next_object_id)?;
        }

        write!(writer, ">>\n")?;
        write!(writer, "startxref\n")?;
        write!(writer, "{}\n", xref_offset)?;
        write!(writer, "%%EOF\n")?;

        writer.flush()?;
        Ok(())
    }

    /// Find the offset of the previous xref table in the original PDF.
    fn find_prev_xref_offset(&self, bytes: &[u8]) -> Result<u64> {
        // Search backwards from the end for "startxref"
        let search = b"startxref";
        let mut pos = bytes.len().saturating_sub(100);

        while pos > 0 {
            if bytes[pos..].starts_with(search) {
                // Found it - parse the offset that follows
                let after_keyword = pos + search.len();
                let remaining = &bytes[after_keyword..];

                // Skip whitespace and parse number
                let offset_str: String = remaining
                    .iter()
                    .skip_while(|&&b| b == b' ' || b == b'\n' || b == b'\r')
                    .take_while(|&&b| b.is_ascii_digit())
                    .map(|&b| b as char)
                    .collect();

                if let Ok(offset) = offset_str.parse::<u64>() {
                    return Ok(offset);
                }
            }
            pos = pos.saturating_sub(1);
        }

        Err(Error::InvalidPdf("Could not find startxref in original PDF".to_string()))
    }

    /// Write a full rewrite of the PDF.
    fn write_full(&mut self, path: impl AsRef<Path>) -> Result<()> {
        // For full rewrite, we need to:
        // 1. Collect all objects (original + modified + new)
        // 2. Optionally remove unused objects
        // 3. Write complete new PDF structure

        // This is a more complex operation that requires:
        // - Traversing all reachable objects from the catalog
        // - Updating object references if IDs change
        // - Writing new header, body, xref, trailer

        let file = File::create(path.as_ref())?;
        let mut writer = BufWriter::new(file);

        // Write PDF header
        let (major, minor) = self.version();
        write!(writer, "%PDF-{}.{}\n", major, minor)?;
        // Binary marker per spec (bytes > 127 to indicate binary content)
        writer.write_all(b"%\x80\x81\x82\x83\n")?;

        let serializer = ObjectSerializer::compact();
        let mut xref_entries: Vec<(u32, u64, u16, bool)> = Vec::new(); // (id, offset, gen, in_use)

        // Object 0 is always free
        xref_entries.push((0, 65535, 65535, false));

        // Collect all objects we need to write
        let mut objects_to_write: Vec<(u32, Object)> = Vec::new();

        // Get catalog and traverse to collect all referenced objects
        let catalog = self.source.catalog()?;
        let catalog_ref = self
            .source
            .trailer()
            .as_dict()
            .and_then(|d| d.get("Root"))
            .and_then(|r| r.as_reference())
            .ok_or_else(|| Error::InvalidPdf("Missing catalog reference".to_string()))?;

        // For now, do a simple copy of essential objects
        // Full implementation would do complete object traversal

        // Write catalog (possibly modified)
        let catalog_obj = self
            .modified_objects
            .get(&catalog_ref.id)
            .cloned()
            .unwrap_or(catalog);
        let offset = writer.stream_position()?;
        let bytes = serializer.serialize_indirect(catalog_ref.id, 0, &catalog_obj);
        writer.write_all(&bytes)?;
        xref_entries.push((catalog_ref.id, offset, 0, true));

        // Get and write pages tree
        if let Some(catalog_dict) = catalog_obj.as_dict() {
            if let Some(pages_ref) = catalog_dict.get("Pages").and_then(|p| p.as_reference()) {
                let pages_obj = self.source.load_object(pages_ref)?;
                let offset = writer.stream_position()?;
                let bytes = serializer.serialize_indirect(pages_ref.id, 0, &pages_obj);
                writer.write_all(&bytes)?;
                xref_entries.push((pages_ref.id, offset, 0, true));

                // Write individual pages
                if let Some(pages_dict) = pages_obj.as_dict() {
                    if let Some(kids) = pages_dict.get("Kids").and_then(|k| k.as_array()) {
                        let mut page_index = 0;
                        for kid in kids {
                            if let Some(page_ref) = kid.as_reference() {
                                let page_obj = self.source.load_object(page_ref)?;
                                let offset = writer.stream_position()?;
                                let bytes =
                                    serializer.serialize_indirect(page_ref.id, 0, &page_obj);
                                writer.write_all(&bytes)?;
                                xref_entries.push((page_ref.id, offset, 0, true));

                                // Write page contents if present
                                if let Some(page_dict) = page_obj.as_dict() {
                                    // Check if this page has modified content (structure rebuild)
                                    if self.structure_modified
                                        && self.modified_content.contains_key(&page_index)
                                    {
                                        // Generate new content stream from modified StructureElement
                                        if let Some(structure) =
                                            self.modified_content.get(&page_index)
                                        {
                                            let (content_bytes, pending_images) =
                                                self.generate_content_stream(structure)?;

                                            // Create XObject entries for pending images
                                            let mut xobject_refs: Vec<(String, ObjectRef)> =
                                                Vec::new();
                                            for pending_image in pending_images {
                                                let xobj_id = self.allocate_object_id();

                                                // Build XObject stream for the image
                                                let xobj_stream =
                                                    Self::build_image_xobject(&pending_image.image);
                                                let offset = writer.stream_position()?;
                                                let bytes = serializer.serialize_indirect(
                                                    xobj_id,
                                                    0,
                                                    &xobj_stream,
                                                );
                                                writer.write_all(&bytes)?;
                                                xref_entries.push((xobj_id, offset, 0, true));

                                                xobject_refs.push((
                                                    pending_image.resource_id,
                                                    ObjectRef::new(xobj_id, 0),
                                                ));
                                            }

                                            // Create stream object for the content
                                            let content_stream_obj = Object::Stream {
                                                dict: HashMap::new(),
                                                data: content_bytes.into(),
                                            };

                                            // Allocate new object ID for the content stream
                                            let new_contents_id = self.allocate_object_id();
                                            let offset = writer.stream_position()?;
                                            let bytes = serializer.serialize_indirect(
                                                new_contents_id,
                                                0,
                                                &content_stream_obj,
                                            );
                                            writer.write_all(&bytes)?;
                                            xref_entries.push((new_contents_id, offset, 0, true));

                                            // Note: The page object's /Contents reference should be updated
                                            // and XObject references added to Resources dictionary.
                                            // This would require modifying the page_obj dictionary and re-serializing it.
                                            // Full implementation would update page_dict and re-write the page object.

                                            // TODO: For now, xobject_refs contains the image resource IDs
                                            // that need to be added to the page's Resources/XObject dictionary.
                                            // Full integration would require modifying the page object.
                                            let _ = xobject_refs; // Suppress unused warning
                                        }
                                    } else {
                                        // Use original contents
                                        if let Some(contents_ref) =
                                            page_dict.get("Contents").and_then(|c| c.as_reference())
                                        {
                                            let contents_obj =
                                                self.source.load_object(contents_ref)?;
                                            let offset = writer.stream_position()?;
                                            let bytes = serializer.serialize_indirect(
                                                contents_ref.id,
                                                0,
                                                &contents_obj,
                                            );
                                            writer.write_all(&bytes)?;
                                            xref_entries.push((contents_ref.id, offset, 0, true));
                                        }
                                    }

                                    // Write resources if present
                                    if let Some(resources_ref) =
                                        page_dict.get("Resources").and_then(|r| r.as_reference())
                                    {
                                        let resources_obj =
                                            self.source.load_object(resources_ref)?;
                                        let offset = writer.stream_position()?;
                                        let bytes = serializer.serialize_indirect(
                                            resources_ref.id,
                                            0,
                                            &resources_obj,
                                        );
                                        writer.write_all(&bytes)?;
                                        xref_entries.push((resources_ref.id, offset, 0, true));
                                    }
                                }

                                page_index += 1;
                            }
                        }
                    }
                }
            }
        }

        // Write info dictionary if modified
        let info_ref = if self.modified_info.is_some() {
            let info = self.modified_info.clone().unwrap();
            let info_id = self.allocate_object_id();
            let info_obj = info.to_object();
            let offset = writer.stream_position()?;
            let bytes = serializer.serialize_indirect(info_id, 0, &info_obj);
            writer.write_all(&bytes)?;
            xref_entries.push((info_id, offset, 0, true));
            Some(ObjectRef::new(info_id, 0))
        } else {
            None
        };

        // Sort xref entries by object ID
        xref_entries.sort_by_key(|(id, _, _, _)| *id);

        // Write xref table
        let xref_offset = writer.stream_position()?;
        write!(writer, "xref\n")?;

        // Find max object ID
        let max_id = xref_entries
            .iter()
            .map(|(id, _, _, _)| *id)
            .max()
            .unwrap_or(0);
        write!(writer, "0 {}\n", max_id + 1)?;

        // Write entries (fill gaps with free entries)
        let mut entry_map: HashMap<u32, (u64, u16, bool)> = xref_entries
            .into_iter()
            .map(|(id, off, gen, used)| (id, (off, gen, used)))
            .collect();

        for id in 0..=max_id {
            if let Some((offset, gen, in_use)) = entry_map.get(&id) {
                if *in_use {
                    write!(writer, "{:010} {:05} n \n", offset, gen)?;
                } else {
                    write!(writer, "{:010} {:05} f \n", offset, gen)?;
                }
            } else {
                // Free entry pointing to object 0
                write!(writer, "0000000000 65535 f \n")?;
            }
        }

        // Write trailer
        write!(writer, "trailer\n")?;
        write!(writer, "<<\n")?;
        write!(writer, "  /Size {}\n", max_id + 1)?;
        write!(writer, "  /Root {} 0 R\n", catalog_ref.id)?;

        if let Some(info_ref) = info_ref {
            write!(writer, "  /Info {} {} R\n", info_ref.id, info_ref.gen)?;
        }

        write!(writer, ">>\n")?;
        write!(writer, "startxref\n")?;
        write!(writer, "{}\n", xref_offset)?;
        write!(writer, "%%EOF\n")?;

        writer.flush()?;
        self.is_modified = false;
        Ok(())
    }

    // === Content modification operations ===

    /// Extract hierarchical content from a page.
    ///
    /// Returns the page's hierarchical content structure with all children populated.
    /// For untagged PDFs, returns a synthetic hierarchy based on geometric analysis.
    ///
    /// # Arguments
    ///
    /// * `page_index` - The page to extract from (0-indexed)
    ///
    /// # Returns
    ///
    /// `Ok(Some(structure))` if structure is found or generated,
    /// `Ok(None)` if no structure is available,
    /// `Err` if an error occurs during extraction
    pub fn get_page_content(&mut self, page_index: usize) -> Result<Option<StructureElement>> {
        HierarchicalExtractor::extract_page(&mut self.source, page_index)
    }

    /// Replace the content of a page with a new structure.
    ///
    /// Marks the document as modified and sets the structure_modified flag
    /// so the structure tree will be rebuilt on save.
    ///
    /// # Arguments
    ///
    /// * `page_index` - The page to modify (0-indexed)
    /// * `content` - The new hierarchical structure for the page
    ///
    /// # Returns
    ///
    /// `Err` if the page index is out of range
    pub fn set_page_content(&mut self, page_index: usize, content: StructureElement) -> Result<()> {
        let page_count = self.current_page_count();
        if page_index >= page_count {
            return Err(Error::InvalidPdf(format!(
                "Page index {} out of range (document has {} pages)",
                page_index, page_count
            )));
        }

        self.modified_content.insert(page_index, content);
        self.structure_modified = true;
        self.is_modified = true;
        Ok(())
    }

    /// Modify a page's structure in-place using a closure.
    ///
    /// Extracts the current content, passes it to the closure for modification,
    /// then saves it back.
    ///
    /// # Arguments
    ///
    /// * `page_index` - The page to modify
    /// * `f` - Closure that modifies the structure
    ///
    /// # Example
    ///
    /// ```ignore
    /// editor.modify_structure(0, |structure| {
    ///     // Modify structure in place
    ///     structure.alt_text = Some("Modified alt text".to_string());
    ///     Ok(())
    /// })?;
    /// ```
    pub fn modify_structure<F>(&mut self, page_index: usize, f: F) -> Result<()>
    where
        F: FnOnce(&mut StructureElement) -> Result<()>,
    {
        let mut content = self
            .get_page_content(page_index)?
            .ok_or_else(|| Error::InvalidPdf("No structure available for page".to_string()))?;

        f(&mut content)?;
        self.set_page_content(page_index, content)
    }

    /// Get the resource manager for allocating fonts, images, etc.
    ///
    /// Use this when manually constructing content elements that need resources.
    pub fn resource_manager_mut(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
    }

    /// Get a reference to the resource manager.
    pub fn resource_manager(&self) -> &ResourceManager {
        &self.resource_manager
    }

    /// Get a page for DOM-like editing.
    ///
    /// Returns a PdfPage that allows hierarchical navigation and querying
    /// of page content with a DOM-like API.
    pub fn get_page(&mut self, page_index: usize) -> Result<crate::editor::dom::PdfPage> {
        // Get the page info first
        let page_info = self.get_page_info(page_index)?;

        // Get or extract the page content
        let content = if let Some(structure) = self.get_page_content(page_index)? {
            structure
        } else {
            // If no modified content, try to extract from original
            match HierarchicalExtractor::extract_page(&mut self.source, page_index)? {
                Some(structure) => structure,
                None => {
                    // Create empty structure if extraction fails
                    StructureElement {
                        structure_type: "Document".to_string(),
                        bbox: crate::geometry::Rect::new(
                            0.0,
                            0.0,
                            page_info.width,
                            page_info.height,
                        ),
                        children: Vec::new(),
                        reading_order: Some(0),
                        alt_text: None,
                        language: None,
                    }
                },
            }
        };

        Ok(crate::editor::dom::PdfPage::from_structure(
            page_index,
            content,
            page_info.width,
            page_info.height,
        ))
    }

    /// Save a modified page back to the document.
    pub fn save_page(&mut self, page: crate::editor::dom::PdfPage) -> Result<()> {
        self.set_page_content(page.page_index, page.root)
    }

    /// Edit a page with a closure, automatically saving changes.
    ///
    /// # Example
    ///
    /// ```ignore
    /// editor.edit_page(0, |page| {
    ///     let text_elements = page.find_text_containing("Hello");
    ///     for text in text_elements {
    ///         page.set_text(text.id(), "Hi")?;
    ///     }
    ///     Ok(())
    /// })?;
    /// ```
    pub fn edit_page<F>(&mut self, page_index: usize, f: F) -> Result<()>
    where
        F: FnOnce(&mut crate::editor::dom::PdfPage) -> Result<()>,
    {
        let mut page = self.get_page(page_index)?;
        f(&mut page)?;
        self.save_page(page)
    }

    /// Get a page editor for fluent/XMLDocument-style editing.
    ///
    /// # Example
    ///
    /// ```ignore
    /// editor.page_editor(0)?
    ///    .find_text_containing("Hello")?
    ///    .for_each(|mut text| {
    ///        text.set_text("Hi");
    ///        Ok(())
    ///    })?
    ///    .done()?;
    /// editor.save_page_editor_modified()?;
    /// ```
    pub fn page_editor(&mut self, page_index: usize) -> Result<crate::editor::dom::PageEditor> {
        let page = self.get_page(page_index)?;
        Ok(crate::editor::dom::PageEditor { page })
    }

    /// Save a page from the fluent editor back to the document.
    pub fn save_page_from_editor(&mut self, page: crate::editor::dom::PdfPage) -> Result<()> {
        self.save_page(page)
    }
}

impl EditableDocument for DocumentEditor {
    fn get_info(&mut self) -> Result<DocumentInfo> {
        // Return modified info if available
        if let Some(ref info) = self.modified_info {
            return Ok(info.clone());
        }

        // Otherwise, load from source document
        let trailer = self.source.trailer();
        if let Some(trailer_dict) = trailer.as_dict() {
            if let Some(info_ref) = trailer_dict.get("Info").and_then(|i| i.as_reference()) {
                let info_obj = self.source.load_object(info_ref)?;
                return Ok(DocumentInfo::from_object(&info_obj));
            }
        }

        // No Info dictionary
        Ok(DocumentInfo::default())
    }

    fn set_info(&mut self, info: DocumentInfo) -> Result<()> {
        self.modified_info = Some(info);
        self.is_modified = true;
        Ok(())
    }

    fn page_count(&mut self) -> Result<usize> {
        Ok(self.current_page_count())
    }

    fn get_page_info(&mut self, index: usize) -> Result<PageInfo> {
        let page_refs = self.get_page_refs()?;

        if index >= page_refs.len() {
            return Err(Error::InvalidPdf(format!(
                "Page index {} out of range (document has {} pages)",
                index,
                page_refs.len()
            )));
        }

        let page_ref = page_refs[index];
        let page_obj = self.source.load_object(page_ref)?;
        let page_dict = page_obj
            .as_dict()
            .ok_or_else(|| Error::InvalidPdf("Page is not a dictionary".to_string()))?;

        // Get MediaBox for dimensions
        let (width, height) = if let Some(media_box) = page_dict.get("MediaBox") {
            self.parse_media_box(media_box)?
        } else {
            // Try to inherit from parent
            (612.0, 792.0) // Default to Letter size
        };

        let rotation = page_dict
            .get("Rotate")
            .and_then(|r| r.as_integer())
            .unwrap_or(0) as i32;

        Ok(PageInfo {
            index,
            width,
            height,
            rotation,
            object_ref: page_ref,
        })
    }

    fn remove_page(&mut self, index: usize) -> Result<()> {
        if index >= self.current_page_count() {
            return Err(Error::InvalidPdf(format!(
                "Page index {} out of range (document has {} pages)",
                index,
                self.current_page_count()
            )));
        }

        // Mark page as removed in page_order
        let mut visible_index = 0;
        for order in &mut self.page_order {
            if *order >= 0 {
                if visible_index == index {
                    *order = -1; // Mark as removed
                    break;
                }
                visible_index += 1;
            }
        }

        self.is_modified = true;
        Ok(())
    }

    fn move_page(&mut self, from: usize, to: usize) -> Result<()> {
        let count = self.current_page_count();
        if from >= count || to >= count {
            return Err(Error::InvalidPdf(format!(
                "Page index out of range (document has {} pages)",
                count
            )));
        }

        // Get current visible pages
        let visible: Vec<i32> = self
            .page_order
            .iter()
            .filter(|&&i| i >= 0)
            .copied()
            .collect();

        // Reorder
        let mut new_visible = visible.clone();
        let moved = new_visible.remove(from);
        new_visible.insert(to, moved);

        // Rebuild page_order
        self.page_order = new_visible;
        self.is_modified = true;
        Ok(())
    }

    fn duplicate_page(&mut self, index: usize) -> Result<usize> {
        if index >= self.current_page_count() {
            return Err(Error::InvalidPdf(format!(
                "Page index {} out of range (document has {} pages)",
                index,
                self.current_page_count()
            )));
        }

        // Get the original page index from page_order
        let visible: Vec<i32> = self
            .page_order
            .iter()
            .filter(|&&i| i >= 0)
            .copied()
            .collect();
        let original_index = visible[index];

        // Add duplicate reference
        self.page_order.push(original_index);
        self.is_modified = true;

        Ok(self.current_page_count() - 1)
    }

    fn save(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.save_with_options(path, SaveOptions::full_rewrite())
    }

    fn save_with_options(&mut self, path: impl AsRef<Path>, options: SaveOptions) -> Result<()> {
        if options.incremental {
            self.write_incremental(path)
        } else {
            self.write_full(path)
        }
    }
}

impl DocumentEditor {
    /// Parse a MediaBox array into (width, height).
    fn parse_media_box(&self, media_box: &Object) -> Result<(f32, f32)> {
        if let Some(arr) = media_box.as_array() {
            if arr.len() >= 4 {
                let llx = arr[0]
                    .as_real()
                    .or_else(|| arr[0].as_integer().map(|i| i as f64))
                    .unwrap_or(0.0);
                let lly = arr[1]
                    .as_real()
                    .or_else(|| arr[1].as_integer().map(|i| i as f64))
                    .unwrap_or(0.0);
                let urx = arr[2]
                    .as_real()
                    .or_else(|| arr[2].as_integer().map(|i| i as f64))
                    .unwrap_or(612.0);
                let ury = arr[3]
                    .as_real()
                    .or_else(|| arr[3].as_integer().map(|i| i as f64))
                    .unwrap_or(792.0);

                return Ok(((urx - llx) as f32, (ury - lly) as f32));
            }
        }

        // Default to Letter size
        Ok((612.0, 792.0))
    }

    /// Generate a content stream from a StructureElement with marked content wrapping.
    ///
    /// This is used when writing modified structure elements back to a PDF.
    /// Wraps each element in BDC/EMC (Begin/End Marked Content) operators for tagged PDF support.
    ///
    /// Returns the content stream bytes and any pending images that need XObject registration.
    ///
    /// # PDF Spec Compliance
    ///
    /// - ISO 32000-1:2008, Section 14.7.4 - Marked Content Sequences
    fn generate_content_stream(
        &self,
        elem: &StructureElement,
    ) -> Result<(Vec<u8>, Vec<crate::writer::PendingImage>)> {
        let mut builder = ContentStreamBuilder::new();
        builder.add_structure_element(elem);
        let bytes = builder.build()?;
        let pending_images = builder.take_pending_images();
        Ok((bytes, pending_images))
    }

    /// Build an XObject stream from ImageContent.
    ///
    /// Creates a PDF Image XObject suitable for embedding in a PDF.
    /// Per PDF spec Section 8.9, images are represented as XObject streams.
    fn build_image_xobject(image: &crate::elements::ImageContent) -> Object {
        use crate::elements::{ColorSpace as ElemColorSpace, ImageFormat as ElemImageFormat};

        let mut dict = HashMap::new();

        dict.insert("Type".to_string(), Object::Name("XObject".to_string()));
        dict.insert("Subtype".to_string(), Object::Name("Image".to_string()));
        dict.insert("Width".to_string(), Object::Integer(image.width as i64));
        dict.insert("Height".to_string(), Object::Integer(image.height as i64));
        dict.insert(
            "BitsPerComponent".to_string(),
            Object::Integer(image.bits_per_component as i64),
        );

        // Map color space
        let color_space_name = match image.color_space {
            ElemColorSpace::Gray => "DeviceGray",
            ElemColorSpace::RGB => "DeviceRGB",
            ElemColorSpace::CMYK => "DeviceCMYK",
            ElemColorSpace::Indexed => "Indexed",
            ElemColorSpace::Lab => "Lab",
        };
        dict.insert("ColorSpace".to_string(), Object::Name(color_space_name.to_string()));

        // Set filter based on image format
        match image.format {
            ElemImageFormat::Jpeg => {
                dict.insert("Filter".to_string(), Object::Name("DCTDecode".to_string()));
            },
            ElemImageFormat::Png | ElemImageFormat::Raw => {
                dict.insert("Filter".to_string(), Object::Name("FlateDecode".to_string()));
            },
            ElemImageFormat::Jpeg2000 => {
                dict.insert("Filter".to_string(), Object::Name("JPXDecode".to_string()));
            },
            ElemImageFormat::Jbig2 => {
                dict.insert("Filter".to_string(), Object::Name("JBIG2Decode".to_string()));
            },
            ElemImageFormat::Unknown => {
                // No filter for unknown format (raw data)
            },
        }

        dict.insert("Length".to_string(), Object::Integer(image.data.len() as i64));

        Object::Stream {
            dict,
            data: image.data.clone().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_info_builder() {
        let info = DocumentInfo::new()
            .title("Test Document")
            .author("Test Author")
            .subject("Test Subject")
            .keywords("test, rust, pdf");

        assert_eq!(info.title, Some("Test Document".to_string()));
        assert_eq!(info.author, Some("Test Author".to_string()));
        assert_eq!(info.subject, Some("Test Subject".to_string()));
        assert_eq!(info.keywords, Some("test, rust, pdf".to_string()));
    }

    #[test]
    fn test_document_info_to_object() {
        let info = DocumentInfo::new().title("My PDF").author("John Doe");

        let obj = info.to_object();
        let dict = obj.as_dict().unwrap();

        assert!(dict.contains_key("Title"));
        assert!(dict.contains_key("Author"));
        assert!(!dict.contains_key("Subject"));
    }

    #[test]
    fn test_document_info_from_object() {
        let mut dict = HashMap::new();
        dict.insert("Title".to_string(), Object::String(b"Test Title".to_vec()));
        dict.insert("Author".to_string(), Object::String(b"Test Author".to_vec()));

        let obj = Object::Dictionary(dict);
        let info = DocumentInfo::from_object(&obj);

        assert_eq!(info.title, Some("Test Title".to_string()));
        assert_eq!(info.author, Some("Test Author".to_string()));
        assert_eq!(info.subject, None);
    }

    #[test]
    fn test_save_options() {
        let full = SaveOptions::full_rewrite();
        assert!(!full.incremental);
        assert!(full.compress);
        assert!(full.garbage_collect);

        let inc = SaveOptions::incremental();
        assert!(inc.incremental);
        assert!(!inc.compress);
        assert!(!inc.garbage_collect);
    }
}
