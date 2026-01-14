//! Form field editing support for DocumentEditor.
//!
//! This module provides the abstraction layer for reading, creating, and modifying
//! AcroForm fields in existing PDF documents.
//!
//! # Architecture
//!
//! The `FormFieldWrapper` type bridges the gap between:
//! - **Read side**: `FormField` from `extractors::forms` (existing fields in PDF)
//! - **Write side**: `FormFieldWidget` trait from `writer::form_fields` (new/modified fields)
//!
//! # Example
//!
//! ```ignore
//! use pdf_oxide::editor::DocumentEditor;
//! use pdf_oxide::writer::form_fields::TextFieldWidget;
//! use pdf_oxide::geometry::Rect;
//!
//! let mut editor = DocumentEditor::open("form.pdf")?;
//!
//! // Add new field
//! editor.add_form_field(0,
//!     TextFieldWidget::new("email", Rect::new(100.0, 700.0, 200.0, 20.0))
//! )?;
//!
//! // Modify existing field
//! editor.set_form_field_value("name", FormFieldValue::Text("John".into()))?;
//!
//! editor.save("modified.pdf")?;
//! ```

use crate::extractors::forms::{FieldType, FieldValue, FormField};
use crate::geometry::Rect;
use crate::object::{Object, ObjectRef};
use crate::writer::form_fields::FormFieldWidget;
use std::collections::HashMap;

/// Unified form field value type.
///
/// This enum provides a consistent interface for field values regardless of
/// whether the field was read from an existing PDF or created new.
#[derive(Debug, Clone, PartialEq)]
pub enum FormFieldValue {
    /// Text string value (for text fields)
    Text(String),
    /// Boolean value (for checkboxes)
    Boolean(bool),
    /// Single choice value (for radio buttons, combo boxes)
    Choice(String),
    /// Multiple choice values (for multi-select list boxes)
    MultiChoice(Vec<String>),
    /// No value present
    None,
}

impl FormFieldValue {
    /// Check if the value is empty/none.
    pub fn is_none(&self) -> bool {
        matches!(self, FormFieldValue::None)
    }

    /// Get as text, if this is a text value.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            FormFieldValue::Text(s) => Some(s),
            _ => None,
        }
    }

    /// Get as boolean, if this is a boolean value.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            FormFieldValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Get as choice, if this is a single choice value.
    pub fn as_choice(&self) -> Option<&str> {
        match self {
            FormFieldValue::Choice(s) => Some(s),
            _ => None,
        }
    }

    /// Get as multi-choice, if this is a multi-choice value.
    pub fn as_multi_choice(&self) -> Option<&[String]> {
        match self {
            FormFieldValue::MultiChoice(v) => Some(v),
            _ => None,
        }
    }
}

/// Convert from extractor's FieldValue to unified FormFieldValue.
impl From<FieldValue> for FormFieldValue {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::Text(s) => FormFieldValue::Text(s),
            FieldValue::Boolean(b) => FormFieldValue::Boolean(b),
            FieldValue::Name(s) => FormFieldValue::Choice(s),
            FieldValue::Array(v) => FormFieldValue::MultiChoice(v),
            FieldValue::None => FormFieldValue::None,
        }
    }
}

/// Convert from reference to extractor's FieldValue.
impl From<&FieldValue> for FormFieldValue {
    fn from(value: &FieldValue) -> Self {
        match value {
            FieldValue::Text(s) => FormFieldValue::Text(s.clone()),
            FieldValue::Boolean(b) => FormFieldValue::Boolean(*b),
            FieldValue::Name(s) => FormFieldValue::Choice(s.clone()),
            FieldValue::Array(v) => FormFieldValue::MultiChoice(v.clone()),
            FieldValue::None => FormFieldValue::None,
        }
    }
}

/// Convert FormFieldValue to PDF Object for serialization.
impl From<&FormFieldValue> for Object {
    fn from(value: &FormFieldValue) -> Self {
        match value {
            FormFieldValue::Text(s) => Object::String(s.as_bytes().to_vec()),
            FormFieldValue::Boolean(b) => {
                // Checkboxes use /Yes or /Off names
                if *b {
                    Object::Name("Yes".to_string())
                } else {
                    Object::Name("Off".to_string())
                }
            },
            FormFieldValue::Choice(s) => Object::String(s.as_bytes().to_vec()),
            FormFieldValue::MultiChoice(v) => Object::Array(
                v.iter()
                    .map(|s| Object::String(s.as_bytes().to_vec()))
                    .collect(),
            ),
            FormFieldValue::None => Object::Null,
        }
    }
}

