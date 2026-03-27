namespace MdTerm.Models;

public sealed class CliOptions
{
    public bool ShowHelp { get; init; }
    public bool ShowVersion { get; init; }
    public bool ListStyles { get; init; }
    public RenderMode Mode { get; init; } = RenderMode.Ascii;
    public string? FilePath { get; init; }
}
