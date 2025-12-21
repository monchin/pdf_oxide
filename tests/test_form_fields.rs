//! Integration tests for interactive PDF form field creation.
//!
//! Tests the complete form field creation workflow including:
//! - Individual field types (text, checkbox, radio, combo, list, push button)
//! - AcroForm integration
//! - PDF structure validation

use pdf_oxide::geometry::Rect;
use pdf_oxide::writer::{
    CheckboxWidget, ChoiceOption, ComboBoxWidget, FormAction, ListBoxWidget, PdfWriter,
    PushButtonWidget, RadioButtonGroup, TextAlignment, TextFieldWidget,
};

#[test]
fn test_create_pdf_with_text_field() {
    let mut writer = PdfWriter::new();
    {
        let mut page = writer.add_page(612.0, 792.0);

        let text_field = TextFieldWidget::new("name", Rect::new(72.0, 700.0, 200.0, 20.0))
            .with_value("John Doe")
            .required();

        page.add_text_field(text_field);
    }

    let bytes = writer.finish().expect("Failed to create PDF");

    // Verify PDF was created
    assert!(!bytes.is_empty());
    assert!(bytes.starts_with(b"%PDF-"));

    // Verify form field structures are present
    let content = String::from_utf8_lossy(&bytes);
    assert!(content.contains("/AcroForm"));
    assert!(content.contains("/FT /Tx")); // Text field type
    assert!(content.contains("/T (name)")); // Field name
}

#[test]
fn test_create_pdf_with_checkbox() {
    let mut writer = PdfWriter::new();
    {
        let mut page = writer.add_page(612.0, 792.0);

        let checkbox = CheckboxWidget::new("agree", Rect::new(72.0, 650.0, 15.0, 15.0))
            .checked()
            .with_export_value("Yes");

        page.add_checkbox(checkbox);
    }

    let bytes = writer.finish().expect("Failed to create PDF");

    assert!(!bytes.is_empty());
    let content = String::from_utf8_lossy(&bytes);
    assert!(content.contains("/AcroForm"));
    assert!(content.contains("/FT /Btn")); // Button field type (checkbox is a button)
    assert!(content.contains("/T (agree)")); // Field name
}

#[test]
fn test_create_pdf_with_radio_buttons() {
    let mut writer = PdfWriter::new();
    {
        let mut page = writer.add_page(612.0, 792.0);

        let radio_group = RadioButtonGroup::new("payment_method")
            .add_button("credit", Rect::new(72.0, 600.0, 15.0, 15.0), "Credit Card")
            .add_button("paypal", Rect::new(72.0, 580.0, 15.0, 15.0), "PayPal")
            .add_button("cash", Rect::new(72.0, 560.0, 15.0, 15.0), "Cash")
            .selected("credit");

        page.add_radio_group(radio_group);
    }

    let bytes = writer.finish().expect("Failed to create PDF");

    assert!(!bytes.is_empty());
    let content = String::from_utf8_lossy(&bytes);
    assert!(content.contains("/AcroForm"));
    assert!(content.contains("/T (payment_method)")); // Field name
}

#[test]
fn test_create_pdf_with_combo_box() {
    let mut writer = PdfWriter::new();
    {
        let mut page = writer.add_page(612.0, 792.0);

        let combo = ComboBoxWidget::new("country", Rect::new(72.0, 500.0, 150.0, 20.0))
            .with_options(vec!["USA", "Canada", "UK", "Germany", "France"])
            .with_value("USA");

        page.add_combo_box(combo);
    }

    let bytes = writer.finish().expect("Failed to create PDF");

    assert!(!bytes.is_empty());
    let content = String::from_utf8_lossy(&bytes);
    assert!(content.contains("/AcroForm"));
    assert!(content.contains("/FT /Ch")); // Choice field type
    assert!(content.contains("/T (country)")); // Field name
    assert!(content.contains("/Opt")); // Options array
}

#[test]
fn test_create_pdf_with_list_box() {
    let mut writer = PdfWriter::new();
    {
        let mut page = writer.add_page(612.0, 792.0);

        let list = ListBoxWidget::new("interests", Rect::new(72.0, 400.0, 150.0, 80.0))
            .with_options(vec!["Sports", "Music", "Art", "Technology"])
            .multi_select();

        page.add_list_box(list);
    }

    let bytes = writer.finish().expect("Failed to create PDF");

    assert!(!bytes.is_empty());
    let content = String::from_utf8_lossy(&bytes);
    assert!(content.contains("/AcroForm"));
    assert!(content.contains("/FT /Ch")); // Choice field type
    assert!(content.contains("/T (interests)")); // Field name
}

#[test]
fn test_create_pdf_with_list_box_choice_options() {
    let mut writer = PdfWriter::new();
    {
        let mut page = writer.add_page(612.0, 792.0);

        // Use ChoiceOption with different display/export values
        let list = ListBoxWidget::new("categories", Rect::new(72.0, 400.0, 150.0, 80.0))
            .with_choice_options(vec![
                ChoiceOption::new_with_export("Sports", "cat_sports"),
                ChoiceOption::new_with_export("Music", "cat_music"),
                ChoiceOption::new_with_export("Art", "cat_art"),
            ])
            .multi_select();

        page.add_list_box(list);
    }

    let bytes = writer.finish().expect("Failed to create PDF");

    assert!(!bytes.is_empty());
    let content = String::from_utf8_lossy(&bytes);
    assert!(content.contains("/AcroForm"));
    assert!(content.contains("/T (categories)")); // Field name
}