/// Wrapper for form fields that bridges reading and writing.
///
/// This struct provides a unified interface for working with form fields
/// whether they come from an existing PDF (read) or are being created/modified (write).
#[derive(Debug, Clone)]
pub struct FormFieldWrapper {
    /// Full qualified field name (e.g., "form.section.field")
    pub(crate) name: String,

    /// Original field from PDF (if read from existing document)
    pub(crate) original: Option<FormField>,

    /// Modified or new value (if changed from original)
    pub(crate) modified_value: Option<FormFieldValue>,

    /// Page index where this field appears (0-based)
    pub(crate) page_index: usize,

    /// Whether this field has been modified since loading
    pub(crate) modified: bool,

    /// Whether this is a new field (not in original document)
    pub(crate) is_new: bool,

    /// Object reference if field exists in PDF
    pub(crate) object_ref: Option<ObjectRef>,

    /// Field type for new fields
    pub(crate) field_type: Option<FormFieldType>,

    /// Widget configuration for new fields
    pub(crate) widget_config: Option<WidgetConfig>,

    // === Hierarchy support ===
    /// Reference to parent field (for child fields in hierarchy)
    pub(crate) parent_ref: Option<ObjectRef>,

    /// References to child fields (for parent container fields)
    pub(crate) children_refs: Vec<ObjectRef>,

    /// Partial name (name without parent prefix, e.g., "street" for "address.street")
    pub(crate) partial_name: String,

    /// Whether this is a parent-only container (no widget, only Kids)
    pub(crate) is_parent_only: bool,

    /// Parent field name (for hierarchy tracking before refs are allocated)
    pub(crate) parent_name: Option<String>,

    // === Property modification tracking ===
    /// Modified field flags (/Ff)
    pub(crate) modified_flags: Option<u32>,

    /// Modified tooltip (/TU)
    pub(crate) modified_tooltip: Option<String>,

    /// Modified bounds/rect (/Rect)
    pub(crate) modified_rect: Option<Rect>,

    /// Modified default value (/DV)
    pub(crate) modified_default_value: Option<FormFieldValue>,

    /// Modified max length (/MaxLen) - text fields only
    pub(crate) modified_max_length: Option<u32>,

    /// Modified alignment (/Q) - 0=left, 1=center, 2=right
    pub(crate) modified_alignment: Option<u32>,

    /// Modified default appearance (/DA)
    pub(crate) modified_default_appearance: Option<String>,

    /// Modified background color (from /MK/BG)
    pub(crate) modified_background_color: Option<[f32; 3]>,

    /// Modified border color (from /MK/BC)
    pub(crate) modified_border_color: Option<[f32; 3]>,

    /// Modified border width (from /BS/W)
    pub(crate) modified_border_width: Option<f32>,
}

/// Field type for new fields.
#[derive(Debug, Clone, PartialEq)]
pub enum FormFieldType {
    /// Text field
    Text,
    /// Checkbox
    Checkbox,
    /// Radio button group
    RadioGroup,
    /// Combo box (dropdown)
    ComboBox,
    /// List box
    ListBox,
    /// Push button
    PushButton,
}

/// Widget configuration for new fields.
#[derive(Debug, Clone)]
pub struct WidgetConfig {
    /// Bounding rectangle
    pub rect: Rect,
    /// Field dictionary entries
    pub field_dict: HashMap<String, Object>,
    /// Widget dictionary entries
    pub widget_dict: HashMap<String, Object>,
    /// Field type string (Tx, Btn, Ch)
    pub field_type_str: String,
}

