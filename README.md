# Regli

REGex LIve parser, a tui tool that allows you to filter matches and non matches in file inputs

## Usage

`regli <file.txt>` to render all contents of the file into the tool.

Simply type the regular expression into the box and

- all matches will appear on the left side
- all non-matches will appear on the right side

If not files are inputted, `regli` will be started in mode, allowing you to
search and select files in the tui.

## Local Install

Build locally using

```bash
cargo build
```

Test using

## Development

```bash
cargo test
```
