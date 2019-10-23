use std::convert::AsRef;
use std::io;

pub enum Commands {
  Calculate,
  Help,
  Quit,
}

pub struct UserInput {
  pub command: Commands,
  pub text: String,
}

pub fn get_input() -> UserInput {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let text = input.trim().to_string();
  let command = match text.as_ref() {
    "/help" => Commands::Help,
    "/quit" => Commands::Quit,
    _ => Commands::Calculate,
  };
  UserInput { command, text }
}
