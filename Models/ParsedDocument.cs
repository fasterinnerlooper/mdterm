using AngleSharp.Dom;

namespace MdTerm.Models;

public sealed class ParsedDocument
{
    public required IDocument Document { get; init; }
    public required IReadOnlyList<TocEntry> TableOfContents { get; init; }
}

public sealed class TocEntry
{
    public required string Anchor { get; init; }
    public required string Text { get; init; }
}
