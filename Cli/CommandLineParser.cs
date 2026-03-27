using MdTerm.Models;

namespace MdTerm.Cli;

public sealed class CommandLineParser
{
    public CliOptions Parse(string[] args)
    {
        bool showHelp = false;
        bool showVersion = false;
        bool listStyles = false;
        RenderMode mode = RenderMode.Ascii;
        string? filePath = null;

        for (int i = 0; i < args.Length; i++)
        {
            var arg = args[i];

            if (arg == "--help" || arg == "-h")
                showHelp = true;
            else if (arg == "--version" || arg == "-v")
                showVersion = true;
            else if (arg == "--list-styles")
                listStyles = true;
            else if (arg == "--mode" && i + 1 < args.Length)
                mode = ParseMode(args[++i]);
            else if (arg.StartsWith("--mode="))
            {
                var parts = arg.Split('=', 2);
                if (parts.Length == 2)
                    mode = ParseMode(parts[1]);
            }
            else if (!arg.StartsWith("-"))
                filePath = arg;
        }

        return new CliOptions
        {
            ShowHelp = showHelp,
            ShowVersion = showVersion,
            ListStyles = listStyles,
            Mode = mode,
            FilePath = filePath
        };
    }

    private static RenderMode ParseMode(string value)
    {
        if (Enum.TryParse<RenderMode>(value, ignoreCase: true, out var mode))
            return mode;

        Console.Error.WriteLine($"Unknown mode: {value}");
        return RenderMode.Ascii;
    }
}
