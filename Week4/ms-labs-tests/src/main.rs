fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn add_works() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(10, 12), 22);
    assert_eq!(add(5, -2), 3);
}

#[test]
/// The should_panic attribute makes it possible to check for a panic!. 
/// If we add this attribute to our test function, the test passes when 
/// the code in the function panics. The test fails when the code 
/// doesn't panic.
#[should_panic]
fn add_fails() {
    assert_eq!(add(2, 2), 7);
}

#[test]
/// A function annotated with the [test] attribute can also be annotated 
/// with the [ignore] attribute. This attribute causes that test function 
/// to be skipped during tests.
#[ignore = "not yet reviewed by the Q.A. team"]
fn add_negatives() {
    assert_eq!(add(-2, -2), -4)
}


/// Most unit tests go into a submodule with the #[cfg(test)] attribute.
#[cfg(test)]
mod add_function_tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(2, 2), 7);
    }

    #[test]
    #[ignore]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4)
    }
}

/// Exercise - Write unit tests
pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert_eq!(is_even(2), true);
    }

    #[test]
    fn is_false_when_odd() {
        assert_eq!(is_even(3), false);
    }
}