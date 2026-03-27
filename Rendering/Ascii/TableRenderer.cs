using AngleSharp.Dom;
using Spectre.Console;

namespace MdTerm.Rendering.Ascii;

public sealed class TableRenderer : INodeRenderer
{
    public int Priority => 75;

    public bool CanRender(INode node)
        => node is IElement el && el.TagName == "TABLE";

    public void Render(INode node)
    {
        var el = (IElement)node;
        var table = new Table();

        var header = el.QuerySelector("thead");
        if (header != null)
        {
            foreach (var th in header.QuerySelectorAll("th"))
                table.AddColumn(th.TextContent);
        }

        var rows = el.QuerySelectorAll("tbody tr");
        foreach (var row in rows)
        {
            var cells = row.QuerySelectorAll("td").Select(td => td.TextContent).ToArray();
            table.AddRow(cells);
        }

        AnsiConsole.Write(table);
    }
}
