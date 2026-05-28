class Tfrr < Formula
  desc "tfrr CLI tool"
  homepage "https://github.com/jimberlage/tfrr"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/jimberlage/tfrr/releases/download/v0.1.0/tfrr-aarch64-apple-darwin.tar.gz"
      sha256 "a766f5a17ccf68da4bd65923265272702e21402dfb213af1dadd5ffacc0b665d"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/jimberlage/tfrr/releases/download/v0.1.0/tfrr-aarch64-linux.tar.gz"
      sha256 "59f8873fce701ca6cc576437de0452a0a62dff4479e22a7906cbae83e9e123d2"
    end
    on_intel do
      url "https://github.com/jimberlage/tfrr/releases/download/v0.1.0/tfrr-x86_64-linux.tar.gz"
      sha256 "a867f4db0a871f6ed826c816367a210cbdfcee5a8028eede2404037340fccce5"
    end
  end

  def install
    bin.install "tfrr"
  end

  test do
    system "#{bin}/tfrr", "--version"
  end
end
