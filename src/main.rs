use okstd::prelude::*;
use {
    okid::OkId,
    std::io::{self, Read},
};

#[okstd::main]
async fn main() {
    // Read from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    let second_input = input;

    match (input.parse::<OkId>(), OkId::from_display_safe(second_input)) {
        (Ok(id), None) => {
            println!("{}", id.display_safe());
        }
        (Err(_), Some(id)) => {
            println!("{}", id);
        }
        _ => {
            eprintln!("Invalid input: {}", input);
            std::process::exit(1);
        }
    }
}
