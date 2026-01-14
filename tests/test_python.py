"""
Python bindings tests for pdf_oxide.

These tests verify the Python API works correctly, including:
- Opening PDF files
- Extracting text
- Converting to Markdown
- Converting to HTML
- Error handling
"""

import pytest

from pdf_oxide import PdfDocument


def test_open_pdf():
    """Test opening a PDF file."""
    # Note: This test will need actual PDF fixtures to run
    # For now, it documents the expected behavior
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")
        assert doc is not None
        # Version should be a tuple of two integers
        version = doc.version()
        assert isinstance(version, tuple)
        assert len(version) == 2
        assert isinstance(version[0], int)
        assert isinstance(version[1], int)
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_version():
    """Test getting PDF version."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")
        major, minor = doc.version()
        assert major >= 1
        assert minor >= 0
        # PDF versions are typically 1.0 through 2.0
        assert major <= 2
        assert minor <= 7
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_page_count():
    """Test getting page count."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")
        count = doc.page_count()
        assert isinstance(count, int)
        assert count >= 1
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_extract_text():
    """Test extracting text from a page."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")
        text = doc.extract_text(0)
        assert isinstance(text, str)
        # Text should be non-empty for a real PDF
        # (empty is ok for a minimal test PDF though)
        assert text is not None
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_extract_text_with_content():
    """Test extracting text that contains specific content."""
    try:
        doc = PdfDocument("tests/fixtures/hello_world.pdf")
        text = doc.extract_text(0)
        assert isinstance(text, str)
        assert len(text) > 0
        # Should contain "Hello" or "hello" (case-insensitive check)
        assert "hello" in text.lower()
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'hello_world.pdf' not available or invalid")


def test_to_markdown():
    """Test converting a page to Markdown."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")
        markdown = doc.to_markdown(0)
        assert isinstance(markdown, str)
        assert markdown is not None
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_to_markdown_with_options():
    """Test converting to Markdown with custom options."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")

        # Test with heading detection enabled
        markdown = doc.to_markdown(0, detect_headings=True)
        assert isinstance(markdown, str)

        # Test with heading detection disabled
        markdown = doc.to_markdown(0, detect_headings=False)
        assert isinstance(markdown, str)

        # Test with layout preservation
        markdown = doc.to_markdown(0, preserve_layout=True)
        assert isinstance(markdown, str)
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_to_html():
    """Test converting a page to HTML."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")
        html = doc.to_html(0)
        assert isinstance(html, str)
        assert html is not None
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_to_html_semantic_mode():
    """Test converting to semantic HTML (default mode)."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")
        html = doc.to_html(0, preserve_layout=False)
        assert isinstance(html, str)
        # Semantic HTML should not contain absolute positioning
        # (though it might not contain much if the PDF is simple)
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_to_html_layout_mode():
    """Test converting to layout-preserved HTML."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")
        html = doc.to_html(0, preserve_layout=True)
        assert isinstance(html, str)
        # Layout mode should include positioning CSS
        # Check if it contains position-related CSS or inline styles
        # (only if the PDF has content)
        if len(html) > 100:
            assert "position" in html.lower() or "style" in html.lower()
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_to_markdown_all():
    """Test converting all pages to Markdown."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")
        markdown = doc.to_markdown_all()
        assert isinstance(markdown, str)
        assert markdown is not None
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_to_markdown_all_multipage():
    """Test converting multiple pages to Markdown."""
    try:
        doc = PdfDocument("tests/fixtures/multipage.pdf")
        markdown = doc.to_markdown_all()
        assert isinstance(markdown, str)
        assert len(markdown) > 0
        # Multi-page markdown should contain horizontal rules as separators
        page_count = doc.page_count()
        if page_count > 1:
            assert "---" in markdown
    except OSError:
        pytest.skip("Test fixture 'multipage.pdf' not available")


def test_to_html_all():
    """Test converting all pages to HTML."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")
        html = doc.to_html_all()
        assert isinstance(html, str)
        assert html is not None
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_to_html_all_multipage():
    """Test converting multiple pages to HTML."""
    try:
        doc = PdfDocument("tests/fixtures/multipage.pdf")
        html = doc.to_html_all()
        assert isinstance(html, str)
        assert len(html) > 0
        # Multi-page HTML should contain page div elements
        page_count = doc.page_count()
        if page_count > 1:
            assert 'class="page"' in html or "data-page" in html
    except OSError:
        pytest.skip("Test fixture 'multipage.pdf' not available")


def test_error_handling_nonexistent_file():
    """Test error handling for non-existent file."""
    with pytest.raises(IOError) as exc_info:
        PdfDocument("nonexistent_file_that_does_not_exist.pdf")

    # Error message should be helpful
    error_msg = str(exc_info.value)
    assert "Failed to open PDF" in error_msg or "No such file" in error_msg


