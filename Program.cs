using MdTerm.Cli;
using MdTerm.Models;
using MdTerm.Parsing;
using MdTerm.Rendering;
using MdTerm.Rendering.Ascii;
using Spectre.Console;

namespace MdTerm;

internal class Program
{
    static async Task Main(string[] args)
    {
        var parser = new CommandLineParser();
        var options = parser.Parse(args);

        if (options.ShowHelp)
        {
            HelpWriter.ShowHelp();
            return;
        }

        if (options.ShowVersion)
        {
            HelpWriter.ShowVersion(typeof(Program));
            return;
        }

        if (options.ListStyles)
        {
            HelpWriter.ListStyles();
            return;
        }

        if (options.FilePath == null)
        {
            HelpWriter.ShowHelp();
            return;
        }

        if (!File.Exists(options.FilePath))
        {
            AnsiConsole.MarkupLine($"[red]File not found: {options.FilePath}[/]");
            return;
        }

        var markdown = await File.ReadAllTextAsync(options.FilePath);

        IMarkdownParser mdParser = new HtmlMarkdownParser();
        var parsed = await mdParser.ParseAsync(markdown);

        RenderTableOfContents(parsed.TableOfContents);

        var nodeRenderers = new INodeRenderer[]
        {
            new HeadingRenderer(),
            new CodeBlockRenderer(),
            new ParagraphRenderer(),
            new TableRenderer(),
            new ListRenderer(),
            new TextRenderer()
        };

        IDocumentRenderer renderer = new AsciiDocumentRenderer(nodeRenderers);
        renderer.RenderBody(parsed.Document);
    }

    private static void RenderTableOfContents(IReadOnlyList<TocEntry> toc)
    {
        AnsiConsole.MarkupLine("[underline]Table of Contents[/]");
        foreach (var entry in toc)
        {
            AnsiConsole.MarkupLine($"[bold]{entry.Anchor}[/] {entry.Text}");
        }
        AnsiConsole.WriteLine();
    }
}
