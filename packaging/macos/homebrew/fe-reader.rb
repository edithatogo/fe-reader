cask "fe-reader" do
  version "0.1.0"
  sha256 "PLACEHOLDER"

  url "https://example.invalid/fe-reader-#{version}.dmg"
  name "Fe Reader"
  desc "Local-first cross-platform PDF workflow platform"
  homepage "https://example.invalid/fe-reader"

  app "Fe Reader.app"
  binary "Fe Reader.app/Contents/MacOS/fe-reader", target: "fe-reader"
end
