
pub mod int_sentinel {
    /// A compact representation for `Option<u64>`, obtained by using `u64::max_value()` as a sentinel.
    ///
    /// Compared to a NonZero implementation of u64, this implementation is easier to use as index in e.g. collections.
    /// This representation is solely meant as a means of storing the `Option` more space-efficiently
    /// (e.g. before sending on network, saving on disk, keeping in large in-memory structures).
    /// Users are expected to use the `From` trait to convert it back to an `Option` before an actual use of the value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sentinel_int::int_sentinel::IntSentinel;
    /// // Convert an option into an IntSentinel
    /// let sentinel = IntSentinel::from(Some(42u64)); // The sentinel is "just a u64"
    /// // [...]
    /// // Convert back the sentinel into an Option
    /// let from_sentinel = Option::<u64>::from(sentinel);
    /// assert_eq!(from_sentinel, Some(42u64));
    /// ```
    ///
    /// ```rust
    /// # use sentinel_int::int_sentinel::IntSentinel;
    /// // Convert an option into an IntSentinel
    /// let sentinel = IntSentinel::from(None); // The sentinel is "just a u64"
    /// // [...]
    /// // Convert back the sentinel into an Option
    /// let from_sentinel = Option::<u64>::from(sentinel);
    /// assert_eq!(from_sentinel, None);
    /// ```
    #[derive(Debug)]
    pub struct IntSentinel {
        value: u64,
    }

    impl IntSentinel {
        /// The maximum value that can be represented by this type.
        pub fn max_value() -> u64 {
            IntSentinel::sentinel() - 1
        }

        /// The sentinel value.
        pub fn sentinel() -> u64 {
            u64::max_value()
        }

        /// Constructs a new `IntSentinel` containing `None`.
        ///
        /// # Examples
        ///
        /// ```rust
        /// # use sentinel_int::int_sentinel::IntSentinel;
        /// let sentinel = IntSentinel::new_none();
        /// assert_eq!(sentinel.to_option(), None);
        /// ```
        pub fn new_none() -> Self {
            IntSentinel { value: u64::max_value() }
        }

        /// Constructs a new `IntSentinel` containing the provided `u64`.
        ///
        /// # Panics
        ///
        /// This function panics if `value` is greater than `max_value()` (i.e., if it equals `sentinel()`).
        ///
        /// # Examples
        ///
        /// ```rust
        /// # use sentinel_int::int_sentinel::IntSentinel;
        /// let sentinel = IntSentinel::new_with_some(42u64);
        /// assert_eq!(sentinel.to_option(), Some(42u64));
        /// ```
        pub fn new_with_some(value: u64) -> Self {
            if value == u64::max_value() {
                panic!("Illegal value: {} is the sentinel value.", value);
            }
            IntSentinel { value }
        }

        /// Returns an `Option` corresponding to the value contained in this instance.
        pub fn to_option(&self) -> Option<u64> {
            if self.value == u64::max_value() {
                None
            } else {
                Some(self.value)
            }
        }

        /// Constructs a new `IntSentinel` from a value without checking the sentinel value.
        ///
        /// # Safety
        ///
        /// If using this function to create an `IntSentinel`, `sentinel()` will be transformed into a `None` value,
        /// and any other `u64` will be mapped to a `Some` of the passed value.
        ///
        /// # Examples
        /// ```rust
        /// # use sentinel_int::int_sentinel::IntSentinel;
        /// unsafe {
        ///     assert_eq!(IntSentinel::unchecked_new(IntSentinel::sentinel()).to_option(), None)
        /// }
        /// ```
        ///
        /// ```rust
        /// # use sentinel_int::int_sentinel::IntSentinel;
        /// unsafe {
        ///     assert_eq!(IntSentinel::unchecked_new(42u64).to_option(), Some(42u64))
        /// }
        /// ```
        pub unsafe fn unchecked_new(value: u64) -> Self {
            IntSentinel { value }
        }

        /// Returns the raw contained value without a check.
        ///
        /// # Safety
        ///
        /// This method returns `sentinel()` when the instance contains `None`, it returns the contained value
        /// when the instance contains a different value.
        ///
        /// # Examples
        /// ```rust
        /// # use sentinel_int::int_sentinel::IntSentinel;
        /// unsafe {
        ///     assert_eq!(IntSentinel::from(Some(42)).to_u64_unchecked(), 42);
        /// }
        /// ```
        /// ```rust
        /// # use sentinel_int::int_sentinel::IntSentinel;
        /// unsafe {
        ///     assert_eq!(IntSentinel::from(None).to_u64_unchecked(), IntSentinel::sentinel());
        /// }
        /// ```
        pub unsafe fn to_u64_unchecked(&self) -> u64 {
            self.value
        }
    }

    impl From<Option<u64>> for IntSentinel {
        fn from(option: Option<u64>) -> Self {
            match option {
                Some(value) => IntSentinel::new_with_some(value),
                None => IntSentinel::new_none()
            }
        }
    }

    impl From<IntSentinel> for Option<u64> {
        fn from(sentinel : IntSentinel) -> Self {
            sentinel.to_option()
        }
    }
}

mod tests {
    #[cfg(test)]
    use int_sentinel::*;

    #[cfg(test)]
    #[test]
    fn unsafe_value() {
        let x = 42;
        unsafe {
            let sentinel = IntSentinel::unchecked_new(x);
            assert_eq!(sentinel.to_u64_unchecked(), x);
        }
    }

    #[cfg(test)]
    #[test]
    fn some_value() {
        let x = 42;
        let sentinel = IntSentinel::new_with_some(x);
        assert!(sentinel.to_option().is_some());
        let value = sentinel.to_option().unwrap();
        assert_eq!(value, x);
    }

    #[cfg(test)]
    #[test]
    fn none_value() {
        let sentinel = IntSentinel::new_none();
        assert!(sentinel.to_option().is_none());
    }

    #[cfg(test)]
    #[test]
    fn using_from_some() {
        let with_value = Some(42u64);
        let sentinel = IntSentinel::from(with_value);
        let from_sentinel = Option::<u64>::from(sentinel);
        assert_eq!(from_sentinel, with_value);
    }

    #[cfg(test)]
    #[test]
    fn using_from_none() {
        let sentinel = IntSentinel::from(None);
        let from_sentinel = Option::<u64>::from(sentinel);
        assert_eq!(from_sentinel, None);
    }

    #[cfg(test)]
    #[should_panic]
    #[test]
    fn some_illegal_value() {
        IntSentinel::new_with_some(u64::max_value());
    }

    #[cfg(test)]
    #[should_panic]
    #[test]
    fn using_from_illegal_value() {
        let with_value = Some(u64::max_value());
        IntSentinel::from(with_value);
    }
}
