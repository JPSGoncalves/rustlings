// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a hint.

struct Wrapper<T1,T2>  {
    value1: T1,
    value2: T2
}

impl<T1,T2> Wrapper<T1,T2> {
    pub fn new(val1: T1, val2: T2) -> Self {
        Wrapper { value1: val1, value2:val2 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        let w1 = Wrapper::new(42,"Test");
        assert_eq!(w1.value1, 42);
        assert_eq!(w1.value2, "Test");
    }

    #[test]
    fn store_str_in_wrapper() {
        let w2 = Wrapper::new("Foo", (10, 5));
        assert_eq!(w2.value1, "Foo");
        assert_eq!(w2.value2, (10, 5));
    }
}
