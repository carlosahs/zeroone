use std::ops::Add;

#[derive(PartialEq, Debug)]
struct AVar<T>(Option<T>);

impl<'a, T> Add<&'a AVar<T>> for &'a AVar<T>
where
    &'a T: Add<Output = T>,
{
    type Output = AVar<T>;

    fn add(self, rhs: &'a AVar<T>) -> Self::Output {
        let lhs = match &self.0 {
            Some(v) => v,
            None => panic!("left-hand side variable is undefined"),
        };
        let rhs = match &rhs.0 {
            Some(v) => v,
            None => panic!("right-hand side variable is undefined"),
        };
        let addition = lhs + rhs;
        AVar(Some(addition))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_equality() {
        let x = AVar(Some(10));
        let y = AVar(Some(10));
        assert_eq!(x, y);
    }

    #[test]
    fn variable_addition() {
        let x = AVar(Some(10));
        let y = AVar(Some(55));
        let z = AVar(Some(65));
        assert_eq!(z, &x + &y);
    }

    #[test]
    fn variable_commutative_addition() {
        let x = AVar(Some(10));
        let y = AVar(Some(55));
        assert_eq!(&x + &y, &y + &x);
    }

    #[test]
    #[should_panic(expected = "left-hand side variable is undefined")]
    fn variable_addition_panics_with_undefined_lhs() {
        let x: AVar<i32> = AVar(None);
        let y = AVar(Some(55));
        let _ = &x + &y;
    }

    #[test]
    #[should_panic(expected = "right-hand side variable is undefined")]
    fn variable_addition_panics_with_undefined_rhs() {
        let x = AVar(Some(10));
        let y: AVar<i32> = AVar(None);
        let _ = &x + &y;
    }
}
