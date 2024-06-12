// MetaData -> Attributes
//! Attributes
// #![windows_subsystem = "windows"]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused)]
#[warn(missing_docs)]
#[deny(missing_docs)]
// #![doc = include_str!("./comments.md")]
// #[doc(inline)]

// Libraries
use std::process;
use std::cmp::Ordering;
use std::fs::File;
use std::collections::HashMap;
use colored::Colorize;
use std::{ error, io };
use std::io::{ Write, BufReader, BufRead, ErrorKind };
use rand::{ Error, Rng };
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{ Arc, Mutex };
use std::any::type_name;
use std::ops::{ Add, Mul, Div, Sub, Range, RangeToInclusive, RangeInclusive };
use std::mem::size_of_val;
// use crate::List::*;

/// **Description:** Variables and Types
/// - Immutable Variables
/// - Mutable Variables
/// - Variable Overriding

mod tester;
mod tester2;
mod tester3;
use crate::tester3::tester3::experiment3::content3;

mod pizza_order {
    pub struct Pizza {
        donut: String,
        sauce: String,
        topping: String,
    }
    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            return Pizza {
                donut: String::from("chocolate"),
                sauce: String::from("mustard"),
                topping: String::from(topping),
            };
        }
    }
    pub mod help_customer {
        fn seat_at_table() {
            println!("customer seated at table")
        }
        pub fn take_order() {
            seat_at_table();
            let cust_pizza: super::Pizza = super::Pizza::lunch("cheese");
            serve_customer(cust_pizza);
        }
        fn serve_customer(cust_pizza: super::Pizza) {
            println!("Pizza served with extra {}", cust_pizza.topping);
        }
    }
}

