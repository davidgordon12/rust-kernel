A personal OS written in Rust

# Prerequistes
- Install nasm
- Install make
- Install qemu

## Build instructions
- Clone the repository
- Run ```make``` to compile and link the assembly files
- Run ```make iso``` to create the ISO image
- Run ```make run``` to run qemu with the image

### Footnotes
I am developing on Manjaro, but can be done on Mac or Windows with WSL.

For Mac users, you will want to install BINUTILS and GRUB. Juan Moreno (https://github.com/jupmoreno) has graciously created a brew script for this.

(This should install everything necessary) <br />
```brew install jpmrno/apps/x86_64-elf-gcc```

