#[cfg(test)]
pub mod testcase {
    use super::*;

    #[test]
    fn loop_test() {
        let mut count = 0u32;

        println!("Let's count until infinity!");
        // 无限循环
        loop {
            count += 1;
            if count == 3 {
                // 跳过这次迭代的剩下内容
                continue;
            }
            println!("{}", count);
            if count == 5 {
                // 退出循环
                break;
            }
        }
    }

    #[test]
    fn loop_with_label() {
        'outer: loop {
            println!("Entered the outer loop");

            'inner: loop {
                println!("Entered the inner loop");

                // 这只是中断内部的循环
                //break;

                // 这会中断外层循环
                break 'outer;
            }

            println!("This point will never be reached");
        }

        println!("Exited the outer loop");
    }

    #[test]
    fn loop_return() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        assert_eq!(result, 20);
    }

    #[test]
    fn match_guard() {
        let pair = (2, -2);

        println!("Tell me about {:?}", pair);
        match pair {
            (x, y) if x == y => println!("These are twins"),
            // The ^ `if condition` part is a guard
            (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
            (x, _) if x % 2 == 1 => println!("The first one is odd"),
            // This should not be possible to reach
            _ => println!("No correlation..."),
        }
    }

    #[test]
    fn match_bind() {
        println!("Tell me what type of person you are");

        match 15 {
            0 => println!("I haven't celebrated my first birthday yet"),
            // 可以直接匹配（`match`） 1 ..= 12，但那样的话孩子会是几岁？
            // 相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
            n @ 1..=12 => println!("I'm a child of age {:?}", n),
            n @ 13..=19 => println!("I'm a teen of age {:?}", n),
            // 不符合上面的范围。返回结果。
            n => println!("I'm an old person of age {:?}", n),
        }

        match Some(42) {
            // 得到 `Some` 可变类型，如果它的值（绑定到 `n` 上）等于 42，则匹配。
            Some(n @ 42) => println!("The Answer: {}!", n),
            // 匹配任意其他数字。
            Some(n) => println!("Not interesting... {}", n),
            // 匹配任意其他值（`None` 可变类型）。
            _ => (),
        }
    }

    #[test]
    fn if_let() {
        let number = Some(7);
        let letter = Some('c');
        let emoticon: Option<i32> = None;

        if let Some(i) = number {
            println!("Matched {:?}!", i);
        }

        if let Some(i) = letter {
            println!("Matched {:?}!", i);
        } else {
            // 解构失败。切换到失败情形。
            println!("Didn't match a number. Let's go with a letter!");
        };

        enum Foo {
            Bar,
            Baz,
            Qux(u32),
        }

        // 创建变量
        let a = Foo::Bar;
        let b = Foo::Baz;
        let c = Foo::Qux(100);

        // 变量 b 没有匹配到 Foo::Bar，因此什么也不会打印。
        if let Foo::Bar = b {
            println!("b is foobar");
        }
    }

    #[test]
    fn while_let() {
        fn main() {
            // 将 `optional` 设为 `Option<i32>` 类型
            let mut optional = Some(0);

            // 这读作：当 `let` 将 `optional` 解构成 `Some(i)` 时，就
            // 执行语句块（`{}`）。否则就 `break`。
            while let Some(i) = optional {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ 使用的缩进更少，并且不用显式地处理失败情况。
            }
            // ^ `if let` 有可选的 `else`/`else if` 分句，
            // 而 `while let` 没有。
        }
    }
}
