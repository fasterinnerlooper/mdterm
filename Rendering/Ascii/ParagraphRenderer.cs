using AngleSharp.Dom;
using Spectre.Console;

namespace MdTerm.Rendering.Ascii;

public sealed class ParagraphRenderer : INodeRenderer
{
    public int Priority => 90;

    public bool CanRender(INode node)
        => node is IElement el && el.TagName == "P";

    public void Render(INode node)
    {
        AnsiConsole.MarkupLine(((IElement)node).TextContent);
    }
}
