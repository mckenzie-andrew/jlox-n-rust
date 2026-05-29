use std::env;
use std::fs;
use std::process;
use std::io::{self, Write};
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        // 1 is always the files name.
        println!("Usage: jlox [script]");
        // 64 convention from book.
        process::exit(64);        
    } else if args.len() == 2 {
        match run_file(args[1].as_str()) {
            Ok(_) => {},
            Err(error) => {
                eprintln!("Error: {}", error);
                // TODO: insert appropriate exit code
                process::exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => {},
            Err(error) => {
                eprintln!("Error: {}", error);
                // TODO: insert appropriate exit code
                process::exit(1);
            }
        }
    }
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let bytes = fs::read(path)?;
    run(str::from_utf8(&bytes)?);
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let result = io::stdin().read_line(&mut input)?;
        
        if result == 0 { break; }
        run(&input);
    }

    Ok(())
}


fn run(source: &str) {
    // Scanner scanner = new Scanner(source);
    // List<Token> tokens = scanner.scanTokens();

    // for (Token token : tokens) {
    //  System.out.println(token);
    // }
    let chars = source.chars();
    println!("{:?}", chars);
}