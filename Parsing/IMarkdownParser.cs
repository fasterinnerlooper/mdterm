using MdTerm.Models;

namespace MdTerm.Parsing;

public interface IMarkdownParser
{
    Task<ParsedDocument> ParseAsync(string markdown);
}
