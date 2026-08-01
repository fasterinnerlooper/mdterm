using MdTerm.Cli;
using MdTerm.Models;
using MdTerm.Parsing;
using MdTerm.Rendering;
using MdTerm.Rendering.Ascii;
using Spectre.Console;
using System.Diagnostics;

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

        // Render output through pager where available so long content is navigable
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

        WriteWithPager(() =>
        {
            RenderTableOfContents(parsed.TableOfContents);
            renderer.RenderBody(parsed.Document);
        });
    }

    private static void WriteWithPager(Action renderAction)
    {
        // Capture Console output while rendering
        var originalOut = Console.Out;
        var sw = new StringWriter();
        Console.SetOut(sw);
        try
        {
            renderAction();
        }
        finally
        {
            Console.SetOut(originalOut);
        }

        var content = sw.ToString();
        if (string.IsNullOrEmpty(content))
            return;

        // Choose pager depending on OS. Try to start it; fall back to writing to stdout.
        string pager = null;
        string pagerArgs = null;
        if (OperatingSystem.IsWindows())
        {
            pager = "more";
        }
        else
        {
            pager = "less";
            pagerArgs = "-R"; // allow raw ANSI sequences
        }

        try
        {
            var psi = new ProcessStartInfo
            {
                FileName = pager,
                Arguments = pagerArgs ?? string.Empty,
                RedirectStandardInput = true,
                UseShellExecute = false,
                CreateNoWindow = true
            };

            using var proc = Process.Start(psi);
            if (proc != null)
            {
                using var writer = proc.StandardInput;
                writer.Write(content);
                writer.Close();
                proc.WaitForExit();
                return;
            }
        }
        catch
        {
            // ignore and fall through to write to console
        }

        Console.Write(content);
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
