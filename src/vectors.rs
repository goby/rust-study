
use std::fmt::{Display,Debug};

//fn print_vec<T>(v: &Vec<T>) where T: Display + Debug {
pub fn print_vec<T: Display + Debug>(v: &Vec<T>) { // trait bound
    for i in v {
        print!("{:?},", i);
    }
    print!("\n");
}

#[test]
pub fn main() {

    let v = vec![1,3,3,4,6];
    print_vec(&v);

    let v0 = vec![0; 10];
    print_vec(&v0);

    let i: usize = 3; // i32 will not work, type of index is usize
    println!("v[3] = {}", v[i]);

    let vs = vec!["这", "以", "天"];
    print_vec(&vs);

    // out of bound
    match v.get(100) {
        Some(x) => println!("Item 100 is {}", x),
        None => println!("Item 100 is out of bound")
    }

    // iteration
    for i in &v {
        println!("A reference to {}", i);
    }

    let mut mv = vec![1,2,3,4,5];

    for i in &mut mv {
        println!("A mutable reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }
}