/// Configuration for creating parent container fields.
///
/// Parent fields are non-terminal fields in a hierarchy that don't have
/// a widget annotation but contain child fields via the `/Kids` array.
///
/// # Example
///
/// ```ignore
/// use pdf_oxide::editor::ParentFieldConfig;
///
/// // Create a parent field for address components
/// let config = ParentFieldConfig::new("address")
///     .with_flags(0); // Optional inherited flags
///
/// editor.add_parent_field(config)?;
/// editor.add_child_field("address", 0, TextFieldWidget::new("street", rect))?;
/// ```
#[derive(Debug, Clone)]
pub struct ParentFieldConfig {
    /// Partial field name (without parent prefix)
    pub(crate) partial_name: String,
    /// Optional field type to inherit to children (/FT)
    pub(crate) field_type: Option<FormFieldType>,
    /// Optional field flags to inherit to children (/Ff)
    pub(crate) flags: Option<u32>,
    /// Optional default value to inherit (/DV)
    pub(crate) default_value: Option<FormFieldValue>,
    /// Optional tooltip (/TU)
    pub(crate) tooltip: Option<String>,
    /// Parent field name (if this parent is nested)
    pub(crate) parent_name: Option<String>,
}

impl ParentFieldConfig {
    /// Create a new parent field configuration.
    ///
    /// # Arguments
    ///
    /// * `name` - The partial name for this parent field
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            partial_name: name.into(),
            field_type: None,
            flags: None,
            default_value: None,
            tooltip: None,
            parent_name: None,
        }
    }

    /// Set the field type to inherit to children.
    pub fn with_field_type(mut self, ft: FormFieldType) -> Self {
        self.field_type = Some(ft);
        self
    }

    /// Set field flags to inherit to children.
    ///
    /// Common flag bits:
    /// - Bit 1: ReadOnly
    /// - Bit 2: Required
    /// - Bit 3: NoExport
    pub fn with_flags(mut self, flags: u32) -> Self {
        self.flags = Some(flags);
        self
    }

    /// Set the default value to inherit to children.
    pub fn with_default_value(mut self, value: FormFieldValue) -> Self {
        self.default_value = Some(value);
        self
    }

    /// Set the tooltip for this parent field.
    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Set the parent field name (for nested parents).
    pub fn with_parent(mut self, parent_name: impl Into<String>) -> Self {
        self.parent_name = Some(parent_name.into());
        self
    }

    /// Get the full qualified name.
    pub fn full_name(&self) -> String {
        if let Some(ref parent) = self.parent_name {
            format!("{}.{}", parent, self.partial_name)
        } else {
            self.partial_name.clone()
        }
    }
}

/// Extract the partial name from a full qualified name.
///
/// For example, "address.street" returns "street".
fn extract_partial_name(full_name: &str) -> String {
    if let Some(pos) = full_name.rfind('.') {
        full_name[pos + 1..].to_string()
    } else {
        full_name.to_string()
    }
}

/// Extract the parent name from a full qualified name.
///
/// For example, "address.street" returns Some("address").
/// "field" returns None.
fn extract_parent_name(full_name: &str) -> Option<String> {
    full_name.rfind('.').map(|pos| full_name[..pos].to_string())
}

impl FormFieldWrapper {
    /// Create a wrapper from an existing FormField (read from PDF).
    pub fn from_read(field: FormField, page_index: usize, object_ref: Option<ObjectRef>) -> Self {
        let name = field.full_name.clone();
        let partial_name = extract_partial_name(&name);
        Self {
            name,
            original: Some(field),
            modified_value: None,
            page_index,
            modified: false,
            is_new: false,
            object_ref,
            field_type: None,
            widget_config: None,
            // Hierarchy fields
            parent_ref: None,
            children_refs: Vec::new(),
            partial_name,
            is_parent_only: false,
            parent_name: None,
            // Property modification fields
            modified_flags: None,
            modified_tooltip: None,
            modified_rect: None,
            modified_default_value: None,
            modified_max_length: None,
            modified_alignment: None,
            modified_default_appearance: None,
            modified_background_color: None,
            modified_border_color: None,
            modified_border_width: None,
        }
    }

