# spawn

A simple, effective project creation tool, written in Rust using [cursive](https://github.com/gyscos/cursive).

![language: Rust](https://img.shields.io/badge/language-Rust-orange)
![TUI: cursive](https://img.shields.io/badge/TUI-cursive-blue)

## Features

- **Multi-language project creation**

  | Language | Package manager |
  |----------|-----------------|
  | Rust     | Cargo           |
  | Go       | Go modules      |
  | Python   | uv, poetry, pip |

- **Optional git initialization**: check a box and the new project is initialized as a git repository (`git init`).
- **Creates projects where you are**: new projects are created inside the directory you launched the tool from, and the final dialog shows you the exact path.

## Requirements

- [cargo](https://doc.rust-lang.org/cargo/) (required, both to install spawn and to create Rust projects; comes with the [Rust toolchain](https://rustup.rs/))
- [Go](https://go.dev/dl/) if you create Go projects
- [uv](https://docs.astral.sh/uv/) and [Python 3](https://www.python.org/downloads/) if you create Python projects
- [git](https://git-scm.com/) if you enable git initialization

## Installation & Usage

Install with cargo:

```sh
cargo install --git https://github.com/wazzydotdev/newt
```

Then run it from the directory where you want your new project:

```sh
newt
```

> **Tip:** run the binary from the directory where you want the new project to live. Projects are created relative to your current working directory.
## Project structure

```
spawn/
├── Cargo.toml      # crate manifest (cursive, crossterm, color-eyre)
├── src/
    └── main.rs     # entire application: TUI + scaffolding logic
```

## Roadmap / Ideas

- [ ] More languages (C/C++ with CMake, JavaScript/TypeScript with npm or bun, ...)
- [✓] ~~Choice of package manager per language (e.g. pip/poetry/uv for Python)~~
- [ ] Custom target directory instead of always using the current one
-  [✓] ~~A license picker that adds the license of your choosing from within the TUI~~
