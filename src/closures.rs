
#[cfg(test)]
mod tests {

    // REFERENCE is needed becourse:
    //   Fn is a trait without size info,
    //   &Fn is a reference to real target with size
    //   Finally, the lifetime 'static should be provided
    //fn high_level_function(num: i32) -> &'static (Fn(i32) -> i32) {
    pub fn test_return_closure(num: i32) -> Box<Fn(i32) -> i32> {
        // failed with missmatch type, Fn <===>  closure
        //move |x| x + num

        // wtf!!!!!!!!!!
        Box::new(move |x| x + num)
    }

    #[test]
    fn test_closure_impl() {
        // what's the fuck!!!!!!!!!!!!
        trait Fn<Args>: FnMut<Args> {
            extern "rust-call" fn call(&self, args: Args) -> Self::Output;
        }

        trait FnMut<Args>: FnOnce<Args> {
            extern "rust-call" fn call(&mut self, args: Args) -> Self::Output;
        }

        trait FnOnce<Args> {
            type Output; // associate type, ===> Self:Output

            extern "rust-call" fn call(self, args: Args) -> Self::Output;
        }
    }

    #[test]
    fn test_closure_as_arg() {
        //static
        fn call_with_one<F>(closure: F) -> i32
            where F: Fn(i32) -> i32 {

            closure(1)
        }

        let answer = call_with_one(|x| x + 2);
        assert_eq!(3, answer);

        // dynamic
        fn call_with_two(closure: &Fn(i32) -> i32) -> i32 {
            closure(2)
        }

        // use &|| instead of ||
        let answer = call_with_two(&|x| x + 2);
        assert_eq!(4, answer);
    }

    #[test]
    fn test_move() {
        {
            let mut num = 5;
            {
                // take the ownership of num
                let mut add_num = |x: i32| num += x;
                add_num(5);
            }
            assert_eq!(10, num);
        }
        {
            let mut num = 5;
            {
                // take a copy of num
                let mut add_num = move |x: i32| num += x;
                add_num(5);
            }
            assert_eq!(5, num);
        }
    }

    #[test]
    fn test_lambda() {
        let plus_one = |x: i32| x+1;
        assert_eq!(5, plus_one(4));

        let plus_one2 = |x: i32| -> i32 { x + 1};
        assert_eq!(5, plus_one2(4));

        let plus_two = |x| {
            let mut result: i32 = x;

            result += 1;
            result += 1;

            result
        };
        assert_eq!(6, plus_two(4));
    }

    #[test]
    pub fn main() {
        assert_eq!(test_return_closure(5)(6), 11);
    }
}