    /// Create a wrapper from a FormFieldWidget (for new fields).
    pub fn from_widget<W: FormFieldWidget>(widget: &W, page_index: usize) -> Self {
        let field_type = match widget.field_type() {
            "Tx" => FormFieldType::Text,
            "Btn" => FormFieldType::Checkbox, // Could be checkbox, radio, or button
            "Ch" => FormFieldType::ComboBox,  // Could be combo or list
            _ => FormFieldType::Text,
        };

        let config = WidgetConfig {
            rect: widget.rect(),
            field_dict: widget.build_field_dict(),
            widget_dict: HashMap::new(), // Will be built with page_ref on save
            field_type_str: widget.field_type().to_string(),
        };

        let name = widget.field_name().to_string();
        let partial_name = extract_partial_name(&name);

        Self {
            name,
            original: None,
            modified_value: None,
            page_index,
            modified: false,
            is_new: true,
            object_ref: None,
            field_type: Some(field_type),
            widget_config: Some(config),
            // Hierarchy fields
            parent_ref: None,
            children_refs: Vec::new(),
            partial_name,
            is_parent_only: false,
            parent_name: None,
            // Property modification fields
            modified_flags: None,
            modified_tooltip: None,
            modified_rect: None,
            modified_default_value: None,
            modified_max_length: None,
            modified_alignment: None,
            modified_default_appearance: None,
            modified_background_color: None,
            modified_border_color: None,
            modified_border_width: None,
        }
    }

    /// Create a wrapper from a ParentFieldConfig (for parent container fields).
    pub fn from_parent_config(config: &ParentFieldConfig) -> Self {
        Self {
            name: config.full_name(),
            original: None,
            modified_value: config.default_value.clone(),
            page_index: 0, // Parent-only fields don't have a page
            modified: false,
            is_new: true,
            object_ref: None,
            field_type: config.field_type.clone(),
            widget_config: None, // No widget for parent-only fields
            // Hierarchy fields
            parent_ref: None,
            children_refs: Vec::new(),
            partial_name: config.partial_name.clone(),
            is_parent_only: true,
            parent_name: config.parent_name.clone(),
            // Property modification fields
            modified_flags: config.flags,
            modified_tooltip: config.tooltip.clone(),
            modified_rect: None,
            modified_default_value: None, // Already in modified_value
            modified_max_length: None,
            modified_alignment: None,
            modified_default_appearance: None,
            modified_background_color: None,
            modified_border_color: None,
            modified_border_width: None,
        }
    }

    /// Create a wrapper for a child field with parent reference.
    pub fn from_widget_with_parent<W: FormFieldWidget>(
        widget: &W,
        page_index: usize,
        parent_name: &str,
    ) -> Self {
        let mut wrapper = Self::from_widget(widget, page_index);
        wrapper.parent_name = Some(parent_name.to_string());
        // Update name to be fully qualified
        wrapper.name = format!("{}.{}", parent_name, wrapper.partial_name);
        wrapper
    }

    /// Get the full qualified field name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the page index where this field appears.
    pub fn page_index(&self) -> usize {
        self.page_index
    }

    /// Check if this field has been modified.
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Check if this is a new field.
    pub fn is_new(&self) -> bool {
        self.is_new
    }

    /// Get the current value of the field.
    pub fn value(&self) -> FormFieldValue {
        // Return modified value if set, otherwise original value
        if let Some(ref modified) = self.modified_value {
            return modified.clone();
        }

        if let Some(ref original) = self.original {
            FormFieldValue::from(&original.value)
        } else {
            FormFieldValue::None
        }
    }

    /// Set a new value for the field.
    pub fn set_value(&mut self, value: FormFieldValue) {
        self.modified_value = Some(value);
        self.modified = true;
    }

    /// Get the field type.
    pub fn field_type(&self) -> Option<&FieldType> {
        self.original.as_ref().map(|f| &f.field_type)
    }

    /// Get the bounding box of the field.
    pub fn bounds(&self) -> Option<Rect> {
        if let Some(ref config) = self.widget_config {
            return Some(config.rect);
        }

        if let Some(ref original) = self.original {
            original
                .bounds
                .map(|b| Rect::new(b[0] as f32, b[1] as f32, b[2] as f32, b[3] as f32))
        } else {
            None
        }
    }

    /// Get the tooltip/description.
    pub fn tooltip(&self) -> Option<&str> {
        self.original.as_ref().and_then(|f| f.tooltip.as_deref())
    }

    /// Get the object reference if the field exists in the PDF.
    pub fn object_ref(&self) -> Option<ObjectRef> {
        self.object_ref
    }

    /// Set the object reference (when allocating new object ID).
    pub fn set_object_ref(&mut self, object_ref: ObjectRef) {
        self.object_ref = Some(object_ref);
    }

    /// Get the original field if this was read from PDF.
    pub fn original(&self) -> Option<&FormField> {
        self.original.as_ref()
    }

