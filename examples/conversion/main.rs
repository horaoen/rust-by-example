mod try_conver;
mod tostring;

use std::{convert::From, str::FromStr};

use crate::{try_conver::EvenNumber, tostring::Circle};

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;

    // Try removing the type declaration
    let num: Number = int.into();
    let _n = Number::from(3);
    println!("My number is {:?}", num);
    assert_eq!(try_conver::EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    println!("tostring testcase:");
    
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string()); 
    let circle2 = Circle::from_str("9").unwrap();
    println!("circle parse: {}", circle2.to_string());
    println!("parse : {}", "9".parse::<Circle>().unwrap());
}
