class Parallax < Formula
  desc "Parallax is a simple command line tool to run shell commands in parallel. Think GNU Parallel but simpler."
  homepage "https://github.com/michaelessiet/homebrew-parallax"
  url "https://github.com/michaelessiet/homebrew-parallax/archive/v0.1.0.tar.gz"
  sha256 "46cac1b92c8eba882f90e48fce9c1da88ab8a26b2fec5d2946e69a74402da290"
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
