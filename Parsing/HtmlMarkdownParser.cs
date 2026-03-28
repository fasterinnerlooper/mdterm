using AngleSharp;
using AngleSharp.Dom;
using Markdig;
using MdTerm.Models;

namespace MdTerm.Parsing;

public sealed class HtmlMarkdownParser : IMarkdownParser
{
    private readonly MarkdownPipeline _pipeline = new MarkdownPipelineBuilder()
        .UseAdvancedExtensions()
        .UseEmojiAndSmiley()
        .Build();

    public async Task<ParsedDocument> ParseAsync(string markdown)
    {
        var html = Markdown.ToHtml(markdown, _pipeline);

        var context = BrowsingContext.New(Configuration.Default);
        var document = await context.OpenAsync(req => req.Content(html));

        var toc = BuildTableOfContents(document);

        return new ParsedDocument
        {
            Document = document,
            TableOfContents = toc
        };
    }

    private static IReadOnlyList<TocEntry> BuildTableOfContents(IDocument document)
    {
        var headings = document.QuerySelectorAll("h1, h2, h3, h4, h5, h6")
            .OfType<IElement>()
            .ToList();

        var toc = new List<TocEntry>();
        var counters = new int[6];

        foreach (var h in headings)
        {
            var level = int.Parse(h.TagName.Substring(1)) - 1;
            counters[level]++;
            for (int i = level + 1; i < counters.Length; i++)
                counters[i] = 0;

            var anchor = string.Join(".", counters.Take(level + 1).Where(c => c > 0));
            h.SetAttribute("id", anchor);

            toc.Add(new TocEntry
            {
                Anchor = anchor,
                Text = h.TextContent
            });
        }

        return toc;
    }
}
