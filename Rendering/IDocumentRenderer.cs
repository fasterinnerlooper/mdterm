using AngleSharp.Dom;

namespace MdTerm.Rendering;

public interface IDocumentRenderer
{
    void RenderBody(IDocument document);
}
