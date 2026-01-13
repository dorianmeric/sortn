# sortn

`sortn` is a fast, command-line utility written in Rust that performs **natural sorting** on input lines. Unlike standard lexicographical sorting—which would sort "file10.txt" before "file2.txt"—`sortn` treats multi-digit numbers as single numeric values, resulting in a more human-friendly order (1, 2, 10).

## Features

- **Natural Ordering**: Correctly handles numeric sequences within strings.
- **Case Insensitivity**: Optional flag (`-i`) to ignore character casing during comparison.
- **Reverse Sort**: Quickly invert the sorting order with the `-r` flag.
- **Randomize**: Optional flag (`-n`) to shuffle the input lines instead of sorting.
- **Modern CLI**: Built with **Clap v4**, providing a clean interface and automatic help generation.

## Installation

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

### Reverse Order (`-R`)

```bash
$ cat files.txt | sortn -R
file10.txt
file2.txt
file1.txt

```

### Randomize (`-n`)

Shuffle (randomize) the output order. When `-n` is provided, `sortn` will output the input lines in a random order instead of sorting them.

```bash
$ cat files.txt | sortn -r
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
| `-b` | `--ignore-starting-blanks` | Ignore leading blank characters when comparing. |
| `-h` | `--help` | Print help information. |
| `-V` | `--version` | Print version information. |

## Comparison with GNU `sort`

While the standard GNU `sort` utility includes a `-V` (`--version-sort`) flag that achieves similar results, `sortn` offers several distinctions:

* **Simplicity**: `sortn` is a focused, lightweight tool specifically designed for natural sorting without the overhead of the dozens of flags found in GNU `sort`.
* **Performance**: Written in Rust and utilizing the `natord` crate, `sortn` provides highly competitive performance for in-memory sorting tasks.
* **Memory Usage**: Currently, `sortn` loads all input lines into memory to perform a global sort. For extremely large datasets that exceed available RAM, GNU `sort` (which uses disk-based merging) may be more suitable.

## Performance Benchmarks

`sortn` is optimized for speed by:

1. **Locking I/O**: Accessing `stdin` and `stdout` through locked buffers to minimize system call overhead.
2. **Efficient Comparison**: Using the `natord` crate which is optimized for natural string comparison without unnecessary allocations.
3. **Zero-Cost Abstractions**: Leveraging Rust's ownership model to manage string data efficiently during the sort process.

