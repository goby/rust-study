extern crate guessing;

use guessing::macros;

#[test]
fn it_works() {
    macros::main();
    
    assert_eq!(1,1);
}
