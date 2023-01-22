mod match_test;
mod testcase;

fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // 这个表达式返回一个 `i32` 类型。
        10 * n
    } else {
        println!(", and is a big number, half the number");

        // 这个表达式也必须返回一个 `i32` 类型。
        n / 2
        // 试一试 ^ 试着加上一个分号来结束这条表达式。
    };

    println!("{} -> {}", n, big_n);
}
