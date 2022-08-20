use std::fmt::Debug;

/// Collection of equality assertions.
pub trait EqualityAssertion<T> {
    /// Verifies values are equal.
    ///
    /// Uses [PartialEq] to compare values.
    ///
    /// # Panics
    ///
    /// When [PartialEq] returns false.
    ///
    /// # Examples
    /// ```
    /// # use assertr::EqualityAssertion;
    /// 1.should_be(1);
    /// ```
    fn should_be(self, expected: T);
}

impl<T> EqualityAssertion<T> for T
where
    T: PartialEq + Debug,
{
    fn should_be(self, expected: T) {
        assert_eq!(self, expected, "Expected {:?} but was {:?}", expected, self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod should_be {
        use super::*;

        #[test]
        #[should_panic(expected = "Expected 2 but was 1")]
        fn with_values_not_matching_panics() {
            1.should_be(2);
        }

        #[test]
        fn with_values_matching_does_not_panic() {
            1.should_be(1);
        }
    }
}
