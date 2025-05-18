# TermPilot

**Explain terminal output with GPT-4 in one command.**

---

## Example

```
record_output ls -la
explain
```

Terminal output:
```
-rw-r--r-- 1 user staff 1024 May 15 notes.txt
```

GPT says:
> "This lists a file called `notes.txt` with read/write permissions, owned by 'user', size 1024 bytes, last modified May 15."

---

## ⚙️ Install

### 1. Clone and build

```
git clone https://github.com/YOURNAME/termpilot
cd termpilot
cargo build --release
```

### 2. Move the binary somewhere on your path (optional)

```
cp target/release/termpilot /usr/local/bin/termpilot
```

### 3. Add these to your shell config (`~/.zshrc`, `~/.bashrc`, etc.)

```
function record_output() {
  "$@" 2>&1 | tee ~/.last_output
}

alias explain='cat ~/.last_output | termpilot'
```

---

## API Key Handling

- You'll be prompted for your OpenAI key on first run.
- It's saved to `~/.config/termpilot/config.toml`
- Reset it anytime with:
```
termpilot --reset-key
```

---

## Flags

| Flag            | Description                      |
|-----------------|----------------------------------|
| `--file <path>` | Read output from a file          |
| `--reset-key`   | Clear stored API key             |
| `--help`        | Show help text                   |
| `--version`     | Show version number              |

---

## License

MIT — open source and ready to fork.

---

## 💡 Why TermPilot?

Because shell commands are fast. But understanding what they just did shouldn't slow you down.

```
GPT, explain this mess.
```
```
Done.
```
