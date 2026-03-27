using AngleSharp.Dom;

namespace MdTerm.Rendering.Ascii;

public sealed class AsciiDocumentRenderer : IDocumentRenderer
{
    private readonly IReadOnlyList<INodeRenderer> _renderers;

    public AsciiDocumentRenderer(IEnumerable<INodeRenderer> renderers)
    {
        _renderers = renderers.OrderByDescending(r => r.Priority).ToList();
    }

    public void RenderBody(IDocument document)
    {
        foreach (var node in document.Body!.ChildNodes)
        {
            RenderNode(node);
        }
    }

    private void RenderNode(INode node)
    {
        var renderer = _renderers.FirstOrDefault(r => r.CanRender(node));
        if (renderer != null)
        {
            renderer.Render(node);
        }
        else
        {
            foreach (var child in node.ChildNodes)
                RenderNode(child);
        }
    }
}