#[test]
fn test_create_pdf_with_push_button() {
    let mut writer = PdfWriter::new();
    {
        let mut page = writer.add_page(612.0, 792.0);

        let submit = PushButtonWidget::new("submit", Rect::new(72.0, 300.0, 80.0, 25.0))
            .with_caption("Submit")
            .with_action(FormAction::SubmitForm {
                url: "https://example.com/submit".to_string(),
                flags: Default::default(),
            });

        let reset = PushButtonWidget::new("reset", Rect::new(160.0, 300.0, 80.0, 25.0))
            .with_caption("Reset")
            .with_action(FormAction::ResetForm);

        page.add_push_button(submit);
        page.add_push_button(reset);
    }

    let bytes = writer.finish().expect("Failed to create PDF");

    assert!(!bytes.is_empty());
    let content = String::from_utf8_lossy(&bytes);
    assert!(content.contains("/AcroForm"));
    assert!(content.contains("/T (submit)")); // Field name
    assert!(content.contains("/T (reset)")); // Field name
}

#[test]
fn test_create_complete_form() {
    let mut writer = PdfWriter::new();
    {
        let mut page = writer.add_page(612.0, 792.0);

        // Add various field types
        page.add_text_field(
            TextFieldWidget::new("fullName", Rect::new(150.0, 700.0, 200.0, 20.0))
                .with_value("")
                .required(),
        );

        page.add_text_field(
            TextFieldWidget::new("email", Rect::new(150.0, 670.0, 200.0, 20.0)).with_value(""),
        );

        page.add_text_field(
            TextFieldWidget::new("comments", Rect::new(150.0, 600.0, 300.0, 60.0)).multiline(),
        );

        page.add_checkbox(CheckboxWidget::new("newsletter", Rect::new(150.0, 560.0, 15.0, 15.0)));

        page.add_combo_box(
            ComboBoxWidget::new("country", Rect::new(150.0, 530.0, 150.0, 20.0))
                .with_options(vec!["Select...", "USA", "Canada", "UK"]),
        );

        page.add_push_button(
            PushButtonWidget::new("submit", Rect::new(150.0, 480.0, 80.0, 25.0))
                .with_caption("Submit")
                .with_action(FormAction::SubmitForm {
                    url: "https://example.com/submit".to_string(),
                    flags: Default::default(),
                }),
        );
    }

    let bytes = writer.finish().expect("Failed to create PDF");

    assert!(!bytes.is_empty());

    let content = String::from_utf8_lossy(&bytes);

    // Verify AcroForm dictionary
    assert!(content.contains("/AcroForm"));
    assert!(content.contains("/NeedAppearances true"));
    assert!(content.contains("/DA")); // Default appearance
    assert!(content.contains("/DR")); // Default resources

    // Verify field types are present
    assert!(content.contains("/FT /Tx")); // Text fields
    assert!(content.contains("/FT /Btn")); // Button fields (checkbox, push button)
    assert!(content.contains("/FT /Ch")); // Choice field (combo box)
}

#[test]
fn test_text_field_options() {
    let mut writer = PdfWriter::new();
    {
        let mut page = writer.add_page(612.0, 792.0);

        // Password field
        page.add_text_field(
            TextFieldWidget::new("password", Rect::new(72.0, 700.0, 200.0, 20.0)).password(),
        );

        // Comb field with max length
        page.add_text_field(
            TextFieldWidget::new("ssn", Rect::new(72.0, 670.0, 150.0, 20.0))
                .with_max_length(9)
                .comb(),
        );

        // Right-aligned currency field
        page.add_text_field(
            TextFieldWidget::new("amount", Rect::new(72.0, 640.0, 100.0, 20.0))
                .with_alignment(TextAlignment::Right),
        );

        // Read-only field
        page.add_text_field(
            TextFieldWidget::new("readonly", Rect::new(72.0, 610.0, 200.0, 20.0))
                .with_value("Cannot edit this")
                .read_only(),
        );
    }

    let bytes = writer.finish().expect("Failed to create PDF");

    assert!(!bytes.is_empty());
    let content = String::from_utf8_lossy(&bytes);

    // Verify fields are present
    assert!(content.contains("/T (password)"));
    assert!(content.contains("/T (ssn)"));
    assert!(content.contains("/T (amount)"));
    assert!(content.contains("/T (readonly)"));
}

#[test]
fn test_multiple_pages_with_forms() {
    let mut writer = PdfWriter::new();

    // Page 1 - Personal info
    {
        let mut page = writer.add_page(612.0, 792.0);
        page.add_text_field(TextFieldWidget::new(
            "page1_name",
            Rect::new(72.0, 700.0, 200.0, 20.0),
        ));
    }

    // Page 2 - Preferences
    {
        let mut page = writer.add_page(612.0, 792.0);
        page.add_checkbox(CheckboxWidget::new("page2_option", Rect::new(72.0, 700.0, 15.0, 15.0)));
    }

    let bytes = writer.finish().expect("Failed to create PDF");

    assert!(!bytes.is_empty());
    let content = String::from_utf8_lossy(&bytes);

    // Verify fields from both pages
    assert!(content.contains("/T (page1_name)"));
    assert!(content.contains("/T (page2_option)"));

    // AcroForm should contain fields from all pages
    assert!(content.contains("/AcroForm"));
}

#[test]
fn test_no_form_fields_no_acroform() {
    let mut writer = PdfWriter::new();
    {
        let mut page = writer.add_page(612.0, 792.0);
        page.add_text("Hello World", 72.0, 700.0, "Helvetica", 12.0);
    }

    let bytes = writer.finish().expect("Failed to create PDF");

    assert!(!bytes.is_empty());
    let content = String::from_utf8_lossy(&bytes);

    // No AcroForm when no form fields
    assert!(!content.contains("/AcroForm"));
}
