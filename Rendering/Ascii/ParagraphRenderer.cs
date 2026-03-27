using System.Text;
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
        var el = (IElement)node;
        var sb = new StringBuilder("  ");

        foreach (var child in el.ChildNodes)
        {
            if (child is IText textNode)
            {
                sb.Append(textNode.Text.EscapeMarkup());
            }
            else if (child is IElement childEl && childEl.TagName == "A")
            {
                var linkText = childEl.TextContent.EscapeMarkup();
                var href = childEl.GetAttribute("href") ?? "";
                sb.Append($"[underline blue]{linkText}[/] [dim]({href.EscapeMarkup()})[/]");
            }
            else if (child is IElement inlineEl)
            {
                sb.Append(inlineEl.TextContent.EscapeMarkup());
            }
        }

        AnsiConsole.MarkupLine(sb.ToString());
        AnsiConsole.WriteLine();
    }
}
