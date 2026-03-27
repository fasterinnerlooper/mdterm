# MdTerm Rendering Sample

This document demonstrates all supported rendering features.

## Headings

### Third-Level Heading

#### Fourth-Level Heading

##### Fifth-Level Heading

###### Sixth-Level Heading

## Text Formatting

This is a paragraph with **bold text** and *italic text*.

Another paragraph demonstrating inline links: visit the [AngleSharp docs](https://anglesharp.github.io) for parser details, or check [Spectre.Console](https://spectreconsole.net/) for the rendering library.

## Code Blocks

```csharp
using Spectre.Console;

AnsiConsole.MarkupLine("[bold cyan]Hello, MdTerm![/]");
```

```python
def greet(name: str) -> str:
    return f"Hello, {name}!"
```

## Tables

| Feature       | ASCII Mode | Pixel Mode |
|---------------|------------|------------|
| Headings      | Figlet     | True       |
| Colors        | ANSI       | True       |
| Code Blocks   | Panel      | Highlight  |
| Tables        | Rounded    | Styled     |
| Portability   | High       | Low        |

## Unordered List

- First item
- Second item
- Third item

## Ordered List

1. Install dependencies
2. Build the project
3. Run the application

## Multiple Links

Reference links inline: [GitHub](https://github.com), [NuGet](https://nuget.org), and the [Markdown Guide](https://www.markdownguide.org).
