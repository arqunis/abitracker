# Abitracker

This is a useless program for printing how many packages had their versions
change compared to their release numbers after an Arch Linux system has been
updated by pacman.

## Installation

This program is written in [Rust][rust]. After installing a Rust toolchain,
build the program with:

```
$ cargo build --release --locked
```

Then copy it over to `/usr/bin`:
```
$ sudo cp target/release/abitracker /usr/bin
```

Afterwards, install the hook to run the program after an update:
```
$ sudo mkdir -p /usr/pacman.d/hooks
$ sudo cp 100-abitracker.hook /usr/pacman.d/hooks
```

You are done! Simply update your system with `pacman -Syu` and you should be
greeted with a message like:

```
[abitracker]: Packages upgraded today had 41 legitimate upgrades, versus 6 that had to be rebuilt due to other packages
```

## License

Project is licensed under the [MIT License](LICENSE.txt). Its text should be
found within the directory of this README. If it is missing, you may find an
online copy of its text at https://opensource.org/license/mit/

[rust]: https://rust-lang.org
