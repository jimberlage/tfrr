class Tfrr < Formula
  desc "tfrr CLI tool"
  homepage "https://github.com/jimberlage/tfrr"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/jimberlage/tfrr/releases/download/v0.1.0/tfrr-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_SHA256"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/jimberlage/tfrr/releases/download/v0.1.0/tfrr-aarch64-linux.tar.gz"
      sha256 "REPLACE_WITH_SHA256"
    end
    on_intel do
      url "https://github.com/jimberlage/tfrr/releases/download/v0.1.0/tfrr-x86_64-linux.tar.gz"
      sha256 "REPLACE_WITH_SHA256"
    end
  end

  def install
    bin.install "tfrr"
  end

  test do
    system "#{bin}/tfrr", "--version"
  end
end
