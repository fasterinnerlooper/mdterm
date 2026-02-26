"""Tests for mdterm renderer."""

import pytest
from mdterm.renderer import (
    TerminalRenderer,
    RenderOptions,
    Colors,
    HEADING_STYLES,
)


def test_heading_rendering():
    """Test that headings are rendered with correct color codes."""
    renderer = TerminalRenderer(RenderOptions(width=80))
    
    # Test H1
    html = "<h1>Test Heading</h1>"
    result = renderer._render_heading(1, html)
    expected_color = Colors.get_color(HEADING_STYLES[1])
    assert expected_color in result
    assert "Test Heading" in result
    assert Colors.RESET in result
    
    # Test H2
    html = "<h2>Second Level</h2>"
    result = renderer._render_heading(2, html)
    expected_color = Colors.get_color(HEADING_STYLES[2])
    assert expected_color in result


def test_inline_elements():
    """Test inline element rendering."""
    renderer = TerminalRenderer(RenderOptions(width=80))
    
    # Test bold
    content = "<strong>Bold Text</strong>"
    result = renderer._render_inline(content)
    assert Colors.get_color(Colors.STRONG) in result or Colors.STRONG in result
    assert "Bold Text" in result
    
    # Test italic
    content = "<em>Italic Text</em>"
    result = renderer._render_inline(content)
    assert Colors.get_color(Colors.EMPHASIS) in result or Colors.EMPHASIS in result
    assert "Italic Text" in result
    
    # Test links
    content = '<a href="https://example.com">Link Text</a>'
    result = renderer._render_inline(content)
    assert Colors.get_color(Colors.LINK) in result or Colors.LINK in result
    assert "Link Text" in result
    assert "https://example.com" in result
    
    # Test inline code
    content = "<code>print('hello')</code>"
    result = renderer._render_inline(content)
    assert Colors.get_color(Colors.CODE) in result or Colors.CODE in result
    assert "print('hello')" in result


def test_blockquote_rendering():
    """Test blockquote rendering with border."""
    renderer = TerminalRenderer(RenderOptions(width=80))
    
    lines = ["<p>This is a quote.</p>"]
    result = renderer._render_blockquote(lines)
    
    # Should have border character
    assert "│" in result
    # Should have border color
    border_color = Colors.get_color(Colors.BLOCKQUOTE_BORDER)
    assert border_color in result
    # Should have text color
    text_color = Colors.get_color(Colors.BLOCKQUOTE_TEXT)
    assert text_color in result
    assert "This is a quote." in result


def test_table_rendering():
    """Test table rendering with headers."""
    renderer = TerminalRenderer(RenderOptions(width=80))
    
    # Simulate a simple table HTML lines
    lines = [
        "<thead>",
        "<tr><th>Name</th><th>Age</th></tr>",
        "</thead>",
        "<tbody>",
        "<tr><td>Alice</td><td>30</td></tr>",
        "<tr><td>Bob</td><td>25</td></tr>",
        "</tbody>",
    ]
    
    result = renderer._render_table(lines)
    
    # Check for table borders
    assert "│" in result
    assert "├" in result
    assert "┼" in result
    assert "┤" in result
    
    # Check content
    assert "Name" in result
    assert "Age" in result
    assert "Alice" in result
    assert "Bob" in result
    assert "30" in result
    assert "25" in result


def test_list_rendering():
    """Test list item rendering."""
    renderer = TerminalRenderer(RenderOptions(width=80))
    
    # Test list item through full rendering
    markdown = "- List item text"
    result = renderer.render(markdown)
    
    # Should contain bullet point and colored marker
    assert "•" in result
    assert "List item text" in result


def test_no_color_mode():
    """Test that no-color mode strips ANSI codes."""
    renderer = TerminalRenderer(RenderOptions(width=80, theme="light", no_color=True))
    
    markdown = "# Heading\n\n**Bold** and *italic*"
    output = renderer.render(markdown)
    
    # Check that there are no ANSI escape sequences
    assert "\033[" not in output
    # Check that content is still present
    assert "Heading" in output
    assert "Bold" in output
    assert "italic" in output


def test_full_markdown():
    """Test rendering a complete markdown document."""
    renderer = TerminalRenderer(RenderOptions(width=80))
    
    markdown = """# Main Title

## Subtitle

This is a paragraph with **bold** and *italic* and `inline code`.

> A blockquote with multiple lines.
> Second line.

| Col1 | Col2 |
|------|------|
| A    | B    |
| C    | D    |

- Item 1
- Item 2

```python
def hello():
    print(\"Hello\")
```

---

End."""
    
    result = renderer.render(markdown)
    
    # Verify key elements are present
    assert "Main Title" in result
    assert "Subtitle" in result
    assert "bold" in result
    assert "italic" in result
    assert "inline code" in result
    assert "blockquote" in result.lower()
    assert "Col1" in result
    assert "A" in result and "B" in result
    assert "Item 1" in result
    # Check code block contains def, hello, and print
    assert "def" in result and "hello" in result and "print" in result
    assert "Hello" in result
    # Horizontal rule should be present as long line
    assert "─" in result or "═" in result
    assert "End" in result


def test_color_constants():
    """Test that color constants are valid ANSI codes."""
    # All color codes should start with ESC[38;5; or ESC[1m etc.
    assert Colors.RESET == "\033[0m"
    assert Colors.BOLD == "\033[1m"
    assert Colors.ITALIC == "\033[3m"
    assert Colors.UNDERLINE == "\033[4m"
    # 256-color codes should match pattern
    assert Colors.HEADING1.startswith("\033[38;5;")
    assert Colors.LINK.startswith("\033[38;5;")
    assert Colors.CODE.startswith("\033[38;5;")


def test_windows_detection():
    """Test platform detection variables."""
    # Just ensure these booleans exist
    assert isinstance(Colors.IS_WINDOWS, bool)
    assert isinstance(Colors.IS_WINDOWS_TERMINAL, bool)


def test_heading_underlines():
    """Test that H1 and H2 get underlines."""
    renderer = TerminalRenderer(RenderOptions(width=80))
    
    # H1 underline
    result = renderer._render_heading(1, "Heading 1")
    assert "═" in result
    
    # H2 underline
    result = renderer._render_heading(2, "Heading 2")
    assert "─" in result
    
    # H3+ no underline
    result = renderer._render_heading(3, "Heading 3")
    assert "═" not in result and "─" not in result
