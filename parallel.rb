class YourPackage < Formula
  desc "Parallel is a simple command line tool to run shell commands in parallel. Think GNU Parallel but simpler."
  homepage "https://github.com/michaelessiet/parallel"
  url "https://github.com/michaelessiet/parallel/archive/v0.1.0.tar.gz"
  sha256 "the_actual_sha256_of_your_tarball"
  license "MIT" # or whatever license you use

  # depends_on "dependency1"
  # depends_on "dependency2"

  depends_on "rust" => :build
  depends_on "cargo" => :build

  def install
    system "cargo", "install", "--root", prefix, "--path", "."
    # Add any additional installation steps
  end

  test do
    system "#{bin}/parallel", "--version"
  end
end
