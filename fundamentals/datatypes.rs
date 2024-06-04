#![allow(unused_assignments)]
#![allow(unused)]

use std::any::type_name;

/**
 * Application's Main Function
 * @author Zaeem Hassan Tarar
 * @since 04-2024
 * @topic Data-Types
 */
fn main() {
    // Constants
    println!("\n[Constant]");
    const A: i32 = 128;
    println!("A: {}", A);

    // Variables
    println!("\n[Variable]");
    let a: i32 = 1_000;
    println!("a: {}", a);

    // Mutable
    println!("\n[Mutable Variable]");
    let mut b: i32 = 28;
    b = 54;
    println!("b: {}", b);

    // Signed Integer
    println!("\n[Signed Integer]");
    let c: i8 = i8::MIN;
    let c2: i8 = i8::MAX;
    let d: i16 = i16::MIN;
    let d2: i16 = i16::MAX;
    let e: i32 = i32::MIN;
    let e2: i32 = i32::MAX;
    let f: i64 = i64::MIN;
    let f2: i64 = i64::MAX;
    let g: i128 = i128::MIN;
    let g2: i128 = i128::MAX;

    println!("c: {},{}", c, c2);
    println!("d: {},{}", d, d2);
    println!("e: {},{}", e, e2);
    println!("f: {},{}", f, f2);
    println!("g: {},{}", g, g2);

    // UnSigned Integer
    println!("\n[Unsigned Integer]");
    let h: u8 = u8::MIN;
    let h2: u8 = u8::MAX;
    let i: u16 = u16::MIN;
    let i2: u16 = u16::MAX;
    let j: u32 = u32::MIN;
    let j2: u32 = u32::MAX;
    let k: u64 = u64::MIN;
    let k2: u64 = u64::MAX;
    let l: u128 = u128::MIN;
    let l2: u128 = u128::MAX;
    let l3: u8 = b'\x00';
    let l4: u32 = b'\x33'.into();

    println!("h: {},{}", h, h2);
    println!("i: {},{}", i, i2);
    println!("j: {},{}", j, j2);
    println!("k: {},{}", k, k2);
    println!("l: {},{}", l, l2);
    println!("l3: {}", l3);
    println!("l4: {}", l4);

    // Float
    println!("\n[Float]");
    let m: f32 = f32::MIN;
    let m2: f32 = f32::MAX;
    let n: f64 = f64::MIN;
    let n2: f64 = f64::MAX;

    println!("m: {},{}", m, m2);
    println!("n: {},{}", n, n2);

    // Boolean
    println!("\n[Boolean]");
    let o: bool = true;
    println!("o: {}", o);

    // Character
    println!("\n[Character]");
    let p: char = 'Z';
    println!("p: {}", p);

    // String
    println!("\n[String]");
    let q: String = String::from("Hello World !");
    let r: &str = "Hello World !";
    let mut s: String = String::new();
    s.push('A');
    s.push_str(" Okay");
    println!("q: {}", q);
    println!("r: {}", r);
    println!("s: {}", s);

    println!();
}
