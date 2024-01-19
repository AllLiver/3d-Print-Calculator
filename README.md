# 3d Print Calculator
A super simple and versatile CLI to calculate the price of 3d prints.  
### Features
- Saved filament configurations
- Easy to use config.txt
- Single executable
# How to use
- Run the executable and if it is your first time fill out the config.txt
- Enter "create" and follow the prompts if you wish to create a filament config, this config will be written to the config.txt
- Enter "calc" and follow the prompts if you wish to calculate a print
- Enter "exit" to exit the program
# Installation
You can install it in a few ways:
### Windows
Download the latest release from releases or compile it yourself (Follow MacOS and Linux instructions)
### MacOS and Linux
Unfortunately I have not compiled binaries for these platforms, so you will have to compile it yourself  
- Make sure you have [Rust](https://rustup.rs/ "Rustup link") installed
- Download the repository and unzip it
- Navigate to the folder with the repository in it with a terminal
- Then run:
```
cargo build --release
```
- If everything works out, a new binary should be in /target/release
# Configuration
- Do not edit configuration names and only edit values, any incorrect value types will cause the program to crash
- Any lines starting with a "!" mark the beginning of a category, don't mess with or worry about these
- Filament configurations are added via the main program
- Any line starting with a "#" is a commented line
- Instructions are on config.txt too
