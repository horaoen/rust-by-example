use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct EvenNumber(pub i32);

impl TryFrom<i32> for EvenNumber {
    //alais for empty tuple
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

