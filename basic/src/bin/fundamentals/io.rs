use std::io::{ stdin, stdout, Write };

/// **Description:**
///
/// Input/Output Methods
fn main() {
    print!("\nWhat is your name ? ");

    // Name of the User
    let mut name = String::new();

    stdout().flush().expect("Failed to flush stdout");
    stdin().read_line(&mut name).expect("Didn't Receive Input");

    println!("\nHello {}", name.trim());
    println!();
}
