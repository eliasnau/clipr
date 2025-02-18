class Clipr < Formula
  desc "A simple and efficient clipboard utility for the command line"
  homepage "https://github.com/eliasnau/clipr"
  url "https://github.com/eliasnau/clipr/releases/download/v0.1.0/clipr-mac.tar.gz"
  sha256 "710cd5173e1a9a184a461a2de15ab8b3db5b1c97b6078e935dba1f1b10fdca2b"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--root", prefix, "--path", "."
  end

  test do
    system "#{bin}/clipr", "--version"
  end
end