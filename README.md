# mdterm

A markdown terminal viewer that renders markdown files in the terminal with browser-like styling.

## Features

- **Browser-like styling** - Headings, links, code blocks, blockquotes, and tables rendered with styling similar to web browsers
- **Syntax highlighting** - Code blocks are highlighted using Pygments
- **Light and dark themes** - Choose between light and dark color schemes
- **Flexible input** - Read from file arguments or stdin/pipe
- **Windows Terminal compatible** - Works properly on Windows Terminal

## Installation

```bash
pip install mdterm
```

Or install from source:

```bash
pip install -e .
```

## Usage

```bash
# Render a markdown file
mdterm README.md

# Read from stdin
cat README.md | mdterm --stdin

# Use dark theme
mdterm --theme dark README.md

# Specify terminal width
mdterm --width 100 README.md

# Disable colors
mdterm --no-color README.md
```

## Options

- `FILE` - Markdown file to render (optional, use `--stdin` for pipe input)
- `-s, --stdin` - Read from stdin instead of a file
- `-w, --width WIDTH` - Terminal width for wrapping (default: auto-detect)
- `-t, --theme {light,dark}` - Color theme (default: light)
- `--code-theme THEME` - Code syntax highlighting theme (default: monokai)
- `--no-color` - Disable color output

## Examples

### Headings

# Heading 1
## Heading 2
### Heading 3

Rendered with blue color and underlines.

### Links

[Link text](https://example.com)

Rendered in green with URL shown in parentheses.

### Code Blocks

```python
def hello():
    print("Hello, World!")
```

Syntax highlighted with your chosen theme.

### Blockquotes

> This is a blockquote
> with multiple lines

Rendered with a border on the left.

### Tables

| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |

Rendered with box-drawing characters.

## License

MIT
