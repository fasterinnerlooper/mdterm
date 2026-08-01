using System.Reflection;
using AngleSharp.Dom;
using Spectre.Console;

namespace MdTerm.Rendering.Ascii;

public sealed class HeadingRenderer : INodeRenderer
{
    public int Priority => 100;

    private readonly FigletFont _standardFont;
    private readonly FigletFont _smallFont;
    private readonly FigletFont _miniFont;

    public HeadingRenderer()
    {
        var asm = Assembly.GetExecutingAssembly();
        _standardFont = FigletFont.Load(asm.GetManifestResourceStream("MdTerm.Fonts.standard.flf")!);
        _smallFont = FigletFont.Load(asm.GetManifestResourceStream("MdTerm.Fonts.small.flf")!);
        _miniFont = FigletFont.Load(asm.GetManifestResourceStream("MdTerm.Fonts.mini.flf")!);
    }

    public bool CanRender(INode node)
        => node is IElement el && el.TagName.StartsWith('H')
           && int.TryParse(el.TagName.Substring(1), out _);

    public void Render(INode node)
    {
        var el = (IElement)node;
        var level = int.Parse(el.TagName.Substring(1));
        var text = el.TextContent.EscapeMarkup();

        switch (level)
        {
            case 1:
                AnsiConsole.Write(new FigletText(_standardFont, text) { Color = Color.Blue });
                AnsiConsole.Write(new Rule().RuleStyle("white on blue").DoubleBorder());
                break;
            case 2:
                AnsiConsole.WriteLine();
                AnsiConsole.Write(new FigletText(_smallFont, text) { Color = Color.Blue });
                break;
            case 3:
                AnsiConsole.Write(new FigletText(_miniFont, text) { Color = Color.Blue });
                break;
            case 4:
                AnsiConsole.MarkupLine($"[bold]{text}[/]");
                AnsiConsole.MarkupLine("[dim]" + new string('\u2500', Math.Min(text.Length, 40)) + "[/]");
                break;
            case 5:
                AnsiConsole.MarkupLine($"[bold]  {text}[/]");
                break;
            case 6:
                AnsiConsole.MarkupLine($"[dim]  {text}[/]");
                break;
        }
    }
}
