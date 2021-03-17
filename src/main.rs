use std::io::{self, Write};
mod db;
mod db_test;
pub enum Commands {
    CommandExit,
    CommandError
}
pub enum InputType {
    Command,
    Statement
}

pub enum Res {
    Success,
    Fail,
    Exit
}
pub fn generate_command( input: &String) -> Commands{ 

    match &input[..] {
        ".exit" => Commands::CommandExit,
        _ => Commands::CommandError,
    } 
}
fn grab_input() -> String { 

    let mut input = String::new();
    io::stdin().read_line(&mut input)
    .ok()
    .expect("Couldn't read line");   
    return input.trim().to_string();
}

fn print_prompt(){ 
    print!(" > " ); 
    io::stdout().flush().unwrap();
}

pub fn generate_input_type(input: &String) -> InputType{ 

    if input.chars().nth(0).unwrap() == '.' {
        InputType::Command
     } else {
         InputType::Statement
     } 
}

pub fn execute_command(input: &String) -> Res {

    let cmd = generate_command(input); 
    let res = match cmd {
        Commands::CommandExit => Res::Exit,
        Commands::CommandError => Res::Fail
    }; 
    res
}


pub fn execute_statement(input: &String) -> Res {
    Res::Success
}


fn main() { 

    loop { 
        print_prompt(); 
        let input = grab_input();
        if input.is_empty() { continue }
        let input_type = generate_input_type(&input);

        let result = match input_type {
            InputType::Command => execute_command(&input),
            InputType::Statement => execute_statement(&input),
        };
        match result {
            Res::Exit => break,
            Res::Success => continue,
            Res::Fail => println!("Error, wrong statement or command"),
        }
    }
}