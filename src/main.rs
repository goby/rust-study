extern crate rand;


use std::io;
use std::cmp::Ordering;
use rand::Rng;

#[warn(dead_code)]
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

fn move_() {

    fn take(v: Vec<i32>) -> Vec<i32> {
        v 
    }

    let v = vec![1,2,3];
    let v2 = take(v);
    println!("v[0] is {}", v2[0]);
}

fn ownership() {

    let mut x = 5;
    {
        let r1 = &x;
        let r2 = &x;
        let r3 = &x;
        println!("readonly: {}, {}, {}", r1, r2, r3);
    }
    {
        let w = &mut x;
        *w += 1;
    }
    let r1 = &x;
    let r2 = &x;
    let r3 = &x;
    println!("after wr: {}, {}, {}", r1, r2, r3);
}

fn lifetime() {
    struct Foo<'a> {
        x: &'a i32,
    }

    let y = &5;
    let f = Foo{x: y};

    println!("{}", f.x);
}


fn main() {

    //embed::process();
    let (x,y,mut z) = (1,2,3);
    let xx = '中';
    z += 1;
    println!("{}, {}, {}, {}", x, y, z, xx);
    println!("{}, {}, {}", abs(10),abs(-10), abs(0));

    method_type();

    loop_learn();

    loop_label();

    move_();

    ownership();

    lifetime();
}