fn main() {
    // Constants
    const PI: f32 = 3.14159;

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

    // Vectors (List)
    let k1 = vec![1, 2, 3];
    // iterators can be collected into vectors
    let k2: Vec<i32> = (1..10).collect();
    let k3_str = String::from("a b c d");
    let k3_chars = k3_str.chars();
    println!("Chars[k3]: {:?}", k3_chars);
    let mut k3: Vec<char> = k3_chars.collect();
    k3.sort();
    k3.dedup(); // Delete duplicates
    for chr in k3 {
        print!("{}", chr);
    }
    let mut k5 = vec![1, 2, 3, 4, 5, 6];
    // Mutable
    for item in &mut k5 {
        *item *= 2;
    }
    // Immutable
    for item in &k5 {
        print!("{}", item);
    }
    println!("VectorPop: {:?}", k5.pop());
    let mut k4 = Vec::new();
    k4.push(47);
    k4.push(13);
    k4.push(25);
    let k4_first = &k4[0];
    let k4_second = &k4[1];
    match k4.get(1) {
        Some(first) => println!("Found: {}", first),
        Some(13) => println!("Found: {}", 13),
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

    // Functions
    fn say_hello() {
        println!("Hello !!!");
    }
    fn calculate(x: i32, y: i32) -> (i32, i32) {
        // Returning Tuple
        (x + 1, y + 2)
    }
    say_hello();
    println!("Calculate: {:?}", calculate(5, 8));
    let (v1, v2) = calculate(2, 11);
    println!("Calculate: {},{}", v1, v2);

    fn list_sum(list: &[i32]) -> i32 {
        let mut sum: i32 = 0;
        for &item in list.iter() {
            sum += &item;
        }
        return sum;
    }
    let list: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("ListSum: {}", list_sum(&list));

    // Generics
    fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
        return x + y;
    }
    println!("GenSum: {}", get_sum_gen(3, 5));
    println!("GenSum: {}", get_sum_gen(3.5, 5.4));

    // Ownership -> Stack/Heap/Scope
    let str1 = String::from("Hello");
    let str2 = str1;
    // Now, str1 does not exist as str2
    // has borrowed its value
    let str1 = String::from("Hello");
    let str3 = str1.clone(); // value copied
    fn print_str(st: String) {
        println!("{}", st)
    }
    fn return_str(st: String) -> String {
        println!("{}", st);
        return st;
    }
    fn change_str(st: &mut String) {
        st.push_str("Wow");
        println!("{}", st);
    }
    print_str(str1.clone());
    return_str(str1);
    let mut str4 = String::from("Hello");
    change_str(&mut str4);

    // HashMap
    let mut m = HashMap::new();
    m.insert("p", "power");
    m.insert("t", "tower");
    m.insert("fl", "flower");
    println!("m [Length]: {}", m.len());
    for (k, v) in m.iter() {
        println!("{}:{}", k, v);
    }
    if m.contains_key(&"fl") {
        let val = m.get(&"fl");
        match val {
            Some(x) => println!("Found: {}", x),
            None => println!("Nothing Found"),
        }
    }

    // Struct
    #[derive(Debug)]
    struct Customer {
        name: String,
        age: i32,
        balance: f64,
    }

    let mut cust1 = Customer {
        name: String::from("Zaeem"),
        age: 22,
        balance: 10_500.034,
    };
    cust1.balance = cust1.balance + 300.197;
    println!("Customer: {:?}", cust1);

    // Traits
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    // Structs with Generics
    struct Rectangle<T, U> {
        length: T,
        width: U,
    }
    impl Rectangle<i32, i32> {
        fn display(&self) {
            println!("Hello World !");
        }
    }
    struct Circle<T, U> {
        length: T,
        width: U,
    }
    // structure/object
    let mut rec1 = Rectangle {
        length: 34,
        width: 11.5,
    };

    // Trait implementation on Structs
    impl Shape for Rectangle<f32, f32> {
        fn new(length: f32, width: f32) -> Rectangle<f32, f32> {
            return Rectangle { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    impl Shape for Circle<f32, f32> {
        fn new(length: f32, width: f32) -> Circle<f32, f32> {
            return Circle { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    // Polymorphism
    let rec2: Rectangle<f32, f32> = Shape::new(10.4, 23.0);
    let cir2: Circle<f32, f32> = Shape::new(12.0, 15.3);
    println!("Rectangle Area: {}", rec2.area());
    println!("Circle Area: {}", cir2.area());

    // Modules
    crate::pizza_order::help_customer::take_order();

    tester::experiment::content();
    tester2::experiment2::content2();
    content3();

    // Errors / Exceptions
    // panic!("Mega Error !");

    // File System
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            // the following section is optional
            return match error.kind() {
                ErrorKind::NotFound => panic!("Not Found"),
                _other_error => panic!("Unknown"),
            };
        }
    };
    write!(output, "Hello World !").expect("Failed to write in file");
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    // Manual Iteration
    let mut arr1 = [1, 2, 3, 4];
    let mut iter1 = arr1.iter();
    println!("1st: {:?}", iter1.next().unwrap());
    println!("2st: {:?}", iter1.next().unwrap());
    println!("3st: {:?}", iter1.next().unwrap());
    println!("4st: {:?}", iter1.next().unwrap());
    println!("5st: {:?}", iter1.next());

    // Closure -> Function without a name
    let can_vote = |age: i32| { age > 18 };
    println!("Closure: {}", can_vote(8));
    // The order of the following code is important
    let mut num1 = 9;
    let print_num = || {
        println!("{}", num1);
    };
    print_num();
    num1 = 11;
    let mut change_num = || {
        num1 += 1;
    };
    change_num();
    println!("{}", num1);

    // Passing Closures to Functions using Generics
    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
        return func(a, b);
    }
    let sum = |a: i32, b: i32| a + b;
    println!("Sum: {}", use_func(5, 12, sum));
    println!(
        "Prod: {}",
        use_func(5, 12, |a: i32, b: i32| a * b)
    );

    // Box
    let x1 = Box::new(10);
    println!("Box [x1]: {}", x1);

    // Smart Pointers

    // Binary Tree Node using Struct with Generics
    struct TreeNode<T> {
        pub key: T,
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode { key, left: None, right: None }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }
    let node = TreeNode::new(10).left(TreeNode::new(11)).right(TreeNode::new(12));
    println!("TreeNode: {:?}", node.key);

    // Treads
    thread::sleep(Duration::from_millis(1));
    println!("[Threads]");
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Thread1: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..20 {
        println!("MainThread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    thread1.join().unwrap(); // To let all threads finish before execution

    // You can't have a situation where a closure
    // can out-live the current function
    // Arc: Gives Access/Ownership of the Variable
    // Mutex: Locks the Variable for Threads
    //          to be used one at a time

    // Thread Locks with Example
    pub struct Bank {
        balance: f32,
    }
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance - amt < 0.0 {
            println!("Insufficient Bank Balance")
        } else {
            bank_ref.balance -= amt;
            println!("Customer Withdrew: {} with {} Balance left", amt, bank_ref.balance);
        }
    }
    fn customer(the_bank: &Arc<Mutex<Bank>>) {
        withdraw(the_bank, 5.0);
    }
    let bank = Arc::new(Mutex::new(Bank { balance: 100.0 }));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(move || {
            customer(&bank_ref);
        })
    });
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Bank Total: {}", bank.lock().unwrap().balance);

    // Uninitialized
    // let x: i32;
    // assert_eq!(x, 4);
    // println!("X: {}", x)

    // Scope
    {
        println!("Hello World !");
    }

    // Shadowing
    let y = 32;
    let y = "Hello World !";

    // Destructuring
    let (mut x, y) = (1, 2);

    let (x, y);
    (x, ..) = (3, 4);
    println!("X: {}", x);
    [.., y] = [1, 2];
    // .. are left away
    println!("X: {}, Y: {}", x, y);

    // Type Casting
    let v = 38_u8 as u32;

    // Type
    fn type_of<T>(_: &T) -> &'static str {
        return type_name::<T>();
    }
    println!("{}", type_of(&v));

    // Size
    let c1: char = 'a';
    println!("Size: {}", size_of_val(&c1));

    // Range
    assert_eq!(1..5, Range { start: 1, end: 5 });
    assert_eq!(1..=5, RangeInclusive::new(1, 5));

    // Bitwise -> XOR
    let a: u8 = 0b1010; // Binary representation of 10
    let b: u8 = 0b1100; // Binary representation of 12
    let result = a ^ b; // Bitwise XOR operation
    // Print the result in binary
    println!("Result: {:b}", result);

    // Function default return type -> () Empty Tuple
    let v = (); // Empty Tuple holds 0 bytes
    fn demo() {}
    assert_eq!(v, demo());

    // Scope Return
    let y = {
        let x = 6;
        let w = 7;
        x * w // Don't use ";"
    };
    let v = {
        let x = 6;
        let w = 7;
        x * w;
    };
    println!("Y: {}, V: {:?}", y, v);

    // Diverging Function -> Unreachable Statement/Code
    fn test() -> ! {
        panic!("Unreachable Code");
        // unimplemented!();
        // todo!();
    }
    // test();
    println!("Working...");

    // Option
    let a: Option<i32> = Some(3);
    let b: Option<i32> = None;

    // Heap is slower than Stack
    // String is mutable and is stored in the heap
    // Static types get copied
    // Dynamic types get moved pointing same address
    // However, dynamic types can be cloned

    // Passing ownership
    let t = (String::from("Hello"), String::from("World"));
    let (s1, s2) = t.clone();
    println!("Ownership: {},{},{:?}", s1, s2, t);

    let t = (String::from("Hello"), String::from("World"));
    let (ref s1, ref s2) = t;
    println!("Ownership: {},{},{:?}", s1, s2, t);

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }
    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }
    fn takes_and_gives_back_ownership(st: String) -> String {
        return st;
    }
    let s1 = String::from("Hello World !");
    let n1 = 3;
    // This function takes ownership of the
    // variable "s1" and hence the variable
    // no longer exists in this scope
    takes_ownership(s1);
    // static types are always copied
    makes_copy(n1);
    let s2 = String::from("Hello World !");
    let s3 = takes_and_gives_back_ownership(s2);

    // Box -> Smart Pointer
    // Size of the data does not need to be known
    let x = Box::new(5);
    println!("X: {}, *X: {}", x, *x);

    // Partial Move
    struct Person {
        name: String,
        age: Box<u8>,
    }
    let person = Person {
        name: String::from("Zaeem"),
        age: Box::new(22),
    };
    // name is moved but
    // age is references so,
    // age can still be used
    let Person { name, ref age } = person;
    println!("{},{},{}", name, age, person.age);

    let mut s1 = String::from("Hello");
    // Pass by Immutable Reference
    fn calculate_length(st: &String) -> usize {
        return st.len();
    }
    // Pass by Mutable Reference
    fn change(st: &mut String) {
        st.push_str("world");
    }
    let s1_len = calculate_length(&s1);
    println!("Length: {}", s1_len);
    change(&mut s1);
    println!("String: {}", s1);

    // We can only have
    // ONE Mutable Reference
    // OR
    // Any number of Immutable References
    {
        let r = &s1;
        println!("Ref1: {}", r);
        // Since r won't be used again
        // Its safe to use Mutable Reference now
        let r1 = &mut s1;
        println!("Ref1: {}", r1);
    }
    let r2 = &mut s1;
    println!("Ref2: {}", r2);

    // A Function must not refer to any address
    // In other words don't leak reference out of functions

    // Memory Address
    let x: i32 = 34;
    println!("Pointer: {:p}", &x);

    // Catching Reference
    let c = 'A';
    let r1 = &c;
    let ref r2 = c;
    println!("{},{}", *r1, *r2);

    // String -> Format
    let f1 = format!("Format -> Pointer: {:p}", r1);
    println!("Format: {}", f1);

    // String Literal -> Static
    let s1 = "Hello World"; // Normal Pointer
    let s2: Box<str> = "Hello World".into(); // Smart Pointer
    fn print_literal_string(st: &str) {
        println!("Literal: {}", st);
    }
    print_literal_string(&s2);

    // String Slice
    let s1 = String::from("Hello World");
    let hello = &s1[0..5];
    let world = &s1[6..11];

    // String Replace
    let mut s = String::from("Hello World");
    let mut s1 = String::from("Hello World");
    let s2 = s1.replace("World", "There");
    let s3 = s + &s2;
    let s4 = s3.as_str();
    let s5 = s4.to_string(); // .to_owned()
    println!("Replace: {},{}", s1, s2);
    // &String -> &str

    // Can borrow mutable object as immutable

    // Raw string -> Escapes don't work here
    let x1 = r"I am writing Ru\x73\x74."; // Hex
    // Byte Escape
    let x2 = "\u{211D}"; // UniCode
    println!("UniCode: {}, {}", x1, x2);

    // Macro -> Macro Rules
    macro_rules! square {
        ($input:expr) => {
            $input * $input
        };
    }
    println!("Square: {}", square!(5));

    // Multi-Line String Literal
    // "\" escapes the new line expression
    let s1 =
        "lorem ipsum lorem ipsum lorem ipsum lorem ipsum \
        lorem ipsum lorem ipsum lorem ipsum lorem ipsum \
        lorem ipsum lorem ipsum lorem ipsum lorem ipsum \
        lorem ipsum lorem ipsum lorem ipsum lorem ipsum \
        lorem ipsum lorem ipsum lorem ipsum lorem ipsum ";
    println!("Multi: {}", s1);

    // String Index
    let s1 = String::from("Hello World 𝛂");
    // It takes the byte range and not the index range
    let sb1 = &s1[0..1];
    let sb2 = &s1[12..s1.len()];
    println!("StringIndex: {}, {}", sb1, sb2);

    // As Characters -> Not based on Bytes
    let s1 = String::from("Hello World 𝛂");
    for c in s1.chars() {
        print!("{}[{}] ", c, size_of_val(&c));
    }
    println!();

    // Array [in Stack] -> Fixed Size & Same Type
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    // Complier decides the type
    let arr2: [_; 3] = ['a', 'b', 'c'];
    println!("Arr1 [Length]: {}", arr1.len());
    println!("Arr1 [Size]: {}", size_of_val(&arr1));
    println!("Arr2 [Length]: {}", arr2.len());
    println!("Arr2 [Size]: {}", size_of_val(&arr2));

    // Fill String
    let arr1 = [1; 100]; // [1,1,1,...]

    // Safe with Options
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    // Referencing -> Its not Safe
    let ar1 = &arr1[0];
    // As Option -> Its Safe
    let ar2 = arr1.get(0).unwrap();

    // Slice -> Borrow without Ownership
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let ar1 = &arr1[0..3]; // Slicing

    // Hard Code String
    let s1 = "Hello World !" as &str;
    println!("Hard Code: {}", s1);

    // Capacity
    let mut s1 = String::from("Hello World !");
    s1.clear();
    println!("Capacity: {}", s1.capacity());

    // Tuple [in Stack]
    let t1: ((u8, i16), (char, bool), (&str, String)) = (
        (2, -1),
        ('A', true),
        ("Hello", String::from("World")),
    );
    println!("T1: {:?}", t1);
    println!("T1: {}", t1.1.0);
    let (x, y, z);
    (x, z, y) = (1, 2, 3);

    // Structs
    // Attribute To make Structs Printable
    #[derive(Debug)]
    struct User {
        name: String,
        age: u8,
        hobby: String,
    }
    let age = 18;
    let u1 = User {
        name: String::from("Shahab"),
        age,
        hobby: String::from("Playing"),
    };
    let u2 = User {
        age: 25,
        ..u1
    };
    println!("User2: {:?}", u2);

    // Tuple Structs
    struct Point(i32, i32, i32);
    let p1 = Point(1, 23, 25);
    // Destructure
    let Point(x, _, z) = p1;
    println!("TupleStruct: {},{},{}", x, p1.1, z);

    // Unit Like Structs

    // Debug
    #[derive(Debug)]
    struct Rectangle2 {
        width: i32,
        height: i32,
    }
    let scale = 2;
    let rect1 = Rectangle2 {
        width: dbg!(scale * 30),
        height: 50,
    };
    dbg!(&rect1);
    println!("{:?}", rect1);

    // Ref
    struct Person2 {
        name: String,
        age: u8,
    }
    let p1 = Person2 {
        name: String::from("Hello"),
        age: 19,
    };
    // name borrowed
    // age referenced
    let Person2 { name, ref age } = p1;
    println!("{},{}", name, person.age);

    // Tricks to keep ownership
    let p1 = Person2 {
        name: String::from("Hello"),
        age: 19,
    };
    let age = &p1.age;
    let age = p1.age.clone();

    // Enum
    #[derive(Debug)]
    enum FruitPack {
        Apple,
        Mango,
        Orange,
    }
    enum VegePack {
        Tomato = 3,
        Onion, // 4
        Carrot, // 5
    }
    enum FurniturePack {
        Chair, // 0
        Sofa = 3,
        Shelf = 6,
        Table, // 7
    }
    println!("EnumString: {:?}", FruitPack::Apple);
    println!("EnumNumber: {}", FruitPack::Apple as u8);

    // Enum Nested Structures
    #[derive(Debug)]
    enum Message {
        Quit,
        Move {
            x: i32,
            y: i32,
        },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let e1 = Message::Quit;
    let e2 = Message::Move { x: 3, y: 5 };
    let e3 = Message::Write(String::from("Dumb"));
    let e4 = Message::ChangeColor(4, 2, 9);
    println!("{:?}", e1);
    println!("{:?}", e2);
    println!("{:?}", e3);
    println!("{:?}", e4);

    // If with initializing variables and destructuring
    let e5 = Message::Move { x: 1, y: 4 };
    if let Message::Move { x: a, y: b } = e5 {
        println!("Will run: {},{}", a, b);
    } else {
        println!("Will not run");
    }
    match e2 {
        Message::Move { x: a, y: b } => (),
        Message::ChangeColor(r, g, b) => (),
        _ => (),
    }

    // Option Enum
    let x1: Option<i32> = Some(5); // Or None
    let ans = match x1 {
        None => None,
        Some(item) => Some(item),
    };
    println!("Ans: {:?}", ans.unwrap());

    // Linked List

    // Flow Control
    let a1 = [3, 4, 5, 6, 7];
    for item in a1 {
        println!("A1: {}", item);
    }
    for (key, value) in a1.iter().enumerate() {
        println!("A1: {}:{}", key, value);
    }
    for item in a1.iter() {
        println!("A1: {}", item);
    }

    // Break with Return
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Counter: {}", result);

    // Nested Inner Outer Loop
    let mut count = 0;
    'outer: loop {
        'inner: loop {
            if count >= 20 {
                break 'inner;
            }
            count += 2;
        }
        count += 5;
        'inner: loop {
            if count >= 30 {
                break 'outer;
            }
            continue 'outer;
        }
    }

    // If let
    let a1 = Some(9);
    match a1 {
        Some(item) => println!("Item: {}", item),
        _ => (),
    }
    let a1 = Some(6);
    if let Some(item) = a1 {
        println!("Item: {}", item);
    }

    // Matches
    let a1 = ['a', 'V', 'h', 'y', '7', 'R'];
    for item in a1 {
        assert!(matches!(item, 'A'..='Z' | 'a'..='z' | '0'..='9'));
    }
    println!("Verified");

    enum Gender {
        Male,
        Female,
    }
    let mut count = 0;
    let v1 = vec![Gender::Male, Gender::Female, Gender::Male];
    for e in v1 {
        if matches!(e, Gender::Male) {
            count += 1;
        }
    }
    println!("Males: {}", count);

    // Shadowing
    let age = Some(7);
    if let Some(age) = age {
        println!("Done !");
    }

    // @ operator -> Matches and holds the value too
    struct Point3 {
        x: i32,
        y: i32,
    }
    let p1 = Point3 { x: 4, y: 20 };
    match p1 {
        Point3 { x, y: 0 } => println!("X:{}, Y:{}", x, y),
        Point3 { x: 0..=5, y: y @ (10 | 20 | 30) } => println!("X:{}, Y:{}", x, y),
        Point3 { x: 6..=10, y: yy @ (11 | 21 | 31) } => println!("X:{}, Y:{}", x, y),
        Point3 { x: 6..=10, y: y2 @ 3..=8 } => println!("X:{}, Y:{}", x, y),
        Point3 { x, y } => println!("X:{}, Y:{}", x, y),
    }

    // Match + If
    let n1 = Some(4);
    let n2 = 5;
    match n1 {
        Some(item) if item < n2 => assert!(item < n2),
        Some(item) => assert!(item >= n2),
        None => (),
    }

    // Ignoring values
    let n1 = [1, 2, 3, 4, 5, 6, 7];
    match n1 {
        [first, .., last] => {
            assert_eq!(first, 1);
            assert_eq!(last, 7);
        }
    }

    // Matching Reference
    // v1 still remains after matching ᠅ Caution
    let mut v1 = String::from("Hello");
    let mut v2 = &mut v1;
    match v2 {
        value => value.push_str("world"),
    }

    // Termination
    process::exit(0);

    // Associated Functions -> Fn inside Impl
    // Block connected to struct/enum
    // Methods (.) with structs/enum -> Fn that hold instance
    // Multi-Implementation -> Can use this block multiple times
    // self: Self == self and &mut self == self: &mut Self
    // Self == TrafficLight
    // If param is &self, can still use self
    struct TrafficLight {
        width: i32,
        height: i32,
        color: String,
    }
    impl TrafficLight {
        pub fn new(w: i32, h: i32, c: &str) -> Self {
            Self {
                width: w,
                height: h,
                color: String::from(c),
            }
        }
        pub fn area(self) -> i32 {
            return self.width * self.height;
        }
        pub fn get_color(&self) -> &str {
            return &self.color;
        }
        pub fn show_color(self: &Self) {
            println!("The color is {}", self.color);
        }
        pub fn change_color(&mut self) {
            self.color = "green".to_string();
        }
    }

    // Generics / Const Generics
    struct A;
    struct S(A);
    struct SGen<T>(T);
    fn reg_fn(_s: S) {}
    fn gen_spec_t(_s: SGen<A>) {}
    fn gen_spec_i32(_s: SGen<i32>) {}
    fn generic<T>(_s: SGen<T>) {}
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(7));
    // Char -> Generic [Explicit]
    generic::<char>(SGen('B'));
    // Char -> Generic [Implicit]
    generic(SGen('C'));

    // Add<Output>:
    /// Help add type in Params in Fn
    fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
        return a + b;
    }
    fn prod<T>(a: T, b: T) -> T where T: Mul<T, Output = T> {
        return a * b;
    }
    fn divide<T, U>(a: T, b: U) -> T where T: Div<U, Output = T> {
        return a / b;
    }

    struct Val<T> {
        val: T,
    }
    impl<T> Val<T> {
        pub fn value(&self) -> &T {
            return &self.val;
        }
    }
    let x1 = Val { val: 3 };
    let x2 = Val { val: "Hello" };
    x1.value();

    struct Pont<T, U> {
        x: T,
        y: U,
    }
    impl<T, U> Pont<T, U> {
        pub fn mix<V, W>(self, other: Pont<V, W>) -> Pont<T, W> {
            Pont {
                x: self.x,
                y: other.y,
            }
        }
    }
    // This implementation works only
    // for mentioned data types
    impl Pont<f64, f64> {
        fn distance(&self) -> f64 {
            return (self.x.powi(2) + self.y.powi(2)).sqrt();
        }
    }

    // const Generics
    // Means Arr<i32,3> != Arr<i32,4>
    struct Array<T, const N: usize> {
        data: [T; N],
    }
    let arr1: [Array<i32, 3>; 3] = [
        Array { data: [1, 2, 3] },
        Array { data: [1, 2, 3] },
        Array { data: [1, 2, 3] },
    ];

    // Traits
    trait Animal {
        // Optional to Override
        fn say_hi(&self) -> String {
            String::from("Hi !!!")
        }
        fn sound(&self) -> String;
    }
    struct Sheep;
    struct Cow;
    impl Animal for Sheep {
        fn sound(&self) -> String {
            String::from("Maa!")
        }
    }
    impl Animal for Cow {
        fn sound(&self) -> String {
            String::from("Moo!")
        }
    }

    // Derivable Traits -> Debug, Clone, Copy, PartialEq
    // Traits as Parameters with &impl reference
    // Where Clauses and Generics for Multi Trait Bounds
    // Return Types that Implement Traits

    // Attribute -> Helps Compare
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);
    // Attribute -> Helps Print
    #[derive(Debug)]
    struct Inches(i32);
    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;
            Centimeters((inches as f64) * 2.54)
        }
    }
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Seconds(i32);
    let s1 = Seconds(1);
    let s2 = Seconds(1);
    println!("{:?}", s1);
    let c1 = s1 == s2;
    let m1 = Centimeters(100.0);
    let f1 = Inches(12);
    let c2 = f1.to_centimeters() > m1;
}
