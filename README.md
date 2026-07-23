# 🛠️ PatchForge — Hybrid Git Management Tool

**PatchForge** is a fast, modern, and user-friendly desktop Git GUI powered by a **hybrid architecture**: a high-performance **Rust** core (`PyO3` / Native C-Extension) for Git logic and a sleek **Python (`CustomTkinter`)** modern interface.

---

## 🚀 Features

* 🎨 **Modern Dark UI** — Powered by CustomTkinter for a clean, intuitive desktop experience.
* ⚡ **Rust Core Execution** — All Git operations run directly through a high-performance Rust C-extension module (`patchforge_core`).
* 📊 **Check Status (`status`)** — View staged and unstaged repository changes in real-time.
* ➕ **Stage Files (`add`)** — Easily stage specific files or all changes (`.`) at once.
* 💾 **Commit Changes (`commit`)** — Quickly apply commit messages.
* ⬆️ **Push & Pull (`push` / `pull`)** — Sync with remote repositories (`origin`) effortlessly.
* 📜 **Commit History (`log`)** — View a clean, customizable commit history log.
* 🌿 **Manage Branches (`branch`)** — List, create, delete, and switch branches from an interactive dropdown menu.
* 📁 **Built-in Folder Browser** — Easily pick repository folders directly within the app.

---

## 🧰 Tech Stack & Architecture

* **Backend / Core Engine:** [Rust](https://www.rust-lang.org/) (`pyo3` bindings)
* **Frontend / GUI:** Python 3 ([CustomTkinter](https://github.com/TomSchimansky/CustomTkinter))
* **Build System & Packaging:** [Maturin](https://github.com/PyO3/maturin) & [PyInstaller](https://pyinstaller.org/)

---

## 🛠️ Prerequisites

Before building or running PatchForge from source, ensure you have:

* [Rust / Cargo](https://www.rust-lang.org/) (edition 2021 or newer)
* [Python 3.10+](https://www.python.org/)
* [Git](https://git-scm.com/) (installed and added to system `PATH`)

---

## 📥 Building & Installation

### 1. Clone the repository

git clone [https://github.com/YOUR_USERNAME/PatchForge.git](https://github.com/YOUR_USERNAME/PatchForge.git)
cd PatchForge

### 2. Set up Python Virtual Environment
# Create virtual environment
python -m venv .venv

# Activate environment (Windows PowerShell)
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope Process
.\.venv\Scripts\Activate.ps1

### 3. Install dependencies & Compile Rust Core

pip install customtkinter maturin pyinstaller
maturin develop

### 4. Run the Application

python "src/main code base/gui.py"

### 📦 Building Standalone Executable (.exe)
To bundle the entire application (Rust module + Python GUI) into a single standalone .exe file that works without Python or Rust pre-installed:

pyinstaller --noconsole --onefile --collect-all customtkinter --collect-all patchforge_core "src/main code base/gui.py"

The compiled binary will be available in the dist/ directory as gui.exe (feel free to rename it to PatchForge.exe).

📝 License
This project is licensed under the MIT License. See the LICENSE file for details.

## Thanks for using PatchForge! 🚀
