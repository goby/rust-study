extern crate rand;

extern crate guessing;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

#[allow(dead_code)]
fn guessing_number() {

    println!("Guess the number!");

    let sec_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Plesae input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // make guess to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

            //.expect("Invalid number");

        println!("Your guessed: {}", guess);

        match guess.cmp(&sec_number) {
            Ordering::Less      => println!("Too small"),
            Ordering::Greater   => println!("Too large"),
            Ordering::Equal     => {
                println!("You got it!");
                break;
            }
        }
    }
}

fn abs(x: i32) -> i32 {
    if x >= 0 { x } else { -x }
}

fn method_type() {
    fn incr(x: i32) -> i32 {
        x + 1
    }

    let f = incr;
    println!("5 + 1 = {}", f(5));
}

fn loop_learn() {

    let lines = "hello\nworld".lines();
    for (ln, line) in lines.enumerate() {
        println!("{}: {}", ln, line);
    }
}

fn loop_label() {

    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }

            println!("X:{}, y:{}", x, y);
        
        }
    }

}


fn utf8() {
    let (x,y,mut z) = (1,2,3);
    let xx = '中';
    z += 1;
    println!("{}, {}, {}, {}", x, y, z, xx);
    println!("{}, {}, {}", abs(10),abs(-10), abs(0));
}

fn if_let() {
    let option = Some(5);
    fn foo(x: i32) { println!("boooom.. {}", x) }
    fn bar() {}

    if let Some(x) = option{
        foo(x);
    } else {
        bar();
    }

    let mut v = vec![1,2,3,4,5];
    while let Some(x) = v.pop() {
        print!("{}, ", x);
    }
    print!("\n");
}

fn transmute() {

    let a = [8u8, 7u8, 6u8, 5u8];

    // 32      24      16      8       0
    // +-------+-------+-------+-------+
    // |  0x5  |  0x6  |  0x7  |  0x8  |
    // +-------+-------+-------+-------+
    // |  Point.y      |   Point.x     |
    // +-------+-------+-------+-------+
    //
    struct Point {
        x: u16,
        y: u16
    }

    unsafe {
        let b = std::mem::transmute::<[u8; 4], u32>(a);
        println!("{}", b);
        assert_eq!(0x05060708, b);

        let p = std::mem::transmute::<[u8; 4], Point>(a);
        assert_eq!(0x0506u16, p.y);
        assert_eq!(0x0708u16, p.x);
    }
}

fn test_variant() {
    trait Print {
        fn print(&self);
    }

    impl Print for str {
        fn print(&self) {
            println!("{}", self);
        }
    }

    impl<T> Print for [T] where T: std::fmt::Display {
        fn print(&self) {
            print!("[");
            if self.len() > 0 {
                for i in 0..(self.len() - 1) {
                    print!("{}, ", self[i]);
                }
                print!("{}", self[self.len() - 1]);
            }
            println!("]");
        }
    }

    "123".print();
    ["123"].print();
    ["123", "999", "123"].print();

    // unlimit size
    // ?Sized:  maybe Sized
    #[allow(dead_code)]
    struct Foo<T: ?Sized> {
        t: T,
    }
}

fn main() {

    utf8();

    method_type();

    loop_learn();

    loop_label();

    if_let();
    transmute();
    test_variant();

    // lifetimes::main();
    // structs::main();
    // vectors::main();
    // strings::main();
    // traits::main();
    // closures::main();

}
