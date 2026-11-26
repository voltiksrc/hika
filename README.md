# hika

hika is a Linux package manager written in Rust that uses Lua for package recipes. It's mainly focused on binary packages, but I plan on supporting source packages later on. hika is still pretty early in development. It can currently download, verify and extract packages, but installation isn't implemented yet.

## Features

- Lua package recipes
- Package metadata and dependency parsing
- Download mirror fallback
- SHA-256 checksum verification
- Temporary downloads
- Native archive extraction
- Support for `.tar`, `.tar.gz`, `.tar.xz`, `.tar.bz2`, `.tar.zst` and `.deb`

## Building

Make sure you have Rust and Cargo installed.

```sh
git clone https://github.com/voltiksrc/hika.git
cd hika
cargo build --release
```

The binary will be located at `target/release/hika`.

## Usage

For now, hika reads package recipes from the local `recipe/` directory.

```sh
cargo run -- -Acquire hello
```

### Commands

| Command | Description |
| --- | --- |
| `-Acquire <packages...>` | Acquire packages |
| `-Reacquire <packages...>` | Reacquire packages (WIP) |
| `-Transcend` | Update packages (WIP) |
| `--help` | Show available commands |

Currently, `-Acquire` only loads the package recipe, downloads the archive and verifies its checksum. Extraction hasn't been integrated into the command yet.

## Package recipes

Here's what a basic package recipe looks like:

```lua
pkg {
    name = "hello",
    version = "2.12.3",
    description = "GNU Hello, a simple greeting program",

    artifacts = {
        {
            urls = {
                "https://example.org/hello.deb",
                "https://mirror.example.org/hello.deb",
            },
            sha256 = "...",
        },
    },

    runtime_deps = { "glibc" },
}
```

Recipes are stored in `recipe/<package>/package.lua`.

## Plans

- Dependency resolution
- Package installation and removal
- Installed package tracking
- Package repositories
- Source packages

## License

hika is licensed under GPL-3.0-only. See [LICENSE](LICENSE).
