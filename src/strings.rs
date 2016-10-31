
#[test]
pub fn main() {
    let multi_line_string = "你好,
    中国人!"; // &'static str

    assert_eq!("你好,\n    中国人!", multi_line_string);

    let ignore_space = "foo\
        bar";
    assert_eq!("foobar", ignore_space);

    // String and str
    // String, a struct allocated in heap
    let mut s = "闽南".to_string();
    println!("to_string() = {}", s);
    s.push_str("人");
    println!("push_str()  = {}", s);
    // &String  convert to str
    fn takes_slice(slice: &str) {
        println!("[&convert String to str]slice = {}", slice);
    }

    takes_slice(&s);

    {
        // iterate str
        let ss:&str = &s;
        for b in ss.as_bytes() {
            print!("{}, ", b);
        }
        print!("\n");

        for c in ss.chars() {
            print!("{}, ", c);
        }
        print!("\n");
    }

    // slicing
    let dog = "hachiko";
    let dog_part = &dog[2..4];

    println!("dog = {}, dog_part = {}", dog, dog_part);
    //ss[2..4] will failed for no-ascii code
    
    // concatenation
    let str_str = dog.to_string() + dog;
    let string_string = s + ", is humans in the South of Fujian";
    println!("dog.to_string() + ss = {}\n s + &s = {}", str_str, string_string);
}
