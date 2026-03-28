using AngleSharp.Dom;

namespace MdTerm.Rendering.Ascii;

public sealed class StrongRenderer : IInlineRenderer
{
    public bool CanRender(INode node)
        => node is IElement el && (el.TagName == "STRONG" || el.TagName == "B");

    public string Render(INode node, Func<INode, string> renderChild)
    {
        var content = string.Concat(node.ChildNodes.Select(renderChild));
        return $"[bold]{content}[/]";
    }
}
