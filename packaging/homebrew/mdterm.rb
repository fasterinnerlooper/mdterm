class Mdterm < Formula
  desc "Render Markdown files beautifully in the terminal"
  homepage "https://github.com/fasterinnerlooper/mdterm"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/fasterinnerlooper/mdterm/releases/download/vVERSION_PLACEHOLDER/mdterm-osx-arm64.tar.gz"
      sha256 "SHA256_ARM64_PLACEHOLDER"
    end

    on_intel do
      url "https://github.com/fasterinnerlooper/mdterm/releases/download/vVERSION_PLACEHOLDER/mdterm-osx-x64.tar.gz"
      sha256 "SHA256_X64_PLACEHOLDER"
    end
  end

  on_linux do
    url "https://github.com/fasterinnerlooper/mdterm/releases/download/vVERSION_PLACEHOLDER/mdterm-linux-x64.tar.gz"
    sha256 "SHA256_LINUX_PLACEHOLDER"
  end

  def install
    bin.install "mdterm"
  end

  test do
    assert_match "mdterm", shell_output("#{bin}/mdterm --version")
  end
end
