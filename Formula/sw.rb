class Sw < Formula
  desc "A command-line dictionary tool written in Rust"
  homepage "https://github.com/szupzj18/sw"
  url "https://github.com/szupzj18/sw/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "2477e7e03e9a2980799c4ec11d2cef6cf567b4fcafb7496dc1e5dee04d02d10f"
  license "MIT"
  head "https://github.com/szupzj18/sw.git", branch: "main"

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
    output = shell_output("#{bin}/sw --help")
    assert_match "A command-line dictionary tool", output
  end
end