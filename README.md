# Holding Trainer

A desktop application for practicing holding pattern entries. Built with Rust and egui for pilots training instrument procedures.

## Features

- **Simulate Mode**: Practice with any VOR worldwide without a flight simulator
- **X-Plane 11 Integration**: Real-time practice connected to X-Plane 11
- **Interactive Map**: OpenStreetMap tiles with visual overlay
- **Entry Types**: Automatic calculation of Direct, Teardrop, and Parallel entries
- **Display Modes**: Switch between Radial (R-270) and Cardinal (WEST) notation
- **Visual Sectors**: Color-coded entry zones for quick reference

## Requirements

- Windows (tested on Windows)
- X-Plane 11 (optional, only for X-Plane mode)
- FlyWithLua plugin (required for X-Plane 11 mode)

## How to Use

### Simulate Mode
1. Select a VOR from the worldwide database
2. Click "Generate Position" to create a random scenario
3. Click "Calculate Entry" to see the correct entry type

### X-Plane 11 Mode
1. Install the FlyWithLua script (included in repository)
2. Connect to X-Plane 11
3. Click "New Holding" to generate a random holding pattern
4. Fly to the fix and the app will track your entry

## Tech Stack

- Rust
- egui (GUI framework)
- OpenStreetMap (map tiles)
- reqwest (tile downloading)

## Building

```bash
cargo build --release
```

## Support

If you find this useful, consider supporting development:

[Ko-fi](https://ko-fi.com/jgananb)

## License

MIT
