# rust-unix-shell

A small Unix-style command-line shell written in Rust. It was the final project for the Rust Programming course.

Every command is built into the shell itself, using Rust's standard library and Unix file metadata. The shell does not start external programs.

## Features

- **Nine built-in commands:** `ls`, `cd`, `pwd`, `cat`, `write`, `rm`, `du`, `help` and `exit`
- **Its own input tokenizer:** single and double quotes, backslash escapes, and clear errors for unterminated quotes or a trailing backslash
- **Paths with spaces:** quote or escape them (`cat "my notes.txt"`, `cd My\ Folder`)
- **Help for every command:** `-h` on any command, or `help <command>`
- **Errors on stderr:** errors are reported without panicking, so a bad command never ends the session
- **Its own working directory:** the shell tracks it separately from the process's directory, and `cd` with no arguments goes to `$HOME`

## Commands

| Command | Description |
|---|---|
| `ls [-l] [-a] [<path>]` | List directory contents. `-l` shows permissions, size and modification time; `-a` includes hidden entries |
| `cd [<path>]` | Change directory (absolute or relative path, `~`); no argument goes to `$HOME` |
| `pwd` | Print the absolute path of the working directory |
| `cat <file> [<file>...]` | Print one or more files |
| `write <name>` | Write input line by line into a file (created or overwritten); a line with just `q` ends input |
| `rm [-r] <name> [<name>...]` | Remove files, or directory trees with `-r` |
| `du [-a] [-b] [-c] [-h] [<path>]` | Estimate disk usage. `-a` lists all files, `-b` shows sizes in bytes, `-c` adds a grand total, `-h` gives human-readable sizes |
| `help [<command>]` | List the commands, or show help for one |
| `exit` | Quit (Ctrl-D also works) |

## Building and running

The project needs Rust 1.85 or newer, because it uses the 2024 edition.

```sh
git clone https://github.com/Rayska/rust-unix-shell.git
cd rust-unix-shell
cargo run --release
```

The only dependency is [chrono](https://crates.io/crates/chrono), which formats the timestamps in `ls -l`.

**Platform:** Linux and macOS, or WSL on Windows. `ls -l` and `du` read Unix file permissions and block counts through `std::os::unix::fs`, so the project does not build on native Windows.

## Example session

```text
$ write "my notes.txt"
hello world
second line
q
$ ls -l
-rw-r--r--       24 2026-11-06 22:01 "my notes.txt"
drwxr-xr-x     4096 2026-11-06 22:01 notes
$ cat "my notes.txt"
hello world
second line
$ cd notes
$ pwd
/home/ray/demo/notes
$ cd ..
$ du -h -a
4.0K	/home/ray/demo/my notes.txt
4.0K	/home/ray/demo/notes
12K	/home/ray/demo
$ rm "my notes.txt"
$ ls -x
Unknown option: x
...
$ exit
```

## Project structure

```text
src/
├── main.rs              # read–tokenize–dispatch loop, help
└── shell/
    ├── structures.rs    # CmdResult enum, display helpers
    └── commands/        # one module per command: ls, cd, pwd, cat, write, rm, du
```

Each command returns a `CmdResult` (`Ok`, `Exit` or `Unknown`), so control flow stays in `main()`.

## Limitations

The shell runs only its built-in commands. It has no external programs, pipes, redirection, environment-variable expansion or command history.

The course report (`RaySointulaRustUnixShell.pdf`) describes the design in more detail.
