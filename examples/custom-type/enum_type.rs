pub enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

pub fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),

        WebEvent::PageUnload => println!("page unloaded"),

        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),

        // 元组解构
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),

        // 结构体解构
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}
