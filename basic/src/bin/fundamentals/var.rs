// MetaData -> Attributes
#![allow(unused)]

use std::cmp::Ordering;

use colored::Colorize;
use rand::Rng;
/// **Description:** Variables and Types
/// - Immutable Variables
/// - Mutable Variables
/// - Variable Overriding
fn main() {
    // UnSigned Variables
    let a1: i8 = i8::MAX;
    let a2: i16 = i16::MAX;
    let a3: i32 = i32::MAX;
    let a4: i64 = i64::MAX;
    let a5: i128 = i128::MAX;
    let a6: isize = isize::MAX;

    // UnSigned Variables
    let b1: u8 = u8::MAX;
    let b2: u16 = u16::MAX;
    let b3: u32 = u32::MAX;
    let b4: u64 = u64::MAX;
    let b5: u128 = u128::MAX;
    let b6: usize = usize::MAX;

    // Float Variables
    let f1: f32 = f32::MAX;
    let f2: f64 = f64::MAX;

    // Boolean Variables
    let c1 = true;
    let c2 = 3 > 4;
    let c3 = if 5 < 10 { true } else { false };
    let c4 = match a1 {
        i8::MAX => true,
        _ => false,
    };

    // Character Variables
    let d1 = 'A';
    let d2 = '@';

    // String Variables
    let e1 = "Hello World !";
    let e2 = String::from("Hello World !");
    let mut e3 = String::new();
    e3.push('A');
    e3.push(' ');
    e3.push_str("Bird");
    for word in e3.split_whitespace() {
        print!("{}", word);
    }
    println!();
    let e4 = e3.replace("A", "Another");
    println!("Replaced: {}", e4);
    let mut e5 = e1.to_string();
    let e6 = e5.as_bytes();
    let e7 = &e5[1..5]; // Substring via Range
    println!("StringBytes: {:?}", e6);
    println!("Substring: {:?}", e7);
    println!("Substring Length: {}", e7.len());
    // Clears all connected references or pointers etc
    e5.clear();
    let e8 = String::from("Coco melon");
    let e9 = String::from(" very funny !");
    // e8 is cloned & deleted while e9 offers reference
    let e10 = e8 + &e9;
    println!("S2: {}", e9);
    println!("S3: {}", e10);

    // Vectors
    let k1 = vec![1, 2, 3];
    // iterators can be collected into vectors
    let k2: Vec<i32> = (1..10).collect();
    let k3_str = String::from("a b c d");
    let k3_chars = k3_str.chars();
    println!("Chars[k3]: {:?}", k3_chars);
    let mut k3: Vec<char> = k3_chars.collect();
    k3.sort();
    k3.dedup(); // Delete duplicates
    for char in k3 {
        print!("{}", char);
    }
    println!();
    let mut k4 = Vec::new();
    k4.push(47);
    k4.push(13);
    k4.push(25);
    let k4_first = &k4[0];
    let k4_second = &k4[1];
    match k4.get(1) {
        Some(first) => print("Found: {}", first),
        Some(13) => print("Found: {}", 13),
        None => println!("No Matches"),
    }

    // Variable Overriding
    let age = "47";
    println!("Age [String]: {}", age);
    let mut age: i32 = age.trim().parse().expect("Age not Assigned");
    age = age + 1;
    println!("Age [Number]: {}", age);

    // Type-Casting
    let l1: u8 = 7;
    let l2 = l1 as i32;

    // Arithmetic Operators
    let num1 = 7;
    let num2 = 4;
    let num3 = 8.23;
    let num4 = 2.17;
    // Arithmetics with Integers
    println!("[Add] {}", num1 + num2);
    println!("[Subtract] {}", num1 - num2);
    println!("[Multiply] {}", num1 * num2);
    println!("[Divide] {}", num1 / num2);
    println!("[Remainder] {}", num1 % num2);
    // Arithmetics with Floats
    println!("[Add] {}", num3 + num4);
    println!("[Subtract] {}", num3 - num4);
    println!("[Multiply] {}", num3 * num4);
    println!("[Divide] {}", num3 / num4);
    println!("[Remainder] {}", num3 % num4);

    // Flow-Control -> IfElse
    let age = 8;
    if age >= 50 || age < 10 {
        println!("Not Important Birthday");
    } else if age < 50 && age >= 18 {
        println!("Normal Birthday");
    } else {
        println!("Important Birthday");
    }

    // Flow-Control -> Match
    let age = 10;
    match age {
        1..=9 => println!("Not Important Birthday"),
        10..=18 => println!("Important Birthday"),
        19..=49 => println!("Never Important Birthday"),
        50 | 60 => println!("Death Bed"),
        _ => println!("Mysterious"),
    }

    // FlowControl -> Match -> Comparison/Ordering
    let age = 13;
    let std_age = 18;
    match age.cmp(&std_age) {
        Ordering::Less => println!("Young"),
        Ordering::Greater => println!("Old"),
        Ordering::Equal => println!("Perfect"),
    }

    // Range Operator
    let mut g = 1..10;
    let h = 1..=10;
    println!("Range[g]: {:?}", g);
    println!("Range[g.next]: {:?}", g.next());
    println!("Range[g.next.unwrap]: {:?}", g.next().unwrap());
    for item in h {
        println!("Range[h]: {:?}", item);
    }

    // Arrays
    let i = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("i: {:?}", i);
    println!("i [Length]: {:?}", i.len());

    // FlowControl -> Loop
    let i = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut index = 0;
    loop {
        if index >= i.len() {
            break;
        }
        if i[index] % 2 == 0 {
            index += 1;
            continue;
        }
        println!("Loop[{}]: {}", index, i[index]);
        index += 1;
    }

    // FlowControl -> While Loop
    let i = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut index = 0;
    while index < i.len() {
        println!("While[{}]: {}", index, i[index]);
        index += 1;
    }

    // FlowControl -> For Loop
    let i = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for item in i.iter() {
        println!("For: {}", item);
    }

    // Tuples
    let j = (32, "Zaeem", 8.9);
    println!("Tuple[j]: {:?}", j);
    println!("Tuple[j][0]: {:?}", j.0);
    // Destructuring
    let (v1, v2, v3) = j;
    println!("Tuple[j] -> v1={}, v2={}, v3={}", v1, v2, v3);

    // Library
    // Generating Random Number
    let rand_num = rand::thread_rng().gen_range(1..10);
    println!("Random Number: {}", rand_num);

    // Enums
    #[derive(Debug)]
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
    impl Day {
        /// **Description:** The methods finds out if the days selected is a weekend or not.
        ///
        /// **Parameters:**
        /// - `&self`:`{any}` - The value of enum itself
        ///
        /// **Returns:**
        /// - `bool` - Is weekend or not
        ///
        /// **Use:**
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }
    let today = Day::Monday;
    println!("Today [Enum]: {:?}", today);
    println!("Today is Weekend: {:?}", today.is_weekend());
}
