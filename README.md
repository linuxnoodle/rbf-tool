<p align="center"><img src="rbf.png" height="200"/></p>
<h1 align="center">rbf-tool</h1>
BF interpreter (and hopefully soon to be compiler) written according to the [esolang.org](https://esolangs.org/wiki/Brainfuck) specification (this time in rust!).
## Compilation:
```
git clone https://github.com/linuxnoodle/rbf-tool
cd rbf-tool
cargo build --release
```
## Usage:
```
rbf-interpreter: usage: rbf-interpreter [OPTIONS]... [FILE]
    -h, --help                      shows this help message
    -c, --compile [OUTPUT_NAME]     [TODO] compiles the file to LLVM
    -v, --visualize                 [TODO] shows the memory of the program in a TUI browser
```
## TODO:
- [  ] LLVM compilation
- [  ] TUI memory visualization
