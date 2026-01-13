# sortn (Sort-Natural)

`sortn` is a fast, command-line utility written in Rust that performs **natural sorting** on input lines. Unlike standard lexicographical sorting—which would sort "file10.txt" before "file2.txt"—`sortn` treats multi-digit numbers as single numeric values, resulting in a more human-friendly order (1, 2, 10).

## Features

- **Natural Ordering**: Correctly handles numeric sequences within strings.
- **Case Insensitivity**: the sorting is case-sensitive by default (ie. lowercase comes after uppercase). Optional flag (`-i`) to ignore character casing during comparison.
- **Reverse Sort**: Quickly invert the sorting order with the `-r` flag.
- **Randomize**: Optional flag (`-n`) to shuffle the input lines instead of sorting.
- **Unique**: Optional flag (`-u`) to remove duplicate lines (first occurrence kept). Uniqueness is case-sensitive by default; use `-i` to make uniqueness case-insensitive.
- **Whitespace Handling**: Note: leading whitespaces (spaces and tabs) are ignored when sorting. If your ordering depends on whitespace, preprocess input accordingly
- **Skip Blank Lines**: Optional flag (`-b`) to remove blank lines (empty or whitespace-only) from the input before processing; such lines will not be sorted or returned.
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
cargo build --release

```


3. Move the binary to your path:
```bash
cp target/release/sortn /usr/local/bin/

```



## Usage

`sortn` reads from standard input (`stdin`) and writes the sorted result to standard output (`stdout`).

### Basic Sorting

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

### Reverse Order (`-r`)

```bash
$ cat files.txt | sortn -r
file10.txt
file2.txt
file1.txt

```

### Randomize (`-n`)

Shuffle (randomize) the output order. When `-n` is provided, `sortn` will output the input lines in a random order instead of sorting them.

```bash
$ cat files.txt | sortn -n
file2.txt
file1.txt
file10.txt

```

### Case-Insensitive (`-i`)

```bash
$ echo -e "B\na\nC" | sortn -i
a
B
C

```

## Options

| Flag | Long Flag | Description |
| --- | --- | --- |
| `-n` | `--randomize` | Randomize (shuffle) the output order. |
| `-r` | `--reverse` | Sort in reverse order. |
| `-i` | `--ignore-case` | Perform case-insensitive natural sorting. |
| `-u` | `--unique` | Remove duplicate lines (first occurrence kept). Case-sensitive by default; `-i` makes matching case-insensitive. |
| `-b` | `--skip-blank-lines` | Remove blank lines (empty or whitespace-only) before processing; they will not be returned. |
| `-h` | `--help` | Print help information. |
| `-V` | `--version` | Print version information. |

Note: When using `--unique`/`-u`, uniqueness is case-sensitive by default; use `-i` to make uniqueness matching case-insensitive.

## Comparison with GNU `sort`

Note: The natural comparison used by `sortn` (via the `natord::compare` crate) ignores whitespace characters such as spaces and tabs when comparing lines. Keep this in mind if your input relies on whitespace for ordering; preprocess input if necessary.

While the standard GNU `sort` utility includes a `-V` (`--version-sort`) flag that achieves similar results, `sortn` offers several distinctions:

* **Simplicity**: `sortn` is a focused, lightweight tool specifically designed for natural sorting without the overhead of the dozens of flags found in GNU `sort`.
* **Performance**: Written in Rust and utilizing the `natord` crate, `sortn` provides highly competitive performance for in-memory sorting tasks.
* **Memory Usage**: Currently, `sortn` loads all input lines into memory to perform a global sort. For extremely large datasets that exceed available RAM, GNU `sort` (which uses disk-based merging) may be more suitable.

## Performance Benchmarks

`sortn` is optimized for speed by:

1. **Locking I/O**: Accessing `stdin` and `stdout` through locked buffers to minimize system call overhead.
2. **Efficient Comparison**: Using the `natord` crate which is optimized for natural string comparison without unnecessary allocations.
3. **Zero-Cost Abstractions**: Leveraging Rust's ownership model to manage string data efficiently during the sort process.

