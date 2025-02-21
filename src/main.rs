use {
    okid::OkId,
    std::io::{self, Read},
};

fn main() -> io::Result<()> {
    // Read from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
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

    Ok(())
}
