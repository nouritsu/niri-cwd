## About

A program to print the focused window's working directory in Niri.

## Installation

Using cargo
```sh
$ cargo install --git https://github.com/nouritsu/niri-cwd
```

Try it out in a `nix shell`.
```sh
$ nix shell github:nouritsu/niri-cwd#default
```

Install using flakes
```nix
{
  /* in flake.nix */
  inputs = {
    /* ... */
    niri-cwd.url = "github:nouritsu/niri-cwd";
  };

  /* then in configuration.nix */
  environment.systemPackages = let
    inherit (pkgs.stdenv.hostPlatform) system;
  in [
    inputs.niri-cwd.packages.${system}.niri-cwd
    # drop `inputs.` if you do not use inputs
  ];
}
```

## Usage

Upon installation, you can run the program using the `niri-cwd` binary.
Running `niri-cwd` in a terminal is identical to running `pwd`.

Run `sleep 3 && niri-cwd`, then quickly switch to another window to see the window's working directory.
The program will print a human readable error if something goes wrong (eg: ran from a TTY, no active window).

If you don't want the program to fail and instead print a fallback directory, run
```sh
$ niri-cwd -d /path/to/fallback/dir # --default-dir or -d
```

For help:
```sh
$ niri-cwd --help
Usage: niri-cwd [OPTIONS]

Options:
  -d, --default-dir <DIR>  Directory to output in case of error
  -h, --help               Print help
```
