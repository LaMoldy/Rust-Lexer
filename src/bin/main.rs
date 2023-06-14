use std::{fs::File, io::{self, Read}};

use molgylang::lexer::lexer::Lexer;

fn valid_argument_count(args: &Vec<String>) -> bool {
    const MAX_ARGUMENT_COUNT: usize = 2;
    args.len() == MAX_ARGUMENT_COUNT
}

fn valid_file_type(file_path: &String) -> bool {
    const EXPECTED_FILE_PATH: &str = ".molgy";
    file_path.ends_with(EXPECTED_FILE_PATH)
}

fn get_file_content(file_path: &String) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut file_content = String::new();

    file.read_to_string(&mut file_content)?;

    Ok(file_content)
}

fn main() -> Result<(), &'static str> {
    // Gets the arguments
    let args: Vec<String> = std::env::args().collect();
    
    // Checks the amount of arguments provided 
    if !valid_argument_count(&args) {
        return Err("Invalid amount of arguments provided");
    }

    // Checks the file type
    const FILE_PATH_POSITION: usize = 1;
    if !valid_file_type(&args[FILE_PATH_POSITION]) {
        return Err("Invalid file type provided")
    }

    // Gets the file contents
    let file_content = match get_file_content(&args[FILE_PATH_POSITION]) {
        Ok(content) => content,
        Err(_) => return Err("Could not open and read file provided")
    };

    let mut lex = Lexer::new(file_content);

    let tokens = lex.get_tokens();

    for tok in tokens {
        println!("{}", tok);
    }

    Ok(())
}
