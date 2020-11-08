extern crate hello_lib3;

use hello_lib3;

#[test]
fn test_greet() {
    let hello = greeter::Greeter::new("Hello");
    assert_eq!("Hello world", hello.greet("world"));
}
