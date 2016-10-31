
trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 { *self as i64 }
}

fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
    x.convert()
}

fn inverse<T>() -> T
        where i32: ConvertTo<T> {
    42.convert()    
}

// default methods
trait Validate {
    fn is_valid(&self) -> bool;
    fn is_invalid(&self) -> bool { 
        println!("Default::is_inValid");
        !self.is_valid() 
    }
}

struct UseDefault;
struct OverrideDefault;

impl Validate for UseDefault {
    fn is_valid(&self) -> bool { true }
}

impl Validate for OverrideDefault {
    fn is_valid(&self) -> bool { true }

    fn is_invalid(&self) -> bool {
        println!("OverrideDefault::is_invalid");
        self.is_valid()
    }
}


// Drop like .cctor in Cpp?
struct HasDrop {
    id: u32,
}

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("[{}]Dropping!!!", self.id)
    }
}

// static & dynamic dispatch
trait FormatString {
    fn format_string(&self) -> String; 
}

impl FormatString for u8 {
    fn format_string(&self) -> String { format!("u8: {}", *self) }
}

impl FormatString for String {
    fn format_string(&self) -> String { format!("String: {}", *self) }
}

// multi copy after compile, code bloat
fn static_dispatch<T: FormatString>(t: &T) {
    println!("{}", t.format_string())
}

// only one copy after compiling, type erase
fn dynamic_dispatch(t: &FormatString) {
    println!("{}", t.format_string())
}

fn object_safe() {
    let v = vec![2, 0, 5, 9, 9, 5, 0];
    // failed for Clone is not obect safe.
    // let _ = &v as &Clone;
    let _ = &v;
}

// with same function
fn trait_with_same_function() {
    trait Foo {
        fn f(&self);
    }

    trait Bar {
        fn f(&self);
    }

    struct Baz;

    impl Foo for Baz {
        fn f(&self) { println!("Impl Foo for Baz")}
    }

    impl Bar for Baz {
        fn f(&self) { println!("Impl Bar for Baz")}
    }

    let b = Baz;

    // prototype of f need a &self, pass &b
    Foo::f(&b);
    Bar::f(&b);
}

fn trait_with_as_form() {
    // <>::
    trait Foo { fn f(&self); }
    struct Bar;
    impl Bar { fn f(&self) { println!("bar.f(): Impl without trait"); }}
    impl Foo for Bar { fn f(&self) { println!("<Bar as Foo>::f(&): Impl Foo for Bar"); }}

    let b = Bar;
    b.f();
    <Bar as Foo>::f(&b);
}

#[test]
pub fn main() {
    println!(">>>>>> TRAITS >>>>>>");

    {
        println!("Expect 0 drop now: ");
        let _  = HasDrop { id: 0};
        let _1 = HasDrop { id: 1};
        let _2 = HasDrop { id: 2};
        println!("i32.convert()   = {:?}", 6429455.convert()); 
        println!("normal(6429455) = {:?}", normal(&6429455));
        println!("inverse()       = {:?}", inverse::<i64>());
        println!("Expect drop now: ");
    }

    let x = UseDefault;
    let y = OverrideDefault;
    assert!(!x.is_invalid());
    assert!(y.is_invalid());

    let (x, y) = (5u8, "Hello".to_string());

    static_dispatch(&x);
    static_dispatch(&y);

    dynamic_dispatch(&x as &FormatString);
    dynamic_dispatch(&y as &FormatString);

    // concercing, heavy
    dynamic_dispatch(&y);

    self::object_safe();
    self::trait_with_same_function();
    self::trait_with_as_form();
}
