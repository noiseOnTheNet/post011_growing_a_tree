use trees::*;

fn main() {
    let test0 : BTree0=BTree0 { left: None, right: None };
    let test : BTree0 = BTree0 { left: Some(& test0), right: None };
    println!("Hello, world! {:?}", test);
}
