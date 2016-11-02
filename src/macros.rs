
fn test_normal() {
    macro_rules! o_o {
        (
            $(
                $x: expr => [$( $y: expr), *]
             );*
        ) => {
            &[ $($( $x + $y), *), *]
        }
    }

    let a: &[i32]
        = o_o!(10 => [2,3,4,5,6,7];
               20 => [7,6,5,4,3  ]); 

    assert_eq!(a, [12,13,14,15,16,17,27,26,25,24,23]);

}

fn test_hygiene() {
    // hygiene
    fn get_log_state() -> i32 { 3 }

    macro_rules! log {
        ($msg:expr) => {{
            let state: i32 = get_log_state();
            if state > 0 {
                println!("log({}): {}", state, $msg);
            }
        }};
    }

    macro_rules! foo {
        () => (fn x() {})
    }

    macro_rules! foo2 {
        () => ({
            fn x() { println!("x() in marcro") }

            x();
        })
    }

    macro_rules! foo_ident{
        ($x: ident) => {
            fn $x() { }
        }
    }


    let state: &str = "reticulating splines";
    log!(state);

    // fn x() { println!("original x()")};
    // 宏中的方法并不卫生, 只对 let/loop 起作用
    foo!();
    x();

    {
        fn x() { println!("x() outbound")};
        foo2!();
        x();// expect x() outbound
    }

    {
        foo_ident!(another);
        another(); // ident, 防止被涂色
    }

}

fn test_recursion() {

    // 例子爲html
    #![allow(unused_must_use)]
    macro_rules! write_html{
        ($w: expr, ) => (());

        ($w: expr, $e: tt) => (write!($w, "{}", $e));

        ($w: expr, $tag: ident [ $($inner: tt)* ] $($rest:tt)*) => {{
            write!($w, "<{}>", stringify!($tag));
            write_html!($w, $($inner)*);
            write!($w, "</{}>", stringify!($tag));
            write_html!($w, $($rest)*);
        }}
    }


    use std::fmt::Write;
    let mut out = String::new();

    write_html!(&mut out,
        html[
            head[title["test"]]
            body[h1["body"]]
        ]);
    assert_eq!(out, 
        "<html><head><title>test</title></head>\
        <body><h1>body</h1></body></html>");


}

fn do_before() { print!("===> "); }
fn do_after() { println!(" <==="); }

fn test_statement() {

    macro_rules! m_stmt {
        ($($x: stmt),*) => {
            do_before();
            $($x);*;
            do_after()
        }
    }

    //m_stmt!(=> {return;});
    //m_stmt!(let haha = 3 => { print!("{}", haha); });
    m_stmt!(let haha = 3, let _ = 33);
    assert_eq!(haha, 3);
}

fn test_complex() {
    ///
    /// Bitwise Cyclic Tag
    ///
    /// bct!(p,p,p,p,p;d,d,d)
    ///
    /// # Example
    /// ```
    /// assert_eq!(bct!(0, 0, 1, 1, 1; 1, 0), ());
    /// ```
    ///
    macro_rules! bct {
        //
        // cmd 0: d ... => ...(remove leftmost)
        //
        // 0.1 only one bit, remove, and goto halt
        (0, $($ps:tt),* ; $_d: tt)
            => (bct!($($ps),*, 0 ; ));
        // 0.2 remove leftmost($_d)
        (0, $($ps:tt),* ; $_d: tt, $($ds: tt),*)
            => (bct!($($ps),*, 0 ; $($ds),*));

        //
        // cmd 1p: 1... => 1...p
        //
        // 1.1 if data only 1, so result is 1p
        (1, $p: tt, $($ps:tt),* ; 1)
            => (bct!($($ps),*, 1, $p ; 1, $p));
        // 1.2 if data is 1..., so result is 1...p
        (1, $p: tt, $($ps:tt),* ; 1, $($ds: tt),*)
            => (bct!($($ps),*, 1, $p ; 1, $($ds),*, $p));
        // 1.3 if data is 0..., so result is 0...
        (1, $p: tt, $($ps:tt),* ; $($ds:tt),*)
            => (bct!($($ps),*, 1, $p ; $($ds),*));

        //
        // halt if data is empty
        //
        ($($ps:tt),* ; )
            => (());

    }

    // example in http://esolangs.org/wiki/Bitwise_Cyclic_Tag
    bct!(0,0,1,1,1;1,0);
    bct!(1,1,0,1,0,0;1,0);
}

pub fn test() {
    unimplemented!();
}

/// This is the main access to macro tests.
///
/// but What scope the example is?
///
/// # Example
/// ```
/// assert!(true);
///
/// ```
pub fn main() {

    test_normal();

    test_hygiene();

    test_recursion();

    test_statement();

    test_complex();
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        super::main();
    }
}
