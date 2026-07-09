# Getting Started with LifePlanner

This guide walks you through setting up and running LifePlanner from scratch, even if you have never used a terminal or built a Rust/Tauri app before. LifePlanner officially targets **macOS and Windows** (see the Platform badge in the README) — Linux is not covered here.

---

## Windows

### 1. Open a terminal

Right-click the Start button and choose **Terminal** (or **Windows PowerShell** on older versions of Windows).

### 2. Check prerequisites

Run each of these commands one by one:

```powershell
rustc --version
cargo --version
node --version
cargo tauri --version
```

If any of them prints something like `'rustc' is not recognized as an internal or external command`, that tool is not installed (or not on your PATH). Install what's missing:

- **Rust / Cargo**: install via [rustup.rs](https://rustup.rs) (run the installer, then restart your terminal)
- **Node.js**: install via [nodejs.org](https://nodejs.org) (LTS version recommended)
- **Tauri CLI**: once Rust/Cargo is installed, run `cargo install tauri-cli`

### 3. Get the code

**Easiest way (no git required):**
1. Go to the [LifePlanner GitHub page](https://github.com/9t29zhmwdh-coder/LifePlanner)
2. Click the green **Code** button → **Download ZIP**
3. Extract the ZIP file somewhere convenient, e.g. `C:\Projekte\LifePlanner`

**Alternative (if you have git):**
```powershell
git clone https://github.com/9t29zhmwdh-coder/LifePlanner.git
cd LifePlanner
```

### 4. Build and run

In your terminal, navigate into the extracted/cloned folder, then run:

```powershell
cd frontend
npm install
cd ..
cargo tauri dev
```

The first run will take a while as Rust compiles all dependencies. Once it's done, a native LifePlanner window should open automatically.

<!-- TODO: Screenshot -->

To create a standalone installer instead of a dev build, run `cargo tauri build` after the steps above.

---

## macOS

### 1. Open a terminal

Press **Cmd+Space** to open Spotlight, type "Terminal", and press Enter.

### 2. Check prerequisites

```bash
rustc --version
cargo --version
node --version
cargo tauri --version
```

If you get a `command not found` error, that tool needs to be installed:

- **Rust / Cargo**: install via [rustup.rs](https://rustup.rs)
- **Node.js**: install via [nodejs.org](https://nodejs.org)
- **Tauri CLI**: `cargo install tauri-cli`

### 3. Get the code

**Easiest way (no git required):**
1. Go to the [LifePlanner GitHub page](https://github.com/9t29zhmwdh-coder/LifePlanner)
2. Click the green **Code** button → **Download ZIP**
3. Extract it, e.g. into `~/Projekte/LifePlanner`

**Alternative (if you have git):**
```bash
git clone https://github.com/9t29zhmwdh-coder/LifePlanner.git
cd LifePlanner
```

### 4. Build and run

```bash
cd frontend
npm install
cd ..
cargo tauri dev
```

A native LifePlanner window should open once everything has compiled and installed.

---

## Optional: Enable AI features

LifePlanner works fully without AI. If you want the local AI daily briefing:

1. Install [Ollama](https://ollama.com)
2. Run `ollama pull llama3`
3. In LifePlanner, go to **Settings → Local AI** and set the Ollama URL

---

### Troubleshooting

| Issue | Cause | Fix |
|---|---|---|
| `'cargo' is not recognized` / `command not found: cargo` | Rust is not installed or not on PATH | Install via [rustup.rs](https://rustup.rs), then restart your terminal |
| `'npm' is not recognized` / `command not found: npm` | Node.js is not installed or not on PATH | Install via [nodejs.org](https://nodejs.org), then restart your terminal |
| PowerShell blocks a `.ps1` script with "running scripts is disabled on this system" | Windows execution policy restricts script execution | Run PowerShell as Administrator and execute `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned`, then retry |
| Build fails on Windows with linker errors mentioning `link.exe` or MSVC | Missing C++ build tools required by Rust's MSVC toolchain | Install "Desktop development with C++" via the [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) installer |
