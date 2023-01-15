use crate::{
    enum_type::{inspect, WebEvent},
    linked_list::List,
};

mod enum_type;
mod linked_list;

fn main() {
    let point: Point = Point { x: 10.3, y: 0.4 };
    // 结构体更新语法
    let new_point = Point { x: 3.0, ..point };
    println!("{:?}", new_point);

    // 结构体解构
    let Point { x: left, y: right } = new_point;
    println!("left: {}, right: {}", left, right);

    let pair = Pair(3, 4);
    println!("pair: {:?}", pair);

    // 元组解构
    let Pair(a, b) = pair;
    println!("a: {}, b: {}", a, b);

    // test case
    let r = Rectangle {
        length: 3.2,
        width: 3.4,
    };

    println!("Area: {}", rect_area(&r));

    // enum

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // test case for linked_list
    //
    let mut list = List::new();

    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// 元组结构体
#[derive(Debug)]
struct Pair(i32, i32);

#[derive(Debug)]
struct Rectangle {
    length: f32,
    width: f32,
}

fn rect_area(r: &Rectangle) -> f32 {
    let Rectangle {
        length: l,
        width: w,
    } = r;
    l * w
}
