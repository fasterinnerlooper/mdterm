class Mdterm < Formula
  desc "Render Markdown files beautifully in the terminal"
  homepage "https://github.com/fasterinnerlooper/mdterm"
  sha256 ""
  license "MIT"
  url "https://github.com/fasterinnerlooper/mdterm/archive/refs/tags/v1.2.9.tar.gz"

  depends_on "dotnet@9" => :build

  def install
    runtime_id = if OS.mac? && Hardware::CPU.arm?
      "osx-arm64"
    elsif OS.mac?
      "osx-x64"
    elsif Hardware::CPU.arm?
      "linux-arm64"
    else
      "linux-x64"
    end

    system "dotnet", "publish", "MdTerm.csproj",
      "-c", "Release",
      "--self-contained", "false",
      "-r", runtime_id,
      "-o", "./bin/install"

    bin.install "bin/install/mdterm"
  end

  test do
    assert_match "mdterm", shell_output("#{bin}/mdterm --version")
  end
end
