class GitAnalyzer < Formula
  desc "Powerful command-line tool for comprehensive Git repository analysis"
  homepage "https://github.com/alexfilatov/git-analyzer"
  version "0.1.0"
  license "MIT"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/alexfilatov/git-analyzer/releases/download/v0.1.0/git-analyzer-macos-aarch64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_ARM64"
    else
      url "https://github.com/alexfilatov/git-analyzer/releases/download/v0.1.0/git-analyzer-macos-x86_64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_X86_64"
    end
  elsif OS.linux?
    url "https://github.com/alexfilatov/git-analyzer/releases/download/v0.1.0/git-analyzer-linux-x86_64.tar.gz"
    sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_LINUX"
  end

  def install
    bin.install "git-analyzer"
  end

  test do
    system "#{bin}/git-analyzer", "--help"
  end
end