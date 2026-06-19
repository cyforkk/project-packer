# Project Packer

One-click tool to clean project dependencies and pack pure source code for server deployment.

## Features

- **Smart Detection** - Auto-detect project type (Java / Node.js / Python / Rust / Go / .NET / PHP)
- **Tiered Cleaning** - Four cleaning modes, choose what to clean
- **Flexible Output** - ZIP archive or copy to directory
- **Live Preview** - Preview files to be cleaned before packing, with custom selection
- **Progress Display** - Real-time progress percentage and current file during packing

## Cleaning Modes

| Mode | What it cleans |
|------|---------------|
| **Basic** | Dependencies only (node_modules, venv, target, etc.) |
| **Medium** | Dependencies + IDE configs (.idea, .vscode, etc.) |
| **Full** | Dependencies + Build artifacts + IDE configs + VCS + Logs + Temp files |
| **Custom** | User manually selects what to clean |

## Built-in Rules

| Category | Targets |
|----------|---------|
| Dependencies | `node_modules` `target` `venv` `.venv` `vendor` `.m2` `__pycache__` `.eggs` `prototypes` |
| Build Artifacts | `dist` `build` `out` `.next` `.nuxt` |
| IDE Configs | `.idea` `.vscode` `.settings` `.project` `.classpath` |
| VCS | `.git` `.svn` |
| Logs | `*.log` |
| Temp Files | `*.tmp` `.cache` `.DS_Store` `Thumbs.db` |
| Docs | `docs` (not selected by default, manual selection required) |

## Tech Stack

- **Desktop Framework:** Tauri 2.x
- **Frontend:** Vue 3 + TypeScript + Element Plus
- **Backend:** Rust
- **Build:** Vite + Cargo

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (>= 18)
- [Rust](https://www.rust-lang.org/tools/install) (>= 1.77)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (Windows)

### Install & Run

```bash
git clone <repo-url>
cd project-packer
npm install
npm run tauri dev
```

### Build

```bash
npm run tauri build
```

Output:

```
src-tauri/target/release/
├── project-packer.exe                              # Standalone executable
└── bundle/nsis/项目打包工具_0.1.0_x64-setup.exe     # NSIS installer
```

## Project Structure

```
project-packer/
├── src/                          # Frontend (Vue 3)
│   ├── App.vue
│   ├── types.ts
│   └── components/
│       ├── ProjectSelector.vue
│       ├── ProjectInfo.vue
│       ├── CleanModeSelector.vue
│       ├── CleanableItemsList.vue
│       ├── OutputOptions.vue
│       ├── PackProgress.vue
│       └── FilePreviewDialog.vue
├── src-tauri/                    # Backend (Rust)
│   ├── src/
│   │   ├── commands/             # Tauri commands (scan_project / pack_project)
│   │   ├── scanner/              # Project scanner (detection + traversal)
│   │   ├── rules/                # Cleaning rules engine
│   │   ├── packer/               # Packer (ZIP / directory copy)
│   │   └── types.rs
│   └── tauri.conf.json
└── docs/
```

## Usage

1. **Select Project** - Click "Select Folder" button
2. **View Info** - Auto-scan shows project type, size, and cleanable items
3. **Choose Mode** - Select cleaning mode (Basic / Medium / Full / Custom)
4. **Set Output** - Choose output format (ZIP / Copy) and save path
5. **Start Packing** - Click "Start Packing" and wait

## Other Languages

- [中文版本](README.md)

## License

MIT
