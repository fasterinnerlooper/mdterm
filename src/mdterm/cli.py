"""CLI for mdterm - markdown terminal viewer."""

import sys
import os
from pathlib import Path
from typing import Optional

import typer

from mdterm.renderer import TerminalRenderer, RenderOptions, Colors

app = typer.Typer(
    name="mdterm",
    help="Render markdown in terminal with browser-like styling",
    add_completion=False,
)


def get_terminal_width() -> int:
    """Get the terminal width."""
    try:
        import shutil
        width = shutil.get_terminal_size().columns
        return max(width, 40)
    except:
        return 80


@app.command()
def main(
    file: Optional[Path] = typer.Argument(
        None,
        help="Markdown file to render",
        exists=True,
        file_okay=True,
        dir_okay=False,
    ),
    stdin: bool = typer.Option(
        False,
        "--stdin",
        "-s",
        help="Read from stdin instead of a file",
    ),
    width: Optional[int] = typer.Option(
        None,
        "--width",
        "-w",
        help="Terminal width for wrapping (default: auto-detect)",
    ),
    theme: str = typer.Option(
        "light",
        "--theme",
        "-t",
        help="Color theme: light or dark",
        case_sensitive=False,
    ),
    code_theme: str = typer.Option(
        "monokai",
        "--code-theme",
        help="Code syntax highlighting theme",
    ),
    no_color: bool = typer.Option(
        False,
        "--no-color",
        help="Disable color output",
    ),
):
    """
    Render markdown file in terminal with browser-like styling.
    
    Examples:
    
        mdterm README.md
    
        cat README.md | mdterm --stdin
    
        mdterm --theme dark README.md
    """
    # Get input content
    if stdin:
        content = sys.stdin.read()
    elif file:
        content = file.read_text(encoding='utf-8')
    else:
        typer.echo("Error: Please provide a file or use --stdin", err=True)
        raise typer.Exit(code=1)
    
    # Get terminal width
    terminal_width = width or get_terminal_width()
    
    # Create renderer
    options = RenderOptions(
        width=terminal_width,
        theme=theme,
        code_theme=code_theme,
    )
    renderer = TerminalRenderer(options)
    
    # Render markdown
    output = renderer.render(content)
    
    # Output
    if no_color:
        import re
        ansi_escape = re.compile(r'\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])')
        output = ansi_escape.sub('', output)
    
    print(output)


if __name__ == "__main__":
    app()