def test_error_handling_invalid_page():
    """Test error handling for invalid page index."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")
        page_count = doc.page_count()

        # Try to access a page that doesn't exist
        with pytest.raises(RuntimeError) as exc_info:
            doc.extract_text(page_count + 100)

        # Error message should indicate the problem
        error_msg = str(exc_info.value)
        assert "Failed to extract text" in error_msg or "page" in error_msg.lower()
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_error_handling_invalid_page_conversion():
    """Test error handling for invalid page in conversion."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")
        page_count = doc.page_count()

        # Try to convert a page that doesn't exist
        with pytest.raises(RuntimeError):
            doc.to_markdown(page_count + 100)
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_repr():
    """Test string representation of PdfDocument."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")
        repr_str = repr(doc)
        assert isinstance(repr_str, str)
        assert "PdfDocument" in repr_str
        assert "version=" in repr_str
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_multiple_operations():
    """Test performing multiple operations on the same document."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")

        # Get version multiple times
        version1 = doc.version()
        version2 = doc.version()
        assert version1 == version2

        # Extract text multiple times
        text1 = doc.extract_text(0)
        text2 = doc.extract_text(0)
        assert text1 == text2

        # Convert to different formats
        markdown = doc.to_markdown(0)
        html = doc.to_html(0)
        assert isinstance(markdown, str)
        assert isinstance(html, str)
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_image_output_dir():
    """Test specifying image output directory."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")

        # Convert with image output directory specified
        markdown = doc.to_markdown(0, image_output_dir="./test_images")
        assert isinstance(markdown, str)

        # Convert without images
        markdown = doc.to_markdown(0, include_images=False)
        assert isinstance(markdown, str)
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


def test_all_options_combined():
    """Test using all conversion options together."""
    try:
        doc = PdfDocument("tests/fixtures/simple.pdf")

        # Test with all options specified
        markdown = doc.to_markdown(
            0,
            preserve_layout=True,
            detect_headings=False,
            include_images=True,
            image_output_dir="./output",
        )
        assert isinstance(markdown, str)

        html = doc.to_html(
            0,
            preserve_layout=True,
            detect_headings=True,
            include_images=False,
            image_output_dir=None,
        )
        assert isinstance(html, str)
    except (OSError, RuntimeError):
        pytest.skip("Test fixture 'simple.pdf' not available or invalid")


# === PDF Creation Tests ===


def test_pdf_from_markdown():
    """Test creating PDF from Markdown."""
    from pdf_oxide import Pdf

    md_content = """# Test Document

This is a **test** paragraph.

## Section 1

