# 🛠️ PatchForge — Git Management Tool

**PatchForge** is a lightweight, fast, and user-friendly CLI wrapper for **Git**, written in **Rust**. It provides a simple interactive text interface to perform basic and advanced Git operations without needing to memorize complex command syntax.

---

## 🚀 Features

* 📊 **Check Status (`status`)** — Quickly view the current state of your repository.
* ➕ **Add Files (`add`)** — Stage specific files or all changes (`.`) at once.
* 💾 **Commit Changes (`commit`)** — Easily enter commit messages.
* ⬆️ **Push & Pull (`push` / `pull`)** — Sync with remote repositories in one click.
* 📜 **Commit History (`log`)** — Clean and concise view of recent commits.
* 🌿 **Manage Branches (`branch`)** — List, create, delete, and switch branches effortlessly.

---

## 🛠️ Requirements

Before running PatchForge, make sure you have the following installed:

* [Rust / Cargo](https://www.rust-lang.org/) (v1.70 or newer)
* [Git](https://git-scm.com/) (must be added to your system `PATH`)

---

## 📥 Installation & Usage

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/YOUR_USERNAME/PatchForge.git](https://github.com/YOUR_USERNAME/PatchForge.git)
   cd PatchForge

2. **Run the project via Cargo:**
   cargo run

3.**(Optional) Build a release binary:**
  cargo build --release

**🧰 Tech Stack:**
Language: Rust
Core Modules: std::process::Command, std::io

**📝 License:**
This project is licensed under the MIT License. See the LICENSE file for details.

**THANKS!!**
