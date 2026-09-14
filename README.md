# mdterm

Render Markdown files as styled terminal output.

## Highlights

- Parses Markdown with Markdig and AngleSharp.
- Renders headings, paragraphs, links, code blocks, tables, and lists.
- Generates a table of contents from document headings.
- Uses a pager when available so long output is easier to browse.
- Ships release artifacts for Windows, Linux, and macOS.

## Screenshots

### `mdterm sample.md`

![mdterm rendering overview](docs/images/render-overview.png)

![mdterm rendering features](docs/images/render-features.png)

### `mdterm --help`

![mdterm help output](docs/images/help.png)

## Requirements

- .NET SDK 9.0 to build from source

## Installation

### Download a release

Download a prebuilt archive from the [latest release](https://github.com/fasterinnerlooper/mdterm/releases/latest).

### Package manager commands

These are the package manager targets reflected by the repository's release automation and latest successful release workflow:

```bash
winget install fasterinnerlooper.mdterm
choco install mdterm
scoop bucket add fasterinnerlooper https://github.com/fasterinnerlooper/scoop-bucket
scoop install mdterm
paru -S mdterm
# or
yay -S mdterm
```

## Build and test

The CI workflow builds mdterm across `win-x64`, `linux-x64`, `osx-x64`, and `osx-arm64`. The main commands used there are:

```bash
dotnet build MdTerm.csproj -c Release -r linux-x64
dotnet test -c Release --no-build
```

For local development:

```bash
dotnet restore
dotnet build MdTerm.csproj -c Release
dotnet run --project MdTerm.csproj -- sample.md
```

## Usage

```bash
mdterm document.md
mdterm --mode ascii document.md
mdterm --list-styles
mdterm --help
```

## Project layout

- `Cli/` command-line parsing and help output
- `Parsing/` Markdown-to-HTML parsing and table-of-contents generation
- `Rendering/` terminal renderers
- `Fonts/` embedded FIGlet fonts used for heading rendering
- `packaging/` package manifests for release distribution

## Privacy

mdterm is a local CLI tool. It reads a Markdown file from disk and renders it in your terminal. The application code in this repository does not include telemetry, analytics, or outbound network calls for document rendering.

## Security

If you discover a vulnerability, use the private reporting process described in [SECURITY.md](SECURITY.md).

## License

This project is licensed under the [MIT License](LICENSE).
