
fn test_move() {

    fn take(v: Vec<i32>) -> Vec<i32> {
        v 
    }

    let v = vec![1,2,3];
    let v2 = take(v);
    assert_eq!(1, v2[0]);
    //println!("v[0] is {}", v[0]); // v was moved to v2
}

fn test_ownership() {

    let mut x = 5;
    {
        let r1 = &x;
        let r2 = &x;
        let r3 = &x;
        assert_eq!(x, *r1);
        assert_eq!(x, *r2);
        assert_eq!(x, *r3);
    }
    {
        let w = &mut x;
        *w += 1;
    }
    let r1 = &x;
    let r2 = &x;
    assert_eq!(6, *r1);
    assert_eq!(6, *r2);
}

fn test_lifetime() {
    struct Foo<'a> {
        x: &'a i32,
    }

    impl<'a> Foo<'a> {
        fn x(&self) -> &'a i32 {self.x}
    }

    let y = &5;
    let f = Foo{x: y};

    assert_eq!(f.x, f.x());

    let x;
    {
        let z = Foo{x: y};
        x = z.x;
    }

    assert_eq!(x, y);

    let x :&'static str = "StaticString";
    println!("x={}",x);
}

fn test_elided_lifetime() {

    fn print_(s: &str) { print!("{}", s) }  // elided
    fn print<'a>(s: &'a str) { print_(s) }  // expanded

    fn debug_(_: u32, s: &str) { print(s) }             // elided
    fn debug<'a>(i: u32, s: &'a str) { debug_(i, s) }   // expanded

    fn substr_(s: &str, until: usize) -> &str  {
        debug(until as u32, s);
        &s[0..until]
    }   // elided
    fn substr<'a>(s: &'a str, until: usize) -> &'a str { substr_(s, until) } // expanded

    // fn get_str() -> &str { "" } ERROR
    fn get_str<'a>() -> &'a str { substr("hello", 0) }  // no inputs, need lifetime

    // fn combine_str(s1: &str, s2: &str) -> &str { s1 + s2 }
    fn combine_str<'a>(_: &'a str, _: &str) -> &'a str { get_str() }

    fn combine_str_<'a, 'b>(l: &'a str, r: &'b str) -> &'a str {
        combine_str(l, r)
    }

    combine_str_("hello", "world");
}

#[test]
pub fn main() {
    println!(">>>>>> LIFETIME >>>>>>");

    test_ownership();
    test_move();
    test_lifetime();
    test_elided_lifetime();
}
