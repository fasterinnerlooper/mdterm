using AngleSharp.Dom;
using Spectre.Console;

namespace MdTerm.Rendering.Ascii;

public sealed class TextRenderer : INodeRenderer
{
    public int Priority => 10;

    public bool CanRender(INode node)
        => node is IText;

    public void Render(INode node)
    {
        var trimmed = ((IText)node).Text.Trim();
        if (!string.IsNullOrEmpty(trimmed))
            AnsiConsole.MarkupLine(trimmed);
    }
}