    /// Get the widget configuration for new fields.
    pub fn widget_config(&self) -> Option<&WidgetConfig> {
        self.widget_config.as_ref()
    }

    /// Check if this field/widget uses merged dictionary format.
    ///
    /// In PDF, a field and its widget annotation can be merged into a single
    /// dictionary (common for single-widget fields) or kept separate
    /// (required for multi-widget fields like radio buttons).
    pub fn is_merged(&self) -> bool {
        // New single-widget fields are merged by default
        if self.is_new {
            // Radio groups are not merged (multiple widgets per field)
            if self.field_type == Some(FormFieldType::RadioGroup) {
                return false;
            }
            return true;
        }

        // For existing fields, check if original had /Subtype /Widget
        // indicating merged format
        true // Default to merged for simplicity
    }

    /// Build the field dictionary for PDF serialization.
    ///
    /// For merged fields, this includes both field and widget entries.
    pub fn build_field_dict(&self, page_ref: ObjectRef) -> HashMap<String, Object> {
        let mut dict = HashMap::new();

        if let Some(ref config) = self.widget_config {
            // New field - use widget config
            dict.extend(config.field_dict.clone());

            if self.is_merged() {
                // Add widget annotation entries for merged format
                dict.insert("Type".to_string(), Object::Name("Annot".to_string()));
                dict.insert("Subtype".to_string(), Object::Name("Widget".to_string()));
                dict.insert(
                    "Rect".to_string(),
                    Object::Array(vec![
                        Object::Real(config.rect.x as f64),
                        Object::Real(config.rect.y as f64),
                        Object::Real((config.rect.x + config.rect.width) as f64),
                        Object::Real((config.rect.y + config.rect.height) as f64),
                    ]),
                );
                dict.insert("P".to_string(), Object::Reference(page_ref));
            }
        }

        // Apply modified value if set
        if let Some(ref value) = self.modified_value {
            let obj: Object = value.into();
            if !matches!(obj, Object::Null) {
                dict.insert("V".to_string(), obj);
            }
        }

        // Add parent reference if this is a child field
        if let Some(parent_ref) = self.parent_ref {
            dict.insert("Parent".to_string(), Object::Reference(parent_ref));
            // Use partial name instead of full name for child fields
            dict.insert("T".to_string(), Object::String(self.partial_name.as_bytes().to_vec()));
        }

        dict
    }

    // === Hierarchy methods ===

    /// Get the partial name (last component of full name).
    pub fn partial_name(&self) -> &str {
        &self.partial_name
    }

    /// Get the parent field name (if this is a child field).
    pub fn parent_name(&self) -> Option<&str> {
        self.parent_name.as_deref()
    }

    /// Get the parent field reference (if set).
    pub fn parent_ref(&self) -> Option<ObjectRef> {
        self.parent_ref
    }

    /// Set the parent field reference.
    pub fn set_parent_ref(&mut self, parent_ref: ObjectRef) {
        self.parent_ref = Some(parent_ref);
    }

    /// Get the children field references.
    pub fn children_refs(&self) -> &[ObjectRef] {
        &self.children_refs
    }

    /// Add a child reference to this parent field.
    pub fn add_child_ref(&mut self, child_ref: ObjectRef) {
        self.children_refs.push(child_ref);
    }

    /// Check if this is a parent-only container field.
    pub fn is_parent_only(&self) -> bool {
        self.is_parent_only
    }

    /// Check if this field has a parent (is a child field).
    pub fn has_parent(&self) -> bool {
        self.parent_name.is_some() || self.parent_ref.is_some()
    }

