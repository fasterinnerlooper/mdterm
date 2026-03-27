using AngleSharp.Dom;
using Spectre.Console;

namespace MdTerm.Rendering.Ascii;

public sealed class ListRenderer : INodeRenderer
{
    public int Priority => 80;

    public bool CanRender(INode node)
        => node is IElement el && (el.TagName == "UL" || el.TagName == "OL");

    public void Render(INode node)
    {
        var el = (IElement)node;
        var isOrdered = el.TagName == "OL";
        int i = 1;

        foreach (var li in el.QuerySelectorAll("li"))
        {
            var text = li.TextContent.EscapeMarkup();
            if (isOrdered)
                AnsiConsole.MarkupLine($"  [cyan]{i}[/]. {text}");
            else
                AnsiConsole.MarkupLine($"  [cyan]\u2022[/] {text}");
            i++;
        }

        AnsiConsole.WriteLine();
    }
}
