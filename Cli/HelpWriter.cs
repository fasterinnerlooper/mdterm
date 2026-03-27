using Spectre.Console;

namespace MdTerm.Cli;

public static class HelpWriter
{
    public static void ShowHelp()
    {
        AnsiConsole.MarkupLine("[bold]mdterm[/] - Markdown terminal renderer");
        AnsiConsole.WriteLine();
        AnsiConsole.MarkupLine("[bold]Usage:[/] mdterm [[options]] <file>");
        AnsiConsole.WriteLine();
        AnsiConsole.MarkupLine("[bold]Options:[/]");
        AnsiConsole.MarkupLine("  --help, -h          Show this help");
        AnsiConsole.MarkupLine("  --version, -v       Show version information");
        AnsiConsole.MarkupLine("  --list-styles       List available rendering styles");
        AnsiConsole.MarkupLine("  --mode <style>      Set rendering style (ascii)");
        AnsiConsole.WriteLine();
        AnsiConsole.MarkupLine("[bold]Examples:[/]");
        AnsiConsole.MarkupLine("  mdterm document.md");
        AnsiConsole.MarkupLine("  mdterm --mode ascii document.md");
    }

    public static void ShowVersion(Type programType)
    {
        var version = programType.Assembly.GetName().Version;
        AnsiConsole.MarkupLine($"[bold]mdterm[/] version {version}");
    }

    public static void ListStyles()
    {
        AnsiConsole.MarkupLine("[bold]Available rendering styles:[/]");
        AnsiConsole.MarkupLine("  [cyan]ascii[/] - ASCII-art using Spectre.Console markup");
    }
}
