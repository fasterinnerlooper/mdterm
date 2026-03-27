using AngleSharp.Dom;
using Spectre.Console;

namespace MdTerm.Rendering.Ascii;

public sealed class HeadingRenderer : INodeRenderer
{
    public int Priority => 100;

    public bool CanRender(INode node)
        => node is IElement el && el.TagName.StartsWith('H');

    public void Render(INode node)
    {
        var el = (IElement)node;
        var figlet = new FigletText(el.TextContent)
        {
            Color = Color.Cyan
        };
        AnsiConsole.Write(figlet);
    }
}
