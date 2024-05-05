# txc

A simple tool for converting text between different formats.

## Installation

### Homebrew

```bash
brew install stounfo/stounfo/txc
```

### Source

```bash
git clone git@github.com:stounfo/txc.git
make build RELEASE=true
```

## Usage

```
$ txc --help
A simple tool for converting text between different formats.

Usage: txc [OPTIONS]

Options:
  -f, --from <FROM>  Type of format to convert from [default: unknown] [possible values: text, single-line, slice-of-line, word, snake-case, kebab-case, camel-case, pascal-case, title, unknown]
  -t, --to <TO>      Type of format to convert to [default: unknown] [possible values: text, single-line, slice-of-line, word, snake-case, kebab-case, camel-case, pascal-case, title, unknown]
  -h, --help         Print help
  -V, --version      Print version

EXAMPLES:

Auto-detect format and convert to PascalCase:
echo -n "hello_world" | txc -t pascal-case

Explicitly specify the input format and convert to PascalCase:
echo "hello_world" | txc -f snake-case -t pascal-case
```
