using AngleSharp.Dom;

namespace MdTerm.Rendering;

public interface INodeRenderer
{
    int Priority { get; }
    bool CanRender(INode node);
    void Render(INode node);
}
