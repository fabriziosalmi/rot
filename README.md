# 🎨 LiveScope - Real-time System Performance Art

LiveScope transforms your system's performance metrics into mesmerizing real-time ASCII art visualizations. Watch your CPU cores dance with colors, memory flow like waves, and network activity sparkle as particles across your terminal!

## ✨ Features

- **🔥 CPU Visualization**: Each CPU core creates unique animated patterns and colors
- **🌊 Memory Waves**: RAM usage flows as dynamic waves across the screen  
- **⭐ Particle Effects**: Network and disk I/O activity creates beautiful particle systems
- **🎨 Multiple Themes**: Fire, Ocean, Matrix, and Rainbow color schemes
- **⚡ High Performance**: Runs at 60fps with minimal resource usage
- **🖥️ Terminal Native**: No GUI required - pure ASCII art magic

## 🚀 Installation & Usage

### Build from Source
```bash
# Clone the repository
git clone <your-repo>
cd livescope

# Build the project
cargo build --release

# Run LiveScope
cargo run --release
```

### Command Line Options
```bash
# Basic usage
cargo run

# Custom refresh rate (16ms = ~60fps)
cargo run -- --refresh 16

# Choose a color theme
cargo run -- --theme fire      # Fire theme (default)
cargo run -- --theme ocean     # Ocean blue theme  
cargo run -- --theme matrix    # Matrix green theme
cargo run -- --theme rainbow   # Rainbow theme

# Enable particle effects
cargo run -- --particles
```

## 🎮 Controls

- **`q`** - Quit LiveScope
- **`p`** - Toggle particle effects (coming soon)

## 🎨 Themes

- **Fire**: Warm oranges and reds that pulse with CPU activity
- **Ocean**: Cool blues and teals that flow like water
- **Matrix**: Classic green-on-black terminal aesthetic  
- **Rainbow**: Full spectrum of colors cycling through patterns

## 🔧 System Requirements

- **OS**: Linux, macOS, Windows
- **Terminal**: Any modern terminal with color support
- **Rust**: 1.70+ (for building from source)

## 🎯 Performance Art Patterns

LiveScope creates different visual patterns based on your system metrics:

- **CPU Cores**: Each core gets its own horizontal band with intensity-based patterns
- **Memory Usage**: Sine wave patterns that grow/shrink with RAM consumption  
- **Network I/O**: Falling particles that speed up with network activity
- **Disk Activity**: Geometric patterns that pulse with file system operations

## 🛠️ Technical Details

- Built in **Rust** for maximum performance and safety
- Uses `sysinfo` for cross-platform system monitoring
- Uses `crossterm` for advanced terminal control and colors
- Async/await with `tokio` for smooth 60fps rendering
- Minimal CPU overhead while creating maximum visual impact

## 🎪 Why LiveScope?

Traditional system monitors show boring graphs and numbers. LiveScope turns your computer's heartbeat into art! Perfect for:

- **Developers** monitoring build processes and system load
- **Streamers** adding visual flair to coding sessions  
- **System Administrators** keeping an eye on server performance
- **Anyone** who wants their terminal to look absolutely stunning

## 📊 Coming Soon

- [ ] Disk I/O visualization patterns
- [ ] Network bandwidth particle trails  
- [ ] Custom color theme creation
- [ ] Save/load configuration profiles
- [ ] Plugin system for custom visualizers
- [ ] Real-time audio reactive mode

---

*"Your system metrics have never looked this good!"* ⚡

Built with ❤️ in Rust 🦀
