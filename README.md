# Advent of Code 2022

This repository contains solutions to https://adventofcode.com/2022 written in
Rust.

Input is not stored alongside this repository; instead, the program expects each
day's input to be stored in `./input/day#.txt`, for example `./input/day2.txt`
or `./input/day20.txt`.

## Usage with NixOS

Use the Nix flake-based ecosystem for working with this repository.
`nix develop` enters a shell environment with the necessary dependencies
available. It's also possible to automatically enter the development environment
using a tool like `nix-direnv`.
