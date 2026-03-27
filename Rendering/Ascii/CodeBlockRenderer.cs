using AngleSharp.Dom;
using Spectre.Console;

namespace MdTerm.Rendering.Ascii;

public sealed class CodeBlockRenderer : INodeRenderer
{
    public int Priority => 85;

    public bool CanRender(INode node)
        => node is IElement el && el.TagName == "PRE";

    public void Render(INode node)
    {
        var el = (IElement)node;
        var panel = new Panel(el.TextContent)
            .Border(BoxBorder.Rounded)
            .Header("[bold]Code[/]");
        AnsiConsole.Write(panel);
    }
}
