#[derive(Debug)]
pub struct BTree0<'a> {pub left : Option<&'a BTree0<'a>>, pub right : Option<&'a BTree0<'a>>}

#[derive(Debug)]
pub struct BTree1 {pub left : Option<Box<BTree1>>, pub right : Option<Box<BTree1>>}
