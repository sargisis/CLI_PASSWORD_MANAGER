<h1 align="center">
  <img src="https://readme-typing-svg.herokuapp.com?font=Fira+Code&size=28&duration=3000&pause=1000&color=00F0FF&center=true&vCenter=true&width=600&lines=CLI+PASSWORD+MANAGER+🔐;Rust+Project;Secure+Command+Line+Password+Tool;Learning+Systems+Programming" alt="Typing SVG" />
</h1>

<p align="center">
  A minimal password manager written in <b>Rust</b> — using structs, enums, modules & CLI parsing.
</p>

---

## 🚀 Features (Final Version)

| Command | Description |
|--------|-------------|
| `add <service> <login> <password>` | Add new entry |
| `get <service>` | Show credentials |
| `list` | Display all entries (current run) |
| `delete <service>` | Remove entry |

> Data is currently stored only in memory.

---

## 📦 Usage

```bash
cargo run -- add gmail myuser pass123
cargo run -- get gmail
cargo run -- list
cargo run -- delete gmail
