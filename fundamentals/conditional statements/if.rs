#![allow(unused)]
#![allow(unused_assignments)]
/**
 * Application's Main Function
 * @author Zaeem Hassan Tarar
 * @since 04-2024
 *
 * @topic If-Else Statement
 */
fn main() {
    println!();

    /**
     * Variable: a
     * @type i32
     */
    let a = 5;

    /**
     * Normal If-Else Statement
     */
    if a > 5 {
        println!("{} - is greater than 5", a);
    } else if a < 5 {
        println!("{} - is lesser than 5", a);
    } else {
        println!("{} - is equals to 5", a);
    }

    /**
     * Assigning/Saving result from If-Else Statement
     */
    let result = if a > 5 { 1 } else if a < 5 { 2 } else { 3 };

    /**
     * Display If-Else Assignment Result
     */
    println!("Result: {}\n", result);
}
