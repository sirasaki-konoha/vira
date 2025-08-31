# vira

**Visual File Read System (vira)**

A simple command-line tool for reading, writing, and managing files with colored output and flexible options.

## Features

- Read files in raw or line-numbered mode
- Write or append content to files with options for rewriting or continuing
- Display file size in human-readable format
- Remove files
- Create new files

## Usage

```
vira <option> <file>
```

### Options

- `-w`, `--write [OPTION] [CONTENT]`  
  Write the following string to a file.  
  Write options:  
    - `r (rewrite)` : Rewrite the file (existing contents will be lost)  
    - `o (overwrite)` : Continue writing (default value)
- `-r`, `--remove`  
  Remove the specified file.
- `-n`, `--new`  
  Create a new file.
- `-s`, `--size`  
  Display the file size.
- `-l`, `--line`  
  Display file contents with line numbers.

## Examples

- Read a file:
  ```
  vira myfile.txt
  ```

- Read a file with line numbers:
  ```
  vira --line myfile.txt
  ```

- Write "hello world" to a file (append):
  ```
  vira myfile.txt --write overwrite "hello world"
  ```

- Rewrite a file with new content:
  ```
  vira myfile.txt --write rewrite "new content"
  ```

- Show file size:
  ```
  vira --size myfile.txt
  ```

- Remove a file:
  ```
  vira --remove myfile.txt
  ```

## Build

Requires [Rust](https://www.rust-lang.org/tools/install).

```
cargo build --release
```

## License
BSD 3-Clause License
Please read [LICENSE](./LICENSE) file.
