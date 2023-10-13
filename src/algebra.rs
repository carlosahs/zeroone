use std::ops::Add;

#[derive(PartialEq, Debug)]
struct AVa<T> {
    value: Option<T>,
}

impl<'a, T> Add<&'a AVa<T>> for &'a AVa<T>
where
    &'a T: Add<Output = T>,
{
    type Output = AVa<T>;

    fn add(self, rhs: &'a AVa<T>) -> Self::Output {
        let lhs = match &self.value {
            Some(v) => v,
            None => panic!("left-hand side variable is undefined"),
        };
        let rhs = match &rhs.value {
            Some(v) => v,
            None => panic!("right-hand side variable is undefined"),
        };
        let addition = lhs + rhs;
        AVa { value: Some(addition) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_addition() {
        let x = AVa { value: Some(10) };
        let y = AVa { value: Some(45) };
        let z = AVa { value: Some(55) };
        assert_eq!(z, &x + &y);
    }

    #[test]
    fn variable_commutative_addition() {
        let x = AVa { value: Some(10) };
        let y = AVa { value: Some(45) };
        assert_eq!(&x + &y, &y + &x);
    }

    #[test]
    #[should_panic(expected = "left-hand side variable is undefined")]
    fn variable_addition_panics_with_undefined_lhs() {
        let x: AVa<i32> = AVa { value: None };
        let y = AVa { value: Some(45) };
        let _ = &x + &y;
    }

    #[test]
    #[should_panic(expected = "right-hand side variable is undefined")]
    fn variable_addition_panics_with_undefined_rhs() {
        let x = AVa { value: Some(45) };
        let y: AVa<i32> = AVa { value: None };
        let _ = &x + &y;
    }
}

