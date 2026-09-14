# Contributing to mdterm

Thanks for your interest in contributing.

## Development requirements

- .NET SDK 9.0

## Local development

```bash
dotnet restore
dotnet build MdTerm.csproj -c Release
dotnet test -c Release --no-build
dotnet run --project MdTerm.csproj -- sample.md
```

## CI commands

The main CI workflow uses these commands:

```bash
dotnet build MdTerm.csproj -c Release -r <runtime>
dotnet test -c Release --no-build
```

It validates the project on:

- `win-x64`
- `linux-x64`
- `osx-x64`
- `osx-arm64`

## Repository layout

- `Cli/` command-line parsing and help output
- `Parsing/` Markdown-to-HTML parsing and table-of-contents generation
- `Rendering/` terminal renderers
- `Fonts/` embedded FIGlet fonts used for heading rendering
- `packaging/` package manifests for release distribution

## Release packaging

Release automation renders packaging templates and publishes packages for:

- Winget
- Scoop
- Chocolatey
- AUR

Snap packaging is present in the repository and only publishes when the required credentials are configured in CI.
