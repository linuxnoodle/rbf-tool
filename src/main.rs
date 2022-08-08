use std::env;
use std::fs;

mod interpreter;
use interpreter::interpret;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("rbf-interpreter: incorrect amount of arguments provided\nTry `rbf-interpreter --help` for more information");
        return;
    }
    for i in 1..args.iter().len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                println!("rbf-interpreter: usage: rbf-interpreter [OPTIONS]... [FILE]
                -h, --help                      shows this help message
                -c, --compile [OUTPUT_NAME]     [TODO] compiles the file to LLVM
                -v, --visualize                 [TODO] shows the memory of the program in a TUI browser");
                std::process::exit(0);
            },
            "--compile" | "-c" => {
                if args.len() < 3 {
                    println!("rbf-interpreter: incorrect amount of arguments provided\nTry `rbf-interpreter --help` for more information");
                    return;
                }
                println!("rbf-interpreter: compiling not yet implemented");
            },
            "--visualize" | "-v" => {
                println!("rbf-interpreter: visualization not yet implemented");
            },
            _ => {
                println!("rbf-interpreter: unknown argument `{}`", args[i]);
                std::process::exit(1);
            }
        }
    }
    if !std::path::Path::new(&args.last().unwrap()).exists() {
        println!("rbf-interpreter: file `{}` does not exist", args.last().unwrap());
        std::process::exit(1);
    }
    let code = fs::read_to_string(&args.last().unwrap())
        .expect("rbf-interpreter: could not read file");
    interpret(&code);
}
