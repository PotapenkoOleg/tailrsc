# tailrsc

**tailrsc** is a Rust version of the BSD `tail` command

## Usage

```
tailrsc [OPTIONS] [FILE]...
```

### Options

| Option | Description |
|---|---|
| `-n`, `--lines <N>` | Output the last N lines (default: 10). N uses format `[+\|-][number][b\|k\|m]` |
| `-c`, `--bytes <N>` | Output the last N bytes. N uses format `[+\|-][number][b\|k\|m]` |
| `-b`, `--blocks <N>` | Output the last N 512-byte blocks |
| `-f`, `--follow` | Output appended data as the file grows |
| `-F` | Like `-f`, but also detect file rename/rotation |
| `-r` | Display input in reverse order, by line |
| `-q`, `--quiet` | Never output headers giving file names |
| `-v`, `--verbose` | Always output headers giving file names |
| `-h`, `--help` | Print help |
| `-V`, `--version` | Print version |

## Examples

Display the last 10 lines of a file (default):

```sh
tailrsc myfile.txt
```

Display the last 20 lines:

```sh
tailrsc -n 20 myfile.txt
```

Display the last 500 lines:

```sh
tailrsc -n 500 foo
```

Display the last 100 bytes:

```sh
tailrsc -c 100 myfile.txt
```

Follow a file as it grows:

```sh
tailrsc -f /var/log/syslog
```

Follow a file and detect renames/rotation:

```sh
tailrsc -F /var/log/messages
```

Read from the beginning and then follow:

```sh
tailrsc -F -n +1 /var/log/messages
```

Display multiple files with headers:

```sh
tailrsc -v file1.txt file2.txt
```

### Error Handling

When a file cannot be opened, `tailrsc` prints a BSD-compatible error to stderr and exits with a non-zero status code:

```sh
$ tailrsc missing.txt
tailrsc: missing.txt: No such file or directory

$ tailrsc file1.txt missing.txt file2.txt
tailrsc: missing.txt: No such file or directory
```

All errors are reported before exiting, matching BSD `tail` behaviour.

## Man Page

### TAIL(1) - General Commands Manual

#### Synopsis

```
tail [-F | -f | -r] [-qv] [-b number | -c number | -n number] [file ...]
```

#### Description

The `tail` utility displays the contents of *file* or, by default, its
standard input, to the standard output.

The display begins at a byte, line or 512-byte block location in the
input. Numbers having a leading plus (`+`) sign are relative to the
beginning of the input, for example, `-c +2` starts the display at the
second byte of the input. Numbers having a leading minus (`-`) sign or
no explicit sign are relative to the end of the input, for example, `-n 2`
displays the last two lines of the input. The default starting location
is `-n 10`, or the last 10 lines of the input.

The options are as follows:

**`-b number`**, **`--blocks=number`** — The location is *number* 512-byte blocks.

**`-c number`**, **`--bytes=number`** — The location is *number* bytes.

**`-f`** — The `-f` option causes `tail` to not stop when end of file is reached,
but rather to wait for additional data to be appended to the input. The
`-f` option is ignored if the standard input is a pipe, but not if it is
a FIFO.

**`-F`** — The `-F` option implies the `-f` option, but `tail` will also check to
see if the file being followed has been renamed or rotated. The file is
closed and reopened when `tail` detects that the filename being read from
has a new inode number. If the file being followed does not (yet) exist
or if it is removed, `tail` will keep looking and will display the file
from the beginning if and when it is created. The `-F` option is the same
as the `-f` option if reading from standard input rather than a file.

**`-n number`**, **`--lines=number`** — The location is *number* lines.

**`-q`**, **`--quiet`**, **`--silent`** — Suppresses printing of headers when multiple files are being examined.

**`-r`** — The `-r` option causes the input to be displayed in reverse order, by
line. Additionally, this option changes the meaning of the `-b`, `-c`
and `-n` options. When the `-r` option is specified, these options specify
the number of bytes, lines or 512-byte blocks to display, instead of the
bytes, lines or blocks from the beginning or end of the input from which
to begin the display. The default for the `-r` option is to display all
of the input.

**`-v`**, **`--verbose`** — Prepend each file with a header.

If more than a single file is specified, or if the `-v` option is used,
each file is preceded by a header consisting of the string `==> XXX <==`
where *XXX* is the name of the file. The `-q` flag disables the printing
of the header in all cases.

All *number* arguments may also be specified with size suffixes supported
by `expand_number(3)`.

#### Exit Status

The `tail` utility exits **0** on success, and **>0** if an error occurs.

## Build & Run Commands

- **Build:** `cargo build`
- **Run:** `cargo run`
- **Test:** `cargo test`
- **Run single test:** `cargo test <test_name>`
- **Check (fast compile check):** `cargo check`
- **Format:** `cargo fmt`
- **Lint:** `cargo clippy`
