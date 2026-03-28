using AngleSharp.Dom;

namespace MdTerm.Rendering.Ascii;

public interface IInlineRenderer
{
    bool CanRender(INode node);
    string Render(INode node, Func<INode, string> renderChild);
}