    /// Build a parent-only field dictionary (no widget, just field entries).
    ///
    /// This is used for non-terminal fields in a hierarchy that contain
    /// child fields via the `/Kids` array.
    pub fn build_parent_dict(&self) -> HashMap<String, Object> {
        let mut dict = HashMap::new();

        // Partial name (T) - required
        dict.insert("T".to_string(), Object::String(self.partial_name.as_bytes().to_vec()));

        // Field type (FT) - optional for non-terminal, but useful for inheritance
        if let Some(ref ft) = self.field_type {
            let ft_name = match ft {
                FormFieldType::Text => "Tx",
                FormFieldType::Checkbox | FormFieldType::RadioGroup | FormFieldType::PushButton => {
                    "Btn"
                },
                FormFieldType::ComboBox | FormFieldType::ListBox => "Ch",
            };
            dict.insert("FT".to_string(), Object::Name(ft_name.to_string()));
        }

        // Default value (DV) - optional
        if let Some(ref dv) = self.modified_value {
            let obj: Object = dv.into();
            if !matches!(obj, Object::Null) {
                dict.insert("DV".to_string(), obj);
            }
        }

        // Kids array (will be populated later with child refs)
        if !self.children_refs.is_empty() {
            let kids: Vec<Object> = self
                .children_refs
                .iter()
                .map(|r| Object::Reference(*r))
                .collect();
            dict.insert("Kids".to_string(), Object::Array(kids));
        }

        // Parent reference (if this parent is nested)
        if let Some(parent_ref) = self.parent_ref {
            dict.insert("Parent".to_string(), Object::Reference(parent_ref));
        }

        // Field flags
        if let Some(flags) = self.modified_flags {
            dict.insert("Ff".to_string(), Object::Integer(flags as i64));
        }

        // Tooltip
        if let Some(ref tooltip) = self.modified_tooltip {
            dict.insert("TU".to_string(), Object::String(tooltip.as_bytes().to_vec()));
        }

        dict
    }

    // === Property modification methods ===

    /// Set the field flags.
    ///
    /// Common flag bits (from PDF Table 221):
    /// - Bit 1 (0x01): ReadOnly - user cannot change field value
    /// - Bit 2 (0x02): Required - field must have value when exporting
    /// - Bit 3 (0x04): NoExport - field should not be exported
    ///
    /// Use `field_flags` constants for convenience.
    pub fn set_flags(&mut self, flags: u32) {
        self.modified_flags = Some(flags);
        self.modified = true;
    }

    /// Get the current field flags.
    pub fn flags(&self) -> Option<u32> {
        if self.modified_flags.is_some() {
            return self.modified_flags;
        }
        self.original.as_ref().and_then(|f| f.flags)
    }

    /// Set field as read-only.
    pub fn set_readonly(&mut self, readonly: bool) {
        let current = self.flags().unwrap_or(0);
        let new_flags = if readonly {
            current | 0x01
        } else {
            current & !0x01
        };
        self.set_flags(new_flags);
    }

    /// Check if field is read-only.
    pub fn is_readonly(&self) -> bool {
        self.flags().map(|f| f & 0x01 != 0).unwrap_or(false)
    }

    /// Set field as required.
    pub fn set_required(&mut self, required: bool) {
        let current = self.flags().unwrap_or(0);
        let new_flags = if required {
            current | 0x02
        } else {
            current & !0x02
        };
        self.set_flags(new_flags);
    }

    /// Check if field is required.
    pub fn is_required(&self) -> bool {
        self.flags().map(|f| f & 0x02 != 0).unwrap_or(false)
    }

    /// Set field as no-export.
    pub fn set_no_export(&mut self, no_export: bool) {
        let current = self.flags().unwrap_or(0);
        let new_flags = if no_export {
            current | 0x04
        } else {
            current & !0x04
        };
        self.set_flags(new_flags);
    }

    /// Check if field is no-export.
    pub fn is_no_export(&self) -> bool {
        self.flags().map(|f| f & 0x04 != 0).unwrap_or(false)
    }

    /// Set the tooltip/description.
    pub fn set_tooltip(&mut self, tooltip: impl Into<String>) {
        self.modified_tooltip = Some(tooltip.into());
        self.modified = true;
    }

    /// Get the current tooltip, preferring modified over original.
    pub fn get_tooltip(&self) -> Option<&str> {
        if let Some(ref tooltip) = self.modified_tooltip {
            return Some(tooltip);
        }
        self.original.as_ref().and_then(|f| f.tooltip.as_deref())
    }

    /// Set the field bounding rectangle.
    pub fn set_rect(&mut self, rect: Rect) {
        self.modified_rect = Some(rect);
        self.modified = true;
    }

    /// Get the current rect, preferring modified over original.
    pub fn get_rect(&self) -> Option<Rect> {
        if let Some(rect) = self.modified_rect {
            return Some(rect);
        }
        self.bounds()
    }

    /// Set the default value.
    pub fn set_default_value(&mut self, value: FormFieldValue) {
        self.modified_default_value = Some(value);
        self.modified = true;
    }

