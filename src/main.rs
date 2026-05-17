use std::io;

use anyhow::Result;

mod scanner;
mod token;

fn main() -> Result<()> {
    // Test the scanner with user input.
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    match scanner::into_tokens(&buf) {
        Ok(tokens) => {
            dbg!(tokens);
        }
        Err(e) => println!("{e}"),
    }
    Ok(())
}
