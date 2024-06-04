#![allow(unused)]

/*

VARIABLES:
- Const
- Variable
    - Mutable
    - Immutable

String: 
    - String new Object
    - String Reference &str

Concatenation:
    - {} for Strings
    - {:?} for Objects

DATA-TYPES:
- Boolean
- Integer
- Float
- String
- Character
+ (Address) => Pointer
+ (Bytes)

DATA-STRUCTURES:
- enum
- Tuple
- Array/List
- Vector
- Heap/Stack
- HashMaps
- Maps
- Structs
- Traits
- Implementations
- Box
- Some/None OK/Err / error.kind()

+ Match / If
+ Loops (loop/while/for/map)
+ Functions
+ Generics
+ Files/Buffers
+ Error/Exception Handling
+ Closures
+ Modules/Nested (super/self)
+ Pointers/References/Smart Pointers
+ Threads/Multi-Threading/Sleep
+ Thread Locks / Move

 */

use std::char::MAX;
use std::{ error, io };
use rand::{ Error, Rng };
use std::io::{ Write, BufReader, BufRead, ErrorKind };
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use colored::Colorize;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{ Arc, Mutex };

mod restaurant;
use crate::restaurant::order_food;

fn test1() {
    /*
        Unsigned Integer: u8, u16, u32, u64, u128, usize
        Signed Integer: i8, i16, i32, i64, i128, isize
     */
    println!("Max u32: {}", u32::MAX);

    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let is_male = true;
    let my_grade = 'A';
    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Days = Days::Monday;
    match today {
        Days::Monday => println!("I hate Monday"),
        _ => println!("No Day!"),
    }
    println!("Weekend: {}", today.is_weekend());

    let num1: f32 = 1.1111111111111;
    println!("f32: {}", num1 + 0.1111111111111);

    let rand_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("Rand: {}", rand_num);

    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

    let can_vote: bool = if age >= 18 { true } else { false };
    println!("Can Vote: {}", can_vote);

    match age {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=u32::MAX => println!("Important Birthday"),
        _ => println!("Not Important Birthday"),
    }

    let voting_age: u32 = 18;
    match age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote!"),
    }

    let arr_1 = [1, 2, 3, 4];
    println!("Arr Length: {}", arr_1.len());

    let mut iter: usize = 0;
    loop {
        if arr_1[iter] % 2 == 0 {
            iter += 1;
            continue;
        }
        if iter >= arr_1.len() {
            break;
        }
        println!("Arr[{}]: {}", iter, arr_1[iter]);
        iter += 1;
    }

    let mut iter2: usize = 0;
    while iter2 < arr_1.len() {
        println!("Arr[{}]: {}", iter2, arr_1[iter2]);
        iter2 += 1;
    }

    for val in arr_1.iter() {
        println!("Arr[]: {}", val);
    }

    let my_tuple = (47, "Zaeem".to_string(), 50_000.0);
    println!("Name: {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age: {}", v1);

    let mut st1 = String::new();
    st1.push('A');
    st1.push_str("word");
    for word in st1.split_whitespace() {
        print!("{}", word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);
    let st3 = String::from("x a f j t d s e t j y e u");
    let mut v_1: Vec<char> = st3.chars().collect();
    v_1.sort();
    v_1.dedup();
    for ch in v_1 {
        println!("{}", ch);
    }

    let st4 = "Hello World !";
    let mut st5 = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("{}", st6.len());
    st5.clear();
    let st6 = String::from("Just One");
    let st7 = String::from("Just One 2");
    let st8 = st6 + &st7;
    for ch in st8.bytes() {
        println!("{}", ch);
    }

    let n1: u8 = 5;
    let n2: u8 = 4;
    let result: u32 = (n1 as u32) + (n2 as u32);

    let mut name = String::new();
    let greeting = "Nice to meet you";
    println!("\nWhat is your name ? ");
    io::stdin().read_line(&mut name).expect("Didn't receive Input");
    println!("\n Hello {}! {}", name.trim_end(), greeting);

    // Vectors
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("1st: {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("Second: {}", second),
        None => println!("Nothing"),
    }
    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vector Length: {}", vec2.len());
    println!("Pop: {:?}", vec2.pop())
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn get_next_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn list_sum(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    return sum;
}

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn print_str(x: String) {
    println!("A string: {}", x);
}

fn print_str2(x: &str) {
    println!("A string: {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string: {}", x);
    return x;
}

fn change_str(x: &mut String) {
    x.push_str("Extended Text");
    println!("Changed String: {}", x);
}

fn test2() {
    let str1 = String::from("World");
    let str2 = str1.clone();
    println!("Hello {}", str1);

    print_str(String::from("Hello World"));
    print_str2("Hello World");
    let mut str3 = print_return_str(String::from("Hello There"));
    change_str(&mut str3);

    // HashMaps
    let mut heroes: HashMap<&str, &str> = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Ironman", "Tony Stark");
    heroes.insert("Spiderman", "Peter Parker");
    for (k, v) in heroes.iter() {
        println!("{}: {}", k, v);
    }
    if heroes.contains_key(&"Ironman") {
        let ironman = heroes.get(&"Ironman");
        match ironman {
            Some(x) => println!("I am Iron Man !"),
            None => println!("No Iron Man !"),
        }
    }
}

fn test3() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob = Customer {
        name: String::from("Bob"),
        address: String::from("123 Street"),
        balance: 434.878,
    };
    bob.address = String::from("345 Street");

    struct Rectangle<T, U> {
        length: T,
        height: U,
    }
    let rec = Rectangle {
        length: 5,
        height: 10.8,
    };

    const PI: f32 = 3.141592;
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Square {
        length: f32,
        width: f32,
    }
    struct Circle {
        length: f32,
        width: f32,
    }
    impl Shape for Square {
        fn new(length: f32, width: f32) -> Square {
            return Square { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }
    let sq1: Square = Shape::new(10.6, 28.9);
    let cr1: Circle = Shape::new(6.7, 8.12);
    println!("\nSquare Area: {}", sq1.area());
    println!("Circle Area: {}", cr1.area());
}

fn test4() {
    let result = add(4, 5);
    println!("Result: {}", result);

    let (res1, res2) = get_next_2(7);
    println!("\nResult2: {} , {}", res1, res2);

    let num_list: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let sum = list_sum(&num_list);
    println!("\nList Sum: {}", sum);

    println!("\nINT + INT: {}", get_sum_gen(5, 6));
    println!("\nFLOAT + FLOAT: {}", get_sum_gen(5.2, 6.7));

    // test3();
    order_food();
}

fn test5() {
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        // Err(error) => { panic!("Problem creating File: {:?}", error) }
        Err(error) =>
            match error.kind() {
                ErrorKind::NotFound =>
                    match File::create(path) {
                        Ok(fc) => fc,
                        Err(e) => { panic!("Problem creating File: {:?}", e) }
                    }
                _other_error => { panic!("Problem creating File: {:?}", error) }
            }
    };
    write!(output, "Just Some\nRandom Words !").expect("Failed to Write to file");
    let input = File::open(path).unwrap();
    // Read every line separately
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }
}

