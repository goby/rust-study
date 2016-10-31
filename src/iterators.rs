
use std::iter;

trait HasFold<T> {
    fn fold2<'a, F>(&self, base: &'a T, closure: F) -> &'a T
        where F: Fn(&'a T, &T) -> &'a T;
}

impl<T> HasFold<T> for [T] where T: iter::Iterator {
    fn fold2<'a, F>(&self, base: &'a T, closure: F) -> &'a T 
        where F: Fn(&'a T, & T) -> &'a T {

        let mut result = Box::new(base);
        for i in self.iter() {
            *result = closure(&*result, i);
        }

        *result
    }
}

pub fn main() {
    use self::HasFold;

    let sum2 = [1,2,3,4,5,6].iter().fold(10, |s, x| s+x);
    assert_eq!(sum2, 31);

}


#[cfg(test)]
mod tests {
    use super::*;
    use vectors;

    #[test]
    fn test_collect() {
        let one_to_one_hundred = (1..101).collect::<Vec<_>>();
        vectors::print_vec(&one_to_one_hundred);
    }

    #[test]
    fn test_find() {
        let greater_than_42 = (0..100).find(|x| *x > 42);
        match greater_than_42 {
            Some(x) => assert!(x > 42),
            None => panic!("should not be here"),
        }

        let greater_than_101 = (0..100).find(|x| *x > 101);
        match greater_than_101 {
            Some(x) => panic!("should not be here"),
            None => {} ,
        }

    }

    #[test]
    fn test_fold() {
        let sum = (0..100).fold(0, |s, x| s+x);
        assert_eq!(sum, 4950);

        main();
    }
    
}
