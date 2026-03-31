class Mdterm < Formula
  desc "Render Markdown files beautifully in the terminal"
  homepage "https://github.com/fasterinnerlooper/mdterm"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/fasterinnerlooper/mdterm/releases/download/v1.1.5/mdterm-osx-arm64.tar.gz"
      sha256 "7a027d9c0cee0f2aa766309f9dd891381cae72fa249c70938614fc243e2f09a2"
    end

    on_intel do
      url "https://github.com/fasterinnerlooper/mdterm/releases/download/v1.1.5/mdterm-osx-x64.tar.gz"
      sha256 "3e576577441fd87f814b34ae39bb2f1ecc41cc594af72253f1b616de532d616c"
    end
  end

  on_linux do
    url "https://github.com/fasterinnerlooper/mdterm/releases/download/v1.1.5/mdterm-linux-x64.tar.gz"
    sha256 "fe44d0a29d8dba38c134093b3f517ee362573c1d9326ec6a463c7d43537db8fd"
  end

  def install
    bin.install "mdterm"
  end

  test do
    assert_match "mdterm", shell_output("#{bin}/mdterm --version")
  end
end
