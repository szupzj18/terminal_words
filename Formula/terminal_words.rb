class TerminalWords < Formula
  desc "A command-line dictionary tool written in Rust"
  homepage "https://github.com/szupzj18/terminal_words"
  url "https://github.com/szupzj18/terminal_words/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "3bb4449eb71bcc6c7fccb3e32dd862a6b6bee1f2c66dd663a9fae3f4da71d208"
  license "MIT"
  head "https://github.com/szupzj18/terminal_words.git", branch: "main"

  depends_on "rust" => :build

  on_macos do
    depends_on "pkg-config" => :build
    depends_on "openssl@3"
  end

  on_linux do
    depends_on "pkg-config" => :build
    depends_on "openssl@3"
  end

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    output = shell_output("#{bin}/terminal_words --help")
    assert_match "A command-line dictionary tool", output
  end
end