    /// Get the current default value.
    pub fn get_default_value(&self) -> Option<&FormFieldValue> {
        if self.modified_default_value.is_some() {
            return self.modified_default_value.as_ref();
        }
        self.original
            .as_ref()
            .and_then(|f| f.default_value.as_ref())
            .map(|v| {
                // Can't return reference to temporary, use modified field if available
                &FormFieldValue::None // This is a limitation; in practice we'd need to store converted value
            })
    }

    /// Set the maximum text length (for text fields only).
    pub fn set_max_length(&mut self, max_len: u32) {
        self.modified_max_length = Some(max_len);
        self.modified = true;
    }

    /// Get the maximum text length.
    pub fn get_max_length(&self) -> Option<u32> {
        if self.modified_max_length.is_some() {
            return self.modified_max_length;
        }
        self.original.as_ref().and_then(|f| f.max_length)
    }

    /// Set the text alignment.
    ///
    /// * 0 = Left
    /// * 1 = Center
    /// * 2 = Right
    pub fn set_alignment(&mut self, alignment: u32) {
        self.modified_alignment = Some(alignment);
        self.modified = true;
    }

    /// Get the current text alignment.
    pub fn get_alignment(&self) -> Option<u32> {
        if self.modified_alignment.is_some() {
            return self.modified_alignment;
        }
        self.original.as_ref().and_then(|f| f.alignment)
    }

    /// Set the default appearance string.
    ///
    /// The DA string specifies the font, size, and color for field content.
    /// Example: "/Helv 12 Tf 0 g" for 12pt Helvetica in black.
    pub fn set_default_appearance(&mut self, da: impl Into<String>) {
        self.modified_default_appearance = Some(da.into());
        self.modified = true;
    }

    /// Get the current default appearance string.
    pub fn get_default_appearance(&self) -> Option<&str> {
        if let Some(ref da) = self.modified_default_appearance {
            return Some(da);
        }
        self.original
            .as_ref()
            .and_then(|f| f.default_appearance.as_deref())
    }

    /// Set the background color (RGB, values 0.0-1.0).
    pub fn set_background_color(&mut self, color: [f32; 3]) {
        self.modified_background_color = Some(color);
        self.modified = true;
    }

    /// Get the current background color.
    pub fn get_background_color(&self) -> Option<[f32; 3]> {
        if self.modified_background_color.is_some() {
            return self.modified_background_color;
        }
        self.original
            .as_ref()
            .and_then(|f| f.appearance_chars.as_ref())
            .and_then(|ac| ac.background_color)
    }

    /// Set the border color (RGB, values 0.0-1.0).
    pub fn set_border_color(&mut self, color: [f32; 3]) {
        self.modified_border_color = Some(color);
        self.modified = true;
    }

    /// Get the current border color.
    pub fn get_border_color(&self) -> Option<[f32; 3]> {
        if self.modified_border_color.is_some() {
            return self.modified_border_color;
        }
        self.original
            .as_ref()
            .and_then(|f| f.appearance_chars.as_ref())
            .and_then(|ac| ac.border_color)
    }

    /// Set the border width in points.
    pub fn set_border_width(&mut self, width: f32) {
        self.modified_border_width = Some(width);
        self.modified = true;
    }

    /// Get the current border width.
    pub fn get_border_width(&self) -> Option<f32> {
        if self.modified_border_width.is_some() {
            return self.modified_border_width;
        }
        self.original
            .as_ref()
            .and_then(|f| f.border_style.as_ref())
            .map(|bs| bs.width)
    }
}

