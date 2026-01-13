# sortn (Sort-Natural)

`sortn` is a fast, command-line utility written in Rust that performs **natural sorting** on input lines. Unlike standard lexicographical sorting—which would sort "file10.txt" before "file2.txt"—`sortn` treats multi-digit numbers as single numeric values, resulting in a more human-friendly order (1, 2, 10).

## Features

- **Natural ordering**: numeric parts of strings are compared as numbers (e.g., `2` < `10`).
- **Case sensitivity**: sorting and uniqueness are case-sensitive by default (uppercase sorts before lowercase). Use `-i` / `--ignore-case` to compare case-insensitively.
- **Reverse**: `-r` / `--reverse` reverses the sort order.
- **Randomize**: `-n` / `--randomize` shuffles the input instead of sorting.
- **Unique**: `-u` / `--make-unique` removes duplicate lines, keeping the first occurrence. Uniqueness is case-sensitive by default; use `-i` to make uniqueness comparisons case-insensitive.
- **Skip blank lines**: `-b` / `--skip-blank-lines` removes empty or whitespace-only lines before processing; such lines will not be returned.
- **Whitespace handling**: the underlying [`natord::compare`](https://docs.rs/natord) used for comparisons ignores whitespace characters (spaces and tabs). If your ordering depends on exact whitespace, preprocess input first.

## Quick behavior summary

- If `--randomize` is used, input is shuffled. Otherwise input is sorted.
- After sorting or shuffling, `--make-unique` (if present) is applied; the first occurrence of each unique key is kept.
- Flags that affect comparisons: `--ignore-case` affects both sorting and uniqueness; `natord::compare` itself ignores whitespace.

## Installation

Download a prebuilt executable for your platform from the Releases page, or build from source (see below).

### Prebuilt binaries (recommended)

Visit the Releases page for the latest prebuilt assets:

#### Windows 
Windows (64-bit) download link: https://github.com/dorianmeric/sortn/releases/download/v1.0.0/sortn-x86_64-pc-windows-msvc.zip

After downloading, extract and move `sortn.exe` to `C:\Program Files\sortn\` and add that folder to your `%PATH%`, or move it to an existing folder already on `PATH` (requires admin to write to `C:\Program Files`).

```powershell
Expand-Archive sortn-*-windows-*.zip -DestinationPath 'C:\Program Files\sortn\'
# Add to PATH permanently (requires a new shell to take effect)
setx PATH "%PATH%;C:\Program Files\sortn\"
```


#### Linux 
Linux (x86 64-bit) download link:   https://github.com/dorianmeric/sortn/releases/download/v1.0.0/sortn-x86_64-unknown-linux-gnu.tar.xz

After downloading, extract and move to `/usr/local/bin` (requires sudo):

```bash
tar xzf sortn-*-linux-*.tar.gz
sudo mv sortn /usr/local/bin/
sudo chmod +x /usr/local/bin/sortn
```

#### macOS
macOS (x86 64-bit) download link:   https://github.com/dorianmeric/sortn/releases/download/v1.0.0/sortn-x86_64-apple-darwin.tar.xz

After downloading, extract and move to `/usr/local/bin` or `~/bin`:

```bash
tar xzf sortn-*-apple-*.tar.gz
sudo mv sortn /usr/local/bin/
sudo chmod +x /usr/local/bin/sortn
# or for a user-local install:
mv sortn ~/bin/
chmod +x ~/bin/sortn
```

If you prefer not to install globally, you can keep the binary in any folder and run it by specifying its path (for example `./sortn`).


### Installation from source

Ensure you have the Rust toolchain installed.

1. Clone this repository.
2. Build the release binary:
```bash
git clone https://github.com/dorianmeric/sortn.git
cd sortn
cargo build --release

```


3. Move the binary to your path:
```bash
cp target/release/sortn /usr/local/bin/

```



## Usage

`sortn` reads lines from standard input and writes results to standard output. Below are common examples and flag behaviors.

### Basic sorting

```bash
$ cat files.txt
file10.txt
file2.txt
file1.txt

$ cat files.txt | sortn
file1.txt
file2.txt
file10.txt
```

### Reverse order

```bash
$ cat files.txt | sortn -r
file10.txt
file2.txt
file1.txt
```

### Randomize

Shuffle the input instead of sorting:

```bash
$ cat files.txt | sortn -n
file2.txt
file1.txt
file10.txt
```

### Unique (`-u`)

Keep first occurrence of duplicate lines.

Input `input.txt`:

```
a
A
a
```

Case-sensitive unique (default):

```bash
$ cat input.txt | sortn -u
a
A
```

Case-insensitive unique (use `-i`):

```bash
$ cat input.txt | sortn -u -i
a
```

### Skip blank lines (`-b`)

```bash
$ printf "a\n\n  \nb\n" | sortn -b
a
b
```

### Whitespace behavior

`natord::compare` ignores whitespace characters when comparing lines. Example:

```
" a1"
"a1"
```

`sortn` will treat these as equivalent for ordering because whitespace is ignored by the comparison function; if you need to preserve whitespace differences, trim or escape them before sorting.

## Options

| Flag | Long Flag | Description |
| --- | --- | --- |
| `-n` | `--randomize` | Shuffle input instead of sorting. |
| `-r` | `--reverse` | Reverse sort order. |
| `-i` | `--ignore-case` | Case-insensitive comparisons for both sorting and uniqueness. |
| `-u` | `--make-unique` | Remove duplicate lines (first occurrence kept). Case-sensitive by default; adding `-i` (`-ui` for short) makes matching case-insensitive. |
| `-b` | `--skip-blank-lines` | Drop blank lines (empty or whitespace-only) before processing; they will not be returned. |
| `-h` | `--help` | Show help information. |
| `-V` | `--version` | Show version information. |

Note:
- when both `--randomize` and sorting flags are present, `--randomize` takes precedence (input is shuffled). 
- After shuffling or sorting, `--make-unique` is applied.

## Comparison with GNU `sort`

The standard GNU `sort` includes a `-V` (`--version-sort`) mode that performs natural-like ordering. `sortn` differs by being a focused Rust utility with these distinctions:

- **Simplicity**: small focused tool with a few flags.
- **Performance**: optimized in Rust and using `natord` for comparisons.
- **Memory model**: `sortn` loads all input into memory for sorting; for extremely large datasets, GNU `sort` (which can use disk) may be more appropriate.

## Performance Benchmarks

`sortn` is optimized for speed by:

1. **Locking I/O**: Accessing `stdin` and `stdout` through locked buffers to minimize system call overhead.
2. **Efficient Comparison**: Using the `natord` crate which is optimized for natural string comparison without unnecessary allocations.
3. **Zero-Cost Abstractions**: Leveraging Rust's ownership model to manage string data efficiently during the sort process.

## Further notes

- Documentation for the comparison algorithm: https://docs.rs/natord
- Quick example: `cat files.txt | sortn -i -r -u`

If you'd like, I can also apply this README patch directly to the repository.

