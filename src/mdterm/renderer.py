"""Markdown to terminal renderer with browser-like styling."""

import re
import sys
import platform
import os
import io
from dataclasses import dataclass
from typing import Optional, List

import markdown
from pygments import highlight
from pygments.formatters.terminal import TerminalFormatter
from pygments.lexers import get_lexer_by_name, TextLexer
from rich.console import Console
from rich.markdown import Markdown as RichMarkdown


# ANSI 256-color codes for browser-like styling
class Colors:
    RESET = "\033[0m"
    
    # Standard bold
    BOLD = "\033[1m"
    ITALIC = "\033[3m"
    UNDERLINE = "\033[4m"
    
    # Detect Windows Terminal and platform
    IS_WINDOWS = platform.system() == "Windows"
    IS_WINDOWS_TERMINAL = IS_WINDOWS and "WT_SESSION" in os.environ
    
    # Use 256-color codes for Windows Terminal
    HEADING1 = "\033[38;5;33m"     # Blue
    HEADING2 = "\033[38;5;33m"     # Blue
    HEADING3 = "\033[38;5;33m"     # Blue
    HEADING4 = "\033[38;5;33m"     # Blue
    HEADING5 = "\033[38;5;33m"     # Blue
    HEADING6 = "\033[38;5;33m"     # Blue
    
    # Links
    LINK = "\033[38;5;32m"        # Green
    
    # Code
    CODE = "\033[38;5;196m"        # Red
    
    # Semantic
    STRONG = "\033[1m"
    EMPHASIS = "\033[3m"
    
    # Lists
    LIST_MARKER = "\033[38;5;244m"  # Gray
    
    # Tables
    TABLE_BORDER = "\033[38;5;250m"  # Light gray
    TABLE_HEADER = "\033[1m"
    
    # Horizontal rule
    HR_COLOR = "\033[38;5;250m"
    
    # Blockquote
    BLOCKQUOTE_BORDER = "\033[38;5;250m"
    BLOCKQUOTE_TEXT = "\033[38;5;244m"
    
    # Windows Terminal specific adjustments
    @staticmethod
    def get_color(color_code: str) -> str:
        """Get appropriate color code for current platform."""
        if Colors.IS_WINDOWS_TERMINAL:
            # Use 256-color codes for Windows Terminal
            return color_code
        else:
            # Use standard ANSI codes for other platforms
            return color_code
    
    @staticmethod
    def configure_windows_output():
        """Configure Windows output for proper UTF-8 encoding."""
        if Colors.IS_WINDOWS:
            # Only reconfigure if not already a TextIOWrapper with utf-8
            if not isinstance(sys.stdout, io.TextIOWrapper) or sys.stdout.encoding != 'utf-8':
                sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', line_buffering=True)
            if not isinstance(sys.stderr, io.TextIOWrapper) or sys.stderr.encoding != 'utf-8':
                sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', line_buffering=True)


# Heading styles
HEADING_STYLES = {
    1: Colors.HEADING1 + Colors.BOLD,
    2: Colors.HEADING2 + Colors.BOLD,
    3: Colors.HEADING3 + Colors.BOLD,
    4: Colors.HEADING4 + Colors.BOLD,
    5: Colors.HEADING5 + Colors.BOLD,
    6: Colors.HEADING6 + Colors.BOLD,
}


@dataclass
class RenderOptions:
    """Options for rendering markdown."""
    width: int = 80
    theme: str = "light"
    code_theme: str = "monokai"
    no_color: bool = False
    use_rich: bool = False


