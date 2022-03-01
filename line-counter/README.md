## Counts lines

Given a directory, recursively finds all files with a given file extension in that directory and all sub-directories, and counts the number of lines in the file and prints it to stdout.

## Setup

Just make sure you have Rust development environment setup.

## Run

```bash
# If you want to scan the directory and subdirectory use this syntax
cargo run <absolute_dir_path> <file_extension>

# If you want to just count the lines in one specific file
cargo run <absoulte_file_path>
```

OR
```bash
# Build the binary
cargo build --release 

# If you want to scan the directory and subdirectory use this syntax
./target/release/counter <absolute_dir_path> <file_extension>

# If you want to just count the lines in one specific file
./target/release/counter <absoulte_file_path>
```

## Examples


```bash
# Count lines in the `main.rs` file in the current directory
./target/release/counter ./main.rs
# => 86

# Read `rs` files from the neighboring directory `interpreter`
./target/release/counter ../interpreter rs

# Read lines of this file
./target/release/counter ./README.md
# => 46
```

## Notes

To count the lines of the file `Iter::count()` method is used, which calls `next()` function until it encounters `None`. When the file ends with empty line, `Buffer` treats the line as `None` and ends counting without adding incrementing the counter.