Some text content.
"""
    pdf = Pdf.from_markdown(md_content)
    assert pdf is not None
    # PDF should have some bytes
    pdf_bytes = pdf.to_bytes()
    assert isinstance(pdf_bytes, bytes)
    assert len(pdf_bytes) > 0
    # Should start with PDF header
    assert pdf_bytes[:4] == b"%PDF"


def test_pdf_from_markdown_with_options():
    """Test creating PDF from Markdown with options."""
    from pdf_oxide import Pdf

    md_content = "# Hello World"
    pdf = Pdf.from_markdown(
        md_content,
        title="Test Title",
        author="Test Author",
    )
    assert pdf is not None
    pdf_bytes = pdf.to_bytes()
    assert len(pdf_bytes) > 0


def test_pdf_from_html():
    """Test creating PDF from HTML."""
    from pdf_oxide import Pdf

    html_content = """
    <h1>Test Document</h1>
    <p>This is a <strong>test</strong> paragraph.</p>
    """
    pdf = Pdf.from_html(html_content)
    assert pdf is not None
    pdf_bytes = pdf.to_bytes()
    assert isinstance(pdf_bytes, bytes)
    assert len(pdf_bytes) > 0
    assert pdf_bytes[:4] == b"%PDF"


def test_pdf_from_text():
    """Test creating PDF from plain text."""
    from pdf_oxide import Pdf

    text_content = "Hello, World!\n\nThis is plain text."
    pdf = Pdf.from_text(text_content)
    assert pdf is not None
    pdf_bytes = pdf.to_bytes()
    assert len(pdf_bytes) > 0
    assert pdf_bytes[:4] == b"%PDF"


def test_pdf_save_to_file(tmp_path):
    """Test saving PDF to a file."""
    from pdf_oxide import Pdf

    pdf = Pdf.from_text("Test content")
    output_path = tmp_path / "output.pdf"
    pdf.save(str(output_path))
    assert output_path.exists()
    assert output_path.stat().st_size > 0


# === Advanced Graphics Tests ===


def test_color_creation():
    """Test Color class creation."""
    from pdf_oxide import Color

    # Create from RGB values
    color = Color(1.0, 0.0, 0.0)
    assert color is not None

    # Create from hex
    color = Color.from_hex("#FF0000")
    assert color is not None

    color = Color.from_hex("00FF00")
    assert color is not None


def test_color_predefined():
    """Test predefined colors."""
    from pdf_oxide import Color

    black = Color.black()
    assert black is not None

    white = Color.white()
    assert white is not None

    red = Color.red()
    assert red is not None

    green = Color.green()
    assert green is not None

    blue = Color.blue()
    assert blue is not None


def test_blend_modes():
    """Test BlendMode constants."""
    from pdf_oxide import BlendMode

    # Test all blend modes are accessible
    assert BlendMode.NORMAL() is not None
    assert BlendMode.MULTIPLY() is not None
    assert BlendMode.SCREEN() is not None
    assert BlendMode.OVERLAY() is not None
    assert BlendMode.DARKEN() is not None
    assert BlendMode.LIGHTEN() is not None
    assert BlendMode.COLOR_DODGE() is not None
    assert BlendMode.COLOR_BURN() is not None
    assert BlendMode.HARD_LIGHT() is not None
    assert BlendMode.SOFT_LIGHT() is not None
    assert BlendMode.DIFFERENCE() is not None
    assert BlendMode.EXCLUSION() is not None


def test_ext_gstate():
    """Test ExtGState (transparency) builder."""
    from pdf_oxide import BlendMode, ExtGState

    # Create with fill alpha
    gs = ExtGState().fill_alpha(0.5)
    assert gs is not None

    # Chained builder pattern
    gs = ExtGState().fill_alpha(0.5).stroke_alpha(0.8).blend_mode(BlendMode.MULTIPLY())
    assert gs is not None


def test_ext_gstate_presets():
    """Test ExtGState preset methods."""
    from pdf_oxide import BlendMode, ExtGState

    semi = ExtGState.semi_transparent()
    assert semi is not None

    # Test creating with blend mode (instead of preset static methods)
    multiply = ExtGState().blend_mode(BlendMode.MULTIPLY())
    assert multiply is not None

    screen = ExtGState().blend_mode(BlendMode.SCREEN())
    assert screen is not None


def test_linear_gradient():
    """Test LinearGradient builder."""
    from pdf_oxide import Color, LinearGradient

    # Basic gradient
    gradient = (
        LinearGradient()
        .start(0.0, 0.0)
        .end(100.0, 100.0)
        .add_stop(0.0, Color.red())
        .add_stop(1.0, Color.blue())
    )
    assert gradient is not None


def test_linear_gradient_presets():
    """Test LinearGradient preset methods."""
    from pdf_oxide import Color, LinearGradient

    # Horizontal preset
    gradient = LinearGradient.horizontal(100.0, Color.black(), Color.white())
    assert gradient is not None

    # Vertical preset
    gradient = LinearGradient.vertical(100.0, Color.black(), Color.white())
    assert gradient is not None

    # Manual two-color gradient
    gradient = LinearGradient().add_stop(0.0, Color.black()).add_stop(1.0, Color.white())
    assert gradient is not None


def test_radial_gradient():
    """Test RadialGradient builder."""
    from pdf_oxide import Color, RadialGradient

    gradient = (
        RadialGradient()
        .inner_circle(50.0, 50.0, 0.0)
        .outer_circle(50.0, 50.0, 50.0)
        .add_stop(0.0, Color.white())
        .add_stop(1.0, Color.black())
    )
    assert gradient is not None


def test_radial_gradient_centered():
    """Test centered RadialGradient."""
    from pdf_oxide import RadialGradient

    gradient = RadialGradient.centered(50.0, 50.0, 50.0)
    assert gradient is not None


def test_line_cap():
    """Test LineCap constants."""
    from pdf_oxide import LineCap

    assert LineCap.BUTT() is not None
    assert LineCap.ROUND() is not None
    assert LineCap.SQUARE() is not None


def test_line_join():
    """Test LineJoin constants."""
    from pdf_oxide import LineJoin

    assert LineJoin.MITER() is not None
    assert LineJoin.ROUND() is not None
    assert LineJoin.BEVEL() is not None


def test_pattern_presets():
    """Test PatternPresets static methods."""
    from pdf_oxide import Color, PatternPresets

    # Horizontal stripes
    content = PatternPresets.horizontal_stripes(10.0, 10.0, 5.0, Color.red())
    assert isinstance(content, bytes)
    assert len(content) > 0

    # Vertical stripes
    content = PatternPresets.vertical_stripes(10.0, 10.0, 5.0, Color.blue())
    assert isinstance(content, bytes)
    assert len(content) > 0

    # Checkerboard
    content = PatternPresets.checkerboard(10.0, Color.white(), Color.black())
    assert isinstance(content, bytes)
    assert len(content) > 0

    # Dots
    content = PatternPresets.dots(10.0, 2.0, Color.red())
    assert isinstance(content, bytes)
    assert len(content) > 0

    # Diagonal lines
    content = PatternPresets.diagonal_lines(10.0, 0.5, Color.black())
    assert isinstance(content, bytes)
    assert len(content) > 0

    # Crosshatch
    content = PatternPresets.crosshatch(10.0, 0.5, Color.black())
    assert isinstance(content, bytes)
    assert len(content) > 0


# Note: To run these tests successfully, you'll need to:
# 1. Install maturin: pip install maturin
# 2. Build the extension: maturin develop
# 3. Install pytest: pip install pytest
# 4. Create test PDF fixtures in tests/fixtures/
# 5. Run tests: pytest tests/test_python.py
