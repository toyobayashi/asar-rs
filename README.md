# asar_rs

[@electron/asar](https://github.com/electron/asar) Rust porting

I'm Rust beginner. This is my first time using rust (the code can be very ugly) and it passes the [official tests](https://github.com/electron/asar/tree/main/test).

```bash
cargo install asar_rs
```

```txt
$ asar
Creating Electron app packages

Usage: asar <COMMAND>

Commands:
  pack          create asar archive
  list          list files of asar archive
  extract-file  extract one file from archive
  extract       extract archive
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

# asar pack ...
$ asar p ./app ./app.asar

# asar extract ...
$ asar e ./app.asar ./_app
```
