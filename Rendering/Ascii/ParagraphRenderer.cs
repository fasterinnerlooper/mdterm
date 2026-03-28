using System.Text;
using AngleSharp.Dom;
using Spectre.Console;

namespace MdTerm.Rendering.Ascii;

public sealed class ParagraphRenderer : INodeRenderer
{
    public int Priority => 90;

    private readonly IReadOnlyList<IInlineRenderer> _inlineRenderers;

    public ParagraphRenderer()
    {
        _inlineRenderers = new IInlineRenderer[]
        {
            new StrongRenderer(),
            new EmphasisRenderer()
        };
    }

    public bool CanRender(INode node)
        => node is IElement el && el.TagName == "P";

    public void Render(INode node)
    {
        var el = (IElement)node;
        var sb = new StringBuilder("  ");

        foreach (var child in el.ChildNodes)
        {
            sb.Append(RenderInlineNode(child));
        }

        AnsiConsole.MarkupLine(sb.ToString());
        AnsiConsole.WriteLine();
    }

    private string RenderInlineNode(INode node)
    {
        if (node is IText textNode)
            return textNode.Text.EscapeMarkup();

        if (node is IElement childEl && childEl.TagName == "A")
        {
            var linkText = childEl.TextContent.EscapeMarkup();
            var href = childEl.GetAttribute("href") ?? "";
            return $"[link={href.EscapeMarkup()}][underline blue]{linkText}[/][/]";
        }

        if (node is IElement inlineEl)
        {
            var renderer = _inlineRenderers.FirstOrDefault(r => r.CanRender(node));
            if (renderer != null)
                return renderer.Render(node, RenderInlineNode);

            return string.Concat(inlineEl.ChildNodes.Select(RenderInlineNode));
        }

        return "";
    }
}
