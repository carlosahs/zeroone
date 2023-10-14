use std::iter::zip;
use std::ops::Add;

#[derive(PartialEq, Debug)]
struct LVec<T> {
    values: Vec<T>,
}

impl<'a, T> Add<&'a LVec<T>> for &'a LVec<T>
where
    &'a T: Add<Output = T>,
{
    type Output = LVec<T>;

    fn add(self, rhs: &'a LVec<T>) -> Self::Output {
        if self.values.len() != rhs.values.len() {
            panic!("vectors with different dimensionality");
        }
        let values = zip(&self.values, &rhs.values).map(|(x, y)| x + y).collect();
        LVec { values }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_vector_addition() {
        let x = LVec {
            values: vec![1, 2, 3],
        };
        let y = LVec {
            values: vec![4, 5, 6],
        };
        let z = LVec {
            values: vec![5, 7, 9],
        };
        assert_eq!(z, &x + &y);
    }

    #[test]
    #[should_panic(expected = "vectors with different dimensionality")]
    fn linear_vector_addition_panics() {
        let x = LVec {
            values: vec![1, 2, 3],
        };
        let y = LVec { values: vec![4, 5] };
        let _ = &x + &y;
    }

    #[test]
    fn linear_vector_commutative_addition() {
        let x = LVec {
            values: vec![1, 2, 3],
        };
        let y = LVec {
            values: vec![4, 5, 6],
        };
        assert_eq!(&x + &y, &y + &x);
    }
}