class TerminalRenderer:
    """Renders markdown to ANSI terminal output with browser-like styling."""
    
    def __init__(self, options: Optional[RenderOptions] = None):
        self.options = options or RenderOptions()
        self.md = markdown.Markdown(
            extensions=['extra', 'tables', 'fenced_code']
        )
        
        # Configure Windows output if needed
        Colors.configure_windows_output()
        
        # Initialize with Windows Terminal detection
        self.is_windows_terminal = Colors.IS_WINDOWS_TERMINAL
        self.is_windows = Colors.IS_WINDOWS
        
        # Rich console will be created lazily when needed
    
    def _get_color(self, color_code: str) -> str:
        """Get color code, respecting no_color option."""
        if self.options.no_color:
            return ""
        return Colors.get_color(color_code)
    
    def render(self, markdown_text: str) -> str:
        """Render markdown text to ANSI terminal output."""
        if self.options.use_rich:
            return self._render_with_rich(markdown_text)
        
        self.md.reset()
        html = self.md.convert(markdown_text)
        return self._html_to_ansi(html)
    
    def _html_to_ansi(self, html: str) -> str:
        """Convert HTML to ANSI terminal output."""
        # Split into lines but preserve structure
        lines = html.split('\n')
        result: List[str] = []
        
        i = 0
        while i < len(lines):
            line = lines[i].strip()
            
            # Handle code blocks
            if '<pre><code' in line:
                # Extract language if present
                code_language = ""
                match = re.search(r'class="language-(\w+)"', line)
                if match:
                    code_language = match.group(1)
                
                # Extract any content on the same line as the opening tag
                code_lines = []
                code_match = re.search(r'<pre><code[^>]*>(.*)', line)
                if code_match:
                    first_line = code_match.group(1).rstrip()
                    if first_line:
                        code_lines.append(first_line)
                
                # Collect remaining code block content
                i += 1
                while i < len(lines) and '</code></pre>' not in lines[i]:
                    code_lines.append(lines[i].rstrip())
                    i += 1
                
        # Render code block
                if code_lines:
                    code_text = '\n'.join(code_lines)
                    highlighted = self._highlight_code(code_text, code_language)
                    # Add border and styling for code blocks
                    border_color = self._get_color(Colors.TABLE_BORDER)
                    reset = Colors.RESET if border_color else ""
                    code_lines = highlighted.split('\n')
                    max_width = max(len(line) for line in code_lines) if code_lines else 0
                    width = min(max_width + 4, self.options.width)
                    
                    result.append(f"{border_color}┌{'─' * (width - 2)}┐{reset}")
                    for line in code_lines:
                        line = line[:width - 4]  # Truncate if too long
                        result.append(f"{border_color}│{reset} {line}{' ' * (width - len(line) - 4)}{border_color}│{reset}")
                    result.append(f"{border_color}└{'─' * (width - 2)}┘{reset}")
                continue
            
            # Handle headings
            if match := re.match(r'<h([1-6])>(.*?)</h\1>', line, re.DOTALL):
                level = int(match.group(1))
                content = match.group(2)
                result.append(self._render_heading(level, content))
            
            # Handle blockquote
            elif line.startswith('<blockquote>'):
                bq_lines = []
                i += 1
                while i < len(lines) and '</blockquote>' not in lines[i]:
                    bq_lines.append(lines[i].strip())
                    i += 1
                result.append(self._render_blockquote(bq_lines))
            
            # Handle table
            elif '<table>' in line:
                table_lines = []
                i += 1
                while i < len(lines) and '</table>' not in lines[i]:
                    table_lines.append(lines[i])
                    i += 1
                result.append(self._render_table(table_lines))
            
            # Handle list
            elif line.startswith('<ul>') or line.startswith('<ol>'):
                i += 1  # Skip opening tag
                continue
            
            # Handle list items
            elif '<li>' in line:
                match = re.search(r'<li>(.*?)</li>', line)
                if match:
                    content = match.group(1)
                    marker_color = self._get_color(Colors.LIST_MARKER)
                    reset = Colors.RESET if marker_color else ""
                    result.append(f"  {marker_color}•{reset} {self._render_inline(content)}")
            
            # Handle horizontal rule
            elif '<hr />' in line or '<hr/>' in line:
                hr_color = self._get_color(Colors.HR_COLOR)
                reset = Colors.RESET if hr_color else ""
                result.append(hr_color + "─" * self.options.width + reset)
            
            # Handle paragraph
            elif line.startswith('<p>') and line.endswith('</p>'):
                content = line[3:-4]
                result.append(self._render_inline(content))
            
            # Handle non-paragraph content
            elif line and not line.startswith('<'):
                result.append(self._render_inline(line))
            
            i += 1
        
        return '\n'.join(result)
    
    def _render_heading(self, level: int, content: str) -> str:
        """Render a heading with browser-like styling."""
        content = re.sub(r'<[^>]+>', '', content)
        content = content.strip()
        
        # Use appropriate color
        heading_color = self._get_color(HEADING_STYLES.get(level, HEADING_STYLES[6]))
        underline_color = self._get_color(Colors.HR_COLOR)
        
        underline = ""
        if level == 1:
            underline = "\n" + underline_color + "═" * min(len(content), self.options.width) + (Colors.RESET if underline_color else "")
        elif level == 2:
            underline = "\n" + underline_color + "─" * min(len(content), self.options.width) + (Colors.RESET if underline_color else "")
        
        return f"{heading_color}{content}{underline}"
    
    def _render_blockquote(self, lines: List[str]) -> str:
        """Render a blockquote with browser-like styling."""
        # Process each line
        processed_lines = []
        for line in lines:
            line = line.strip()
            if line.startswith('<p>'):
                line = line[3:]
            if line.endswith('</p>'):
                line = line[:-4]
            line = self._render_inline(line)
            if line.strip():
                processed_lines.append(line)
        
        result = []
        border_color = self._get_color(Colors.BLOCKQUOTE_BORDER)
        text_color = self._get_color(Colors.BLOCKQUOTE_TEXT)
        reset = Colors.RESET if (border_color or text_color) and not self.options.no_color else ""
        
        # Add blockquote border with proper box drawing
        result.append(f"{border_color}┌{'─' * (self.options.width - 2)}┐{reset}")
        for line in processed_lines:
            line = line[:self.options.width - 4]  # Truncate if too long
            result.append(f"{border_color}│{reset} {text_color}{line}{' ' * (self.options.width - len(line) - 4)}{border_color}│{reset}")
        result.append(f"{border_color}└{'─' * (self.options.width - 2)}┘{reset}")
        
        return '\n'.join(result)
    
    def _render_table(self, lines: List[str]) -> str:
        """Render a table with browser-like styling."""
        if not lines:
            return ""
        
        # Parse table rows - the HTML is multi-line
        full_html = '\n'.join(lines)
        
        # Find all rows
        rows = []
        
        # Get header
        thead_match = re.search(r'<thead>(.*?)</thead>', full_html, re.DOTALL)
        if thead_match:
            thead = thead_match.group(1)
            header_cells = re.findall(r'<th>(.*?)</th>', thead, re.DOTALL)
            if header_cells:
                clean_cells = [re.sub(r'<[^>]+>', '', c).strip() for c in header_cells]
                rows.append((True, clean_cells))
        
        # Get body
        tbody_match = re.search(r'<tbody>(.*?)</tbody>', full_html, re.DOTALL)
        if tbody_match:
            tbody = tbody_match.group(1)
            tr_matches = re.findall(r'<tr>(.*?)</tr>', tbody, re.DOTALL)
            for tr in tr_matches:
                cells = re.findall(r'<td>(.*?)</td>', tr, re.DOTALL)
                if cells:
                    clean_cells = [re.sub(r'<[^>]+>', '', c).strip() for c in cells]
                    rows.append((False, clean_cells))
        
        if not rows:
            return ""
        
        # Calculate column widths
        num_cols = max(len(row[1]) for row in rows) if rows else 0
        col_widths = [0] * num_cols
        for is_header, cells in rows:
            for i, cell in enumerate(cells):
                if i < len(col_widths):
                    col_widths[i] = max(col_widths[i], len(cell))
        
        # Render table with box drawing
        result = []
        
        # Header separator
        if rows and rows[0][0]:  # First row is header
            header_sep = ""
            for i, width in enumerate(col_widths):
                sep_char = "┬" if i < len(col_widths) - 1 else "┐"
                header_sep += '─' * (width + 2) + sep_char
            result.append(header_sep)
        
        for is_header, cells in rows:
            # Render cells
            row_cells = []
            for i, cell in enumerate(cells):
                if i < len(col_widths):
                    width = col_widths[i]
                    style = self._get_color(Colors.TABLE_HEADER) if is_header else ""
                    reset = Colors.RESET if style else ""
                    cell_str = f"{style}{cell:<{width}}{reset}" if is_header else f"{cell:<{width}}"
                    row_cells.append(cell_str)
            
            row_str = "│ " + " │ ".join(row_cells) + " │"
            result.append(row_str)
            
            # Add separator after header
            if is_header:
                sep_cells = []
                for width in col_widths:
                    sep_cells.append('─' * width)
                sep = "├─" + "─┼─".join(sep_cells) + "─┤"
                result.append(sep)
        
        # Bottom border
        bottom_sep = ""
        for i, width in enumerate(col_widths):
            sep_char = "┴" if i < len(col_widths) - 1 else "┘"
            bottom_sep += '─' * (width + 2) + sep_char
        result.append(bottom_sep)
        
        return '\n'.join(result)
    
    def _render_inline(self, content: str) -> str:
        """Render inline HTML elements."""
        reset = Colors.RESET if not self.options.no_color else ""
        
        # Handle strong/bold
        content = re.sub(r'<strong>(.*?)</strong>', 
                        rf"{self._get_color(Colors.STRONG)}\1{reset}", 
                        content)
        
        # Handle emphasis/italic  
        content = re.sub(r'<em>(.*?)</em>', 
                        rf"{self._get_color(Colors.EMPHASIS)}\1{reset}", 
                        content)
        
        # Handle links
        content = re.sub(r'<a href="([^"]+)">(.*?)</a>', 
                        rf"{self._get_color(Colors.LINK)}\2{reset} (\1)", 
                        content)
        
        # Handle code
        content = re.sub(r'<code>(.*?)</code>', 
                        rf"{self._get_color(Colors.CODE)}\1{reset}", 
                        content)
        
        return content
    
    def _highlight_code(self, code: str, language: str) -> str:
        """Highlight code using Pygments."""
        code = code.rstrip('\n')
        
        try:
            if language:
                lexer = get_lexer_by_name(language)
            else:
                lexer = TextLexer()
        except:
            lexer = TextLexer()
        
        formatter = TerminalFormatter(
            bg=self.options.theme,
            style=self.options.code_theme
        )
        result = highlight(code, lexer, formatter)
        
        # Remove ANSI codes if no_color is enabled
        if self.options.no_color:
            import re
            ansi_escape = re.compile(r'\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])')
            result = ansi_escape.sub('', result)
        
        return result
    
    def _render_with_rich(self, markdown_text: str) -> str:
        """Render markdown using Rich for pretty terminal output."""
        # Create Rich console if not already created
        if not hasattr(self, 'rich_console'):
            self.rich_console = Console(
                force_terminal=True,
                force_jupyter=False,
                width=self.options.width,
                record=True
            )
        
        # Create Rich markdown renderer
        rich_md = RichMarkdown(
            markdown_text,
            code_theme=self.options.code_theme
        )
        
        # Render to string
        self.rich_console.print(rich_md)
        output = self.rich_console.export_text()
        self.rich_console.clear()
        
        return output


def render_markdown(markdown_text: str, width: int = 80, theme: str = "light", use_rich: bool = False) -> str:
    """Convenience function to render markdown."""
    options = RenderOptions(width=width, theme=theme, use_rich=use_rich)
    renderer = TerminalRenderer(options)
    return renderer.render(markdown_text)
