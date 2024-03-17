use trees::*;

fn main() {
    let test0 : BTree1 = BTree1 { left: None, right: None };
    let test1 = add_test(false);
    let test : BTree1 = BTree1 { left: Some(Box::new(test0)), right: Some(Box::new(test1)) };
    println!("Hello, world! {:?}", test);
}

fn add_test<'a>(stop : bool) -> BTree1{
    let value = BTree1 {left: (if stop {None} else {let branch = add_test(true); Some (Box::new(branch))})
                               ,right: None};
    return value;
}
