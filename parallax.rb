class Parallax < Formula
  desc "Parallax is a simple command line tool to run shell commands in parallel. Think GNU Parallel but simpler."
  homepage "https://github.com/michaelessiet/homebrew-parallax"
  url "https://github.com/michaelessiet/homebrew-parallax/archive/v0.1.0.tar.gz"
  sha256 "d3f056f2597297eee4e98ba00031003eab873d16f6a95c077956c7214f280634"
  license "MIT" # or whatever license you use

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--root", prefix, "--path", "."
    # Add any additional installation steps
  end

  test do
    system "#{bin}/parallax", "--version"
  end
end
