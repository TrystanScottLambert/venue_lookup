# UWA Venue Finder

A fast terminal tool for fuzzy-searching UWA venue codes. Type a few characters and it instantly narrows down the list. Hit Enter and the venue code is copied to your clipboard.

## Install

### Download a pre-built binary

Go to the [latest release](../../releases/latest) and download the right file for your system:

| Platform | File |
|---|---|
| macOS (Apple Silicon) | `venue_lookup-macos-arm64.tar.gz` |
| macOS (Intel) | `venue_lookup-macos-x86_64.tar.gz` |
| Linux | `venue_lookup-linux-x86_64.tar.gz` |
| Windows | `venue_lookup-windows-x86_64.zip` |

#### macOS / Linux

```sh
# Extract (replace the filename with whichever you downloaded)
tar xzf venue_lookup-macos-arm64.tar.gz

# Move it somewhere on your PATH
mv venue_lookup /usr/local/bin/

# On macOS you may need to allow it first:
# Right-click → Open, or:
xattr -d com.apple.quarantine /usr/local/bin/venue_lookup
```

#### Windows

Extract the `.zip` and put `venue_lookup.exe` somewhere on your PATH, or just run it from the extracted folder.

> **Note:** The clipboard copy feature uses the OSC 52 terminal escape sequence. This works in Windows Terminal, iTerm2, kitty, alacritty, and WezTerm. It does **not** work in macOS Terminal.app or older terminals.

### Build from source

If you have Rust installed:

```sh
git clone https://github.com/TrystanScottLambert/venue_lookup.git
cd venue_lookup
cargo build --release
./target/release/venue_lookup
```

## Usage

Run it:

```
venue_lookup
```

Then:

- **Type** to fuzzy-search venues — results update instantly
- **↑ / ↓** arrow keys to move the selection
- **Enter** to select — copies the venue code to your clipboard
- **Backspace** to edit your search
- **Ctrl+C** to quit

### Example

```
UWA Venue Finder
Type to search. ↑↓ navigate. Enter = select. Ctrl+C = quit.

> fox lect

  106.G59              ARTS - Fox Lecture Theatre
  106.G57              ARTS - Alexander Lecture Theatre
  106.G58              ARTS - Murdoch Lecture Theatre
  ...
```

Selecting the top result copies `106.G59` to your clipboard.

## License

MIT
