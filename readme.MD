A personal OS written in Rust

# Prerequistes
- Install nasm
- Install make
- Install qemu

## Build instructions
1. Clone the repository
2. Install rustup
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
3. Add rust nightly
```rustup override add nightly```
4. Install xargo
```cargo install xargo```
5. Build the project with the following
- Run ```make``` to compile and link the assembly files and lib.rs
- Run ```make iso``` to create the ISO image
- Run ```make run``` to run qemu with the image (this will also run the previous two commands)

### If you recieve any build errors, try running cargo clean in the working directory, and then try make again

### Footnotes
I am developing on Manjaro, but can be done on Mac or Windows with WSL.

For Mac users, you will want to install BINUTILS and GRUB. <br /> Juan Moreno (https://github.com/jupmoreno) has graciously created a brew script for this.

```brew install jpmrno/apps/x86_64-elf-gcc``` 

This should install everything necessary to build this project, though your mileage may vary<br />
