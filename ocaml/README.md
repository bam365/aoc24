# aoc24 ocaml solutions

I'm trying `nix` this year for reproducible builds. I'm using
[opam-nix](https://github.com/tweag/opam-nix) for that. To take advantage of
this, you need to have `nix` installed and nix flakes enabled. I think you
also need to be running a Linux x86_64 system.

To start a development shell (similar to activating a python virtual env), you
run `nix develop` in this directory. The first time you do this, it will install
all the necessary toolchain stuff (ocaml compiler, opam, ocaml-lsp, project opam
dependencies like `containers`) so it will take a few minutes. The cool thing
is you shouldn't need any system dependencies (other than `nix`) to use this
project, and theoretically when I come back to this project 5 years from now
on a different machine it will build just like it does today.

Then run an editor from inside that shell to do code editing. Anything that supports
`ocaml-lsp` should just work.

Use `dune` as normal from inside that shell to build and run the project

`dune build` - build the project
`dune exec bamaoc24 1` - run the solution for day 1 part 1
