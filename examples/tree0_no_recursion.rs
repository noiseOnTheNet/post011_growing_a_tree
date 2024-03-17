use trees:*;

fn main() {
    let test0 : BTree0=BTree0 { left: None, right: None };
    let test1 = add_test(false);
    let test : BTree0 = BTree0 { left: Some(& test0), right: Some(& test1) };
    println!("Hello, world! {:?}", test);
}

fn add_test<'a>(stop : bool) -> BTree0<'a>{
    let value = BTree0 {left: (if stop {None} else {let branch = add_test(true); Some (& branch)})
                               ,right: None};
    return value;
}
