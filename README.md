# hika

hika is a work-in-progress Linux package manager written in Rust, using Lua for package recipes.

It's mainly focused on binary packages, with plans to support building packages from source later on.

The project is still in early development. It can download packages, but it can't install them yet.

## Features

Currently implemented:
- Lua package recipes
- Package metadata and dependency parsing
- Binary archive downloads over HTTPS
- Multiple download mirrors with fallback support
- Temporary files and basic download error handling

## Building

You'll need Rust and Cargo.

```sh
git clone https://github.com/voltiksrc/hika.git
cd hika
cargo build --release
```

The compiled binary will be at `target/release/hika`.

## Usage

hika currently reads recipes from the local `recipe/` directory.

```sh
cargo run -- -Acquire hello
```

Available commands:

| Command | Description |
| --- | --- |
| `-Acquire <packages...>` | Acquire packages |
| `-Reacquire <packages...>` | Reacquire packages (not implemented) |
| `-Transcend` | Update packages (not implemented) |
| `--help` | Show help |

For now, `-Acquire` only loads the recipe and downloads its archive. No files are installed.

## Package recipes

Packages are described using Lua.

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

Recipes are currently stored in `recipe/<package>/package.lua`.

## License

hika is licensed under the GNU General Public License version 3.0 (GPL-3.0-only). See [LICENSE](LICENSE) for the full license text.