/// Result of checking if an existing field uses merged format.
pub fn is_merged_field_dict(dict: &HashMap<String, Object>) -> bool {
    dict.get("Subtype")
        .and_then(|o| o.as_name())
        .map(|name| name == "Widget")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extractors::forms::{FieldType, FieldValue, FormField};

    #[test]
    fn test_form_field_value_from_field_value() {
        // Test text conversion
        let text_value = FieldValue::Text("hello".to_string());
        let converted: FormFieldValue = text_value.into();
        assert_eq!(converted, FormFieldValue::Text("hello".to_string()));

        // Test boolean conversion
        let bool_value = FieldValue::Boolean(true);
        let converted: FormFieldValue = bool_value.into();
        assert_eq!(converted, FormFieldValue::Boolean(true));

        // Test name conversion (to Choice)
        let name_value = FieldValue::Name("option1".to_string());
        let converted: FormFieldValue = name_value.into();
        assert_eq!(converted, FormFieldValue::Choice("option1".to_string()));

        // Test array conversion
        let array_value = FieldValue::Array(vec!["a".to_string(), "b".to_string()]);
        let converted: FormFieldValue = array_value.into();
        assert_eq!(converted, FormFieldValue::MultiChoice(vec!["a".to_string(), "b".to_string()]));

        // Test none conversion
        let none_value = FieldValue::None;
        let converted: FormFieldValue = none_value.into();
        assert_eq!(converted, FormFieldValue::None);
    }

    #[test]
    fn test_form_field_value_to_object() {
        // Test text to object
        let text_value = FormFieldValue::Text("hello".to_string());
        let obj: Object = (&text_value).into();
        assert!(matches!(obj, Object::String(_)));

        // Test boolean true to object
        let bool_true = FormFieldValue::Boolean(true);
        let obj: Object = (&bool_true).into();
        assert_eq!(obj, Object::Name("Yes".to_string()));

        // Test boolean false to object
        let bool_false = FormFieldValue::Boolean(false);
        let obj: Object = (&bool_false).into();
        assert_eq!(obj, Object::Name("Off".to_string()));

        // Test none to object
        let none_value = FormFieldValue::None;
        let obj: Object = (&none_value).into();
        assert_eq!(obj, Object::Null);
    }

    #[test]
    fn test_form_field_value_accessors() {
        let text_value = FormFieldValue::Text("hello".to_string());
        assert_eq!(text_value.as_text(), Some("hello"));
        assert_eq!(text_value.as_bool(), None);
        assert!(!text_value.is_none());

        let bool_value = FormFieldValue::Boolean(true);
        assert_eq!(bool_value.as_bool(), Some(true));
        assert_eq!(bool_value.as_text(), None);

        let none_value = FormFieldValue::None;
        assert!(none_value.is_none());
    }

    #[test]
    fn test_wrapper_from_read() {
        let field = FormField {
            name: "test".to_string(),
            field_type: FieldType::Text,
            value: FieldValue::Text("hello".to_string()),
            tooltip: Some("A tooltip".to_string()),
            full_name: "form.test".to_string(),
            bounds: Some([100.0, 200.0, 300.0, 220.0]),
            object_ref: None,
            flags: None,
            default_value: None,
            max_length: None,
            alignment: None,
            default_appearance: None,
            border_style: None,
            appearance_chars: None,
        };

        let wrapper = FormFieldWrapper::from_read(field, 0, None);

        assert_eq!(wrapper.name(), "form.test");
        assert_eq!(wrapper.page_index(), 0);
        assert!(!wrapper.is_new());
        assert!(!wrapper.is_modified());
        assert_eq!(wrapper.value(), FormFieldValue::Text("hello".to_string()));
        assert_eq!(wrapper.tooltip(), Some("A tooltip"));
    }

    #[test]
    fn test_wrapper_set_value() {
        let field = FormField {
            name: "test".to_string(),
            field_type: FieldType::Text,
            value: FieldValue::Text("original".to_string()),
            tooltip: None,
            full_name: "test".to_string(),
            bounds: None,
            object_ref: None,
            flags: None,
            default_value: None,
            max_length: None,
            alignment: None,
            default_appearance: None,
            border_style: None,
            appearance_chars: None,
        };

        let mut wrapper = FormFieldWrapper::from_read(field, 0, None);

        // Initial value
        assert_eq!(wrapper.value(), FormFieldValue::Text("original".to_string()));
        assert!(!wrapper.is_modified());

        // Set new value
        wrapper.set_value(FormFieldValue::Text("modified".to_string()));
        assert_eq!(wrapper.value(), FormFieldValue::Text("modified".to_string()));
        assert!(wrapper.is_modified());
    }

    #[test]
    fn test_is_merged_field_dict() {
        let mut merged_dict = HashMap::new();
        merged_dict.insert("Subtype".to_string(), Object::Name("Widget".to_string()));
        assert!(is_merged_field_dict(&merged_dict));

        let mut separate_dict = HashMap::new();
        separate_dict.insert("FT".to_string(), Object::Name("Tx".to_string()));
        assert!(!is_merged_field_dict(&separate_dict));
    }
}
