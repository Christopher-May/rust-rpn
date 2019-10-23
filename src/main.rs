mod user_input;
use user_input::get_input;
use user_input::Commands;

mod calculator;
use calculator::calculate_rpn;

fn main() {
    print!("{}[2J", 27 as char);
    print!(
        "
    space between operands and operators \n
    positive and negative operands are supported \n
    + - * / are supported \n
    have fun ;) \n
    "
    );
    loop {
        let input = get_input();
        match input.command {
            Commands::Calculate => {
                println!("{}", calculate_rpn(input.text));
            }
            Commands::Quit => break,
            Commands::Help => {
                println!("learn how to use this by yourself fool!");
            }
        }
    }
}
