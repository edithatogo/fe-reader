cask "fe-reader" do
  version "0.1.0-preview.1"
  sha256 "PLACEHOLDER"

  url "https://github.com/edithatogo/fe-reader/releases/download/v#{version}/fe-reader-#{version}-macos-universal.dmg"
  name "Fe Reader"
  desc "Local-first cross-platform PDF workflow platform"
  homepage "https://edithatogo.github.io/fe-reader/"

  app "Fe Reader.app"
  binary "Fe Reader.app/Contents/MacOS/fe-reader", target: "fe-reader"
end
