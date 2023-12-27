struct AlVa<T> {
    value: Option<T>,
}

impl<T> AlVa<T> {
    pub fn new() -> AlVa<T> {
        AlVa { value: None }
    }

    pub fn from(v: T) -> AlVa<T> {
        AlVa { value: Some(v) }
    }

    pub fn value(&self) -> &Option<T> {
        &self.value
    }

    pub fn assign(&mut self, v: T) {
        self.value = Some(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_defined() {
        let x = AlVa::from(1);
        assert_eq!(x.value(), &Some(1));
    }

    #[test]
    fn variable_undefined() {
        let x: AlVa<i32> = AlVa::new();
        assert_eq!(x.value(), &None);
    }

    #[test]
    fn variable_undefined_then_defined() {
        let mut x = AlVa::new();
        assert_eq!(x.value(), &None);
        x.assign(1);
        assert_eq!(x.value(), &Some(1));
    }
}
