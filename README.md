# 🚀 lol - The Fast Multi-language Code Compiler CLI App

**lol** is a simple yet powerful CLI tool designed to speed up compiling source code across multiple programming languages with a single command. Whether you're juggling C, C++, Python, or other languages, lol scans your project folder, detects your code files, and compiles them all in parallel — making the whole process fast and effortless.

## ✨ Why Build lol in Rust?

Rust is perfect for lol because it combines raw speed, safety, and great concurrency support. The compiled binary runs smoothly on any platform — Windows, macOS, Linux — without extra dependencies. Rust's strong error handling and type safety help make lol stable and reliable, so it won't crash midway through your build. Plus, Rust's async and multithreading abilities allow lol to compile many files at once, drastically cutting down build times.

## 🚀 Features

- **Multi-language Support**: Compile C, C++, Python, Java, Rust, Go, JavaScript, TypeScript, and many more
- **Parallel Compilation**: Uses Rust's concurrency features to compile multiple files simultaneously
- **Smart File Detection**: Automatically detects source files by extension and groups them by language
- **Custom Compiler Flags**: Support for custom compiler flags per language
- **Progress Tracking**: Beautiful progress bars showing compilation progress
- **Cross-platform**: Runs on Windows, macOS, and Linux
- **Configuration**: Persistent user preferences and settings
- **Verbose Output**: Detailed compilation output and error reporting

## 🛠️ Installation

### Prerequisites

Make sure you have Rust installed on your system. You can install it from [rustup.rs](https://rustup.rs/).

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/lol.git
cd lol

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

### Download Pre-built Binary

Pre-built binaries for various platforms are available in the [Releases](https://github.com/yourusername/lol/releases) section.

## 📖 Usage

### Basic Usage

```bash
# Compile all detected languages in a project
lol /path/to/your/project

# Compile specific languages
lol /path/to/your/project --c --cpp --python

# Compile all languages explicitly
lol /path/to/your/project --all

# Show verbose output
lol /path/to/your/project --verbose

# Set number of parallel jobs
lol /path/to/your/project --jobs 8
```

### Command Line Options

```
USAGE:
    lol [OPTIONS] <PROJECT_PATH>

ARGS:
    <PROJECT_PATH>    Project directory to compile

OPTIONS:
    --all                 Compile all detected languages
    --c                   Compile C files
    --cpp                 Compile C++ files
    --python              Compile Python files
    --java                Compile Java files
    --rust                Compile Rust files
    --go                  Compile Go files
    --js                  Compile JavaScript files
    --ts                  Compile TypeScript files
    -j, --jobs <JOBS>     Number of parallel compilation jobs [default: number of CPU cores]
    --cflags <FLAGS>      Custom compiler flags for C
    --cxxflags <FLAGS>    Custom compiler flags for C++
    -v, --verbose         Show verbose output
    -h, --help            Print help information
    -V, --version         Print version information
```

### Examples

```bash
# Compile a C/C++ project with custom flags
lol /path/to/cpp-project --c --cpp --cflags "-O2 -march=native" --cxxflags "-O2 -std=c++20"

# Compile a mixed-language project with verbose output
lol /path/to/mixed-project --all --verbose

# Compile only Python and JavaScript files
lol /path/to/web-project --python --js

# Compile with limited parallelism (useful for resource-constrained systems)
lol /path/to/project --jobs 2
```

## 🔧 Supported Languages

| Language | Extensions | Compiler | Notes |
|----------|------------|----------|-------|
| C | `.c`, `.h` | gcc | Supports custom flags via `--cflags` |
| C++ | `.cpp`, `.cc`, `.cxx`, `.c++`, `.hpp`, `.hxx`, `.h++` | g++ | Supports custom flags via `--cxxflags` |
| Python | `.py`, `.pyw`, `.pyx`, `.pxd` | python3 | Syntax checking via `py_compile` |
| Java | `.java` | javac | |
| Rust | `.rs` | rustc | |
| Go | `.go` | go | |
| JavaScript | `.js`, `.mjs`, `.cjs` | node | Syntax checking via `--check` |
| TypeScript | `.ts`, `.tsx` | tsc | Syntax checking via `--noEmit` |
| C# | `.cs` | dotnet | |
| Swift | `.swift` | swiftc | |
| Kotlin | `.kt`, `.kts` | kotlinc | |
| Scala | `.scala`, `.sc` | scalac | |
| Haskell | `.hs`, `.lhs` | ghc | |
| F# | `.fs`, `.fsx`, `.fsi` | fsharpc | |
| OCaml | `.ml`, `.mli` | ocamlc | |
| Nim | `.nim` | nim | |
| Zig | `.zig` | zig | |
| V | `.v` | v | |
| Odin | `.odin` | odin | |
| Jai | `.jai` | jai | |

## ⚙️ Configuration

lol creates a configuration file at `~/.config/lol/config.json` (Linux/macOS) or `%APPDATA%\lol\config.json` (Windows) on first run.

### Configuration Options

```json
{
  "parallel_jobs": 8,
  "compiler_flags": {
    "c": "-Wall -Wextra -std=c99",
    "cpp": "-Wall -Wextra -std=c++17",
    "rust": "--release",
    "go": "-ldflags=-s -ldflags=-w"
  },
  "ignore_patterns": [
    "*.o",
    "*.obj",
    "build/",
    "target/",
    "node_modules/"
  ],
  "include_patterns": [],
  "output_directory": "build",
  "verbose_output": false,
  "auto_clean": false,
  "watch_mode": false,
  "language_settings": {
    "c": {
      "enabled": true,
      "compiler_flags": ["-Wall", "-Wextra", "-std=c99"],
      "output_format": "o"
    }
  }
}
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_compiler_creation
```

## 🚀 Performance

lol is designed for speed:

- **Parallel Compilation**: Compiles multiple files simultaneously using Rust's async/await and rayon
- **Smart File Grouping**: Groups files by language to minimize compiler startup overhead
- **Efficient File Discovery**: Uses `walkdir` for fast recursive directory traversal
- **Memory Efficient**: Processes files in streams rather than loading everything into memory

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/lol.git
cd lol

# Install dependencies
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt

# Run clippy
cargo clippy
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://rust-lang.org/) for performance and safety
- CLI framework powered by [clap](https://github.com/clap-rs/clap)
- Progress bars by [indicatif](https://github.com/mitsuhiko/indicatif)
- Parallel processing with [rayon](https://github.com/rayon-rs/rayon)
- Async runtime by [tokio](https://github.com/tokio-rs/tokio)

## 📊 Roadmap

- [ ] Incremental compilation by detecting changed files
- [ ] Watch mode that recompiles when files change
- [ ] Integration with build systems (Make, CMake, etc.)
- [ ] Support for more programming languages
- [ ] Plugin system for custom language support
- [ ] CI/CD integration examples
- [ ] Docker support for isolated compilation environments

## 🐛 Known Issues

- Some compilers may not be available on all systems
- Windows path handling may need improvements
- Large projects with many files may experience memory usage

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/yourusername/lol/issues) page
2. Search existing discussions
3. Create a new issue with detailed information

---

**Happy coding! 🎉**

*Built with ❤️ in Rust* 