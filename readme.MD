A minimal kernel written in Rust

# Prerequistes
- nasm
- make
- xorriso
- grub
- qemu

## Build instructions
1. Clone the repository
2. Install rustup
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
3. Switch to rust nightly
```rustup override add nightly```
4. Add rust-src
```rustup component add rust-src```
5. Install xargo
```cargo install xargo```
6. Build the project with the following
- Run ```make``` to compile and link all neccessary files
- Run ```make iso``` to create the ISO image
- Run ```make run``` to run qemu with the image (this will also run the previous two commands)

## Troubleshooting
  In case of a compile error after the previous build instructions, try to run <br />
  ```cargo clean``` <br />
  followed by <br />
  ```make iso``` <br />

### Footnotes
I am developing on Manjaro, but can be done on Mac or Windows with WSL, with some tweaks to the makefile.

For Mac users, you will want to install BINUTILS and GRUB. <br /> Juan Moreno (https://github.com/jupmoreno) has graciously created a brew script for this.

```brew install jpmrno/apps/x86_64-elf-gcc``` 

This should install everything necessary to build this project, though your mileage may vary<br />
