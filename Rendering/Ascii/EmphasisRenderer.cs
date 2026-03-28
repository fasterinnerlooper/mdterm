using AngleSharp.Dom;

namespace MdTerm.Rendering.Ascii;

public sealed class EmphasisRenderer : IInlineRenderer
{
    public bool CanRender(INode node)
        => node is IElement el && (el.TagName == "EM" || el.TagName == "I");

    public string Render(INode node, Func<INode, string> renderChild)
    {
        var content = string.Concat(node.ChildNodes.Select(renderChild));
        return $"[italic]{content}[/]";
    }
}
