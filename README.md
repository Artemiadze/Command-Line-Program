# 🦀 MiniGrep

> A simple command-line text search tool written in Rust.
> Inspired by the classic Unix `grep`, this tool searches for lines containing a given query in a file.
> Supports both **case-sensitive** and **case-insensitive** search.

---

## 🚀 Features

* 🔍 Search for a word or phrase inside a text file
* 🔡 Optional **case-insensitive** mode via environment variable
* ⚡ Built with Rust — safe, fast, and reliable
* 🧪 Includes unit tests for core functionality

---

## 📦 Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install) and Cargo installed.

```bash
git clone <repo_url>
cd minigrep
cargo build --release
```

The compiled binary will appear in `target/release/minigrep`.

---

## 💻 Usage

```bash
cargo run -- <QUERY> <FILE_PATH>
```

### Arguments:

| Argument      | Description                | Example                |
| ------------- | -------------------------- | ---------------------- |
| `<QUERY>`     | The search term            | `to`, `Rust`, `body`   |
| `<FILE_PATH>` | The path to the input file | `poem.txt`, `data.txt` |

---

## 🔠 Case-insensitive Search

You can enable case-insensitive mode using the environment variable `IGNORE_CASE`.

### Linux / macOS:

```bash
IGNORE_CASE=1 cargo run -- to poem.txt
```

### Windows (PowerShell):

```powershell
$env:IGNORE_CASE = "1"
cargo run -- to poem.txt
```

### Windows (cmd.exe):

```cmd
set IGNORE_CASE=1 && cargo run -- to poem.txt
```

---

## 🧩 Example

### File: `poem.txt`

```
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us — don't tell!
They'd banish us, you know.
```

### Run:

```bash
cargo run -- you poem.txt
```

### Output:

```
I'm nobody! Who are you?
Are you nobody, too?
They'd banish us, you know.
```

---

## 🧠 Implementation Overview

* **Config** — stores query, file path, and case sensitivity flag.
* **run()** — reads the file, selects the appropriate search function, and prints results.
* **search()** — performs case-sensitive search.
* **search_case_insensitive()** — performs case-insensitive search.
* **Unit tests** — verify correctness of both search modes.

---

## 🧪 Testing

Run all tests with:

```bash
cargo test
```

Sample output:

```
running 3 tests
test tests::one_result ... ok
test tests::case_sensitive ... ok
test tests::case_insensitive ... ok
```

---

## ⚙️ Environment Variables

| Variable      | Description                              | Example         |
| ------------- | ---------------------------------------- | --------------- |
| `IGNORE_CASE` | Enables case-insensitive search when set | `IGNORE_CASE=1` |

---

## 📚 Example Commands

| Scenario                              | Command                                              |
| ------------------------------------- | ---------------------------------------------------- |
| Case-sensitive search                 | `cargo run -- body poem.txt`                         |
| Case-insensitive (Linux/macOS)        | `IGNORE_CASE=1 cargo run -- body poem.txt`           |
| Case-insensitive (Windows PowerShell) | `$env:IGNORE_CASE = "1"; cargo run -- body poem.txt` |
| Case-insensitive (Windows cmd.exe)    | `set IGNORE_CASE=1 && cargo run -- body poem.txt`    |

---

## 🛠 Requirements

* Rust 1.70+
* Cargo (comes with Rust)

---

## 🧑‍💻 License

This project is open-source and available under the [MIT License](LICENSE.txt).
