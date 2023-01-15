pub enum List {
    Cons(u32, Box<List>),
    Nil,
}

use List::*;

impl List {
    pub fn new() -> List {
        List::Nil
    }

    pub fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    pub fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    // 返回列表的字符串表示（该字符串是堆分配的）
    pub fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，
                // 而不是打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