fn test6() {
    let mut arr_it = [1, 2, 3, 4, 5];
    for val in arr_it.iter() {
        println!("Value: {}", val);
    }
    arr_it.into_iter(); // Consume but can't use it anymore
    let mut iter1 = arr_it.iter();
    println!("Val: {:?}", iter1.next());
    println!("Val: {:?}", iter1.next());
    println!("Val: {:?}", iter1.next());
    println!("Val: {:?}", iter1.next());
    println!("Val: {:?}", iter1.next());
    println!("Val: {:?}", iter1.next());
}

fn test7() {
    let can_vote = |age: i32| { age >= 18 };
    // Closures can access variables outside their body
    let mut num1 = 5;
    let print_var = || { println!("Var: {}", num1) };
    print_var();
    num1 = 7;
    let mut change_var = || {
        num1 += 1;
    };
    change_var();
    println!("Var: {}", num1);
    num1 = 9;
    println!("Var: {}", num1);

    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
        return func(a, b);
    }
    let sum = |a: i32, b: i32| a + b;
    let prod = |a: i32, b: i32| a * b;
    println!("Sum: {}", use_func(5, 6, sum));
    println!("Prod: {}", use_func(5, 6, prod));
}

fn test8() {
    let b1 = Box::new(10);
    println!("{}", b1);

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            return TreeNode { left: None, right: None, key };
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            return self;
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            return self;
        }
    }

    let node1 = TreeNode::new(10);
    let node2 = TreeNode::new(12);
    let node3 = TreeNode::new(13);
    node1.left(node2).right(node3);
}

fn test9() {
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned Thread: {}", i.to_string().bold().red());
            thread::sleep(Duration::from_millis(1));
        }
    });
    let thread2 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned Thread: {}", i.to_string().bold().purple());
            thread::sleep(Duration::from_millis(1));
        }
    });
    let thread3 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned Thread: {}", i.to_string().bold().yellow());
            thread::sleep(Duration::from_millis(1));
        }
    });
    let thread4 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned Thread: {}", i.to_string().bold().magenta());
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread1.join().unwrap();
    thread2.join().unwrap();

    for i in 1..20 {
        println!("Main Thread: {}", i.to_string().bold().green());
        thread::sleep(Duration::from_millis(1));
    }

    thread3.join().unwrap();
    thread4.join().unwrap();
}

fn bank_example() {
    pub struct Bank {
        balance: f32,
    }
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.0 {
            println!("Balance Amount ({}), is not Enough !", bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Amount Withdrawn: {}, Balance Left: {}", amt, bank_ref.balance);
        }
    }
    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.0);
    }
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.0 }));

    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        return thread::spawn(|| {
            return customer(bank_ref);
        });
    });
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total: {}", bank.lock().unwrap().balance);

    // fn withdraw(the_bank: &mut Bank, amt: f32) {
    //     the_bank.balance -= amt;
    // }
    // let mut bank = Bank { balance: 100.0 };
    // withdraw(&mut bank, 5.0);
    // println!("Balance: {}", bank.balance);
    // fn customer(the_bank: &mut Bank) {
    //     withdraw(the_bank, 5.0)
    // }
    // thread
    //     ::spawn(|| {
    //         customer(&mut bank);
    //     })
    //     .join()
    //     .unwrap();
}

fn main() {
    println!("\nWelcome to Rust !!!\n");

    // Code
    bank_example();

    println!("");
}
