use std::cmp::Ordering;
use std::io;
use std::collections::HashMap;

use rand::Rng;

pub fn temp() {
    print!("Guess:");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        break;
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Nope");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("Nope"),
            Ordering::Greater => print!("Nope"),
            Ordering::Equal => {
                print!("Yup");
                break;
            }
        }
    }


    let tup = (500, 6.4, 1);
    println!("{}", tup.1);

    let a = [10; 2];

    println!("Len: {}", a.len());

    let y = {
        let x = 3;
        x + 1
    };

    println!("y: {y}");

    println!("Stuff:");
    println!("{}", stuff(10));

    let arr = [10, 20, 30, 40, 50];

    for item in arr {
        println!("the value is: {item}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    //print!("{}", s);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str("x");
        println!("r1: {r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    r2.push_str("yz");
    println!("r2: {r2}");


    // Slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    //&str string slice
    //&[i32] array slice


    // Structs
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("changed"); // `mut`

    let user2 = User {
        email: String::from("diff"),
        ..user1 // Use vals from user1
    };

    println!("{:?}", user2);
    println!("{:#?}", user2);
    println!("{}", user2.username_len());

    // enum/structs
    let m = Message::Write(String::from("hello"));
    m.call(); // impl on enum

    // if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // collections
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 50; // * == dereference operator
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn stuff(x: i32) -> i32 {
    x + 5
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn username_len(&self) -> usize {
        self.username.len()
    }

    fn some_constructor_no_self(other: User) -> Self {
        Self {
            ..other
        }
    }
}

// Tuple structs
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

enum MyOption<T> {
    None,
    Some(T),
}
