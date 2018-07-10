# sentinel_int

A compact representation for `Option<u64>`, obtained by using `u64::max_value()` as a sentinel.
Similar to [std::num::NonZeroU64](https://doc.rust-lang.org/std/num/struct.NonZeroU64.html), except that the sentinel value is not 0.

Compared to a NonZero implementation of u64, this implementation is easier to use as index in e.g. collections.
This representation is solely meant as a means of storing the `Option` more space-efficiently
(e.g. before sending on network, saving on disk, keeping in large in-memory structures).
Users are expected to use the `From` trait to convert it back to an `Option` before an actual use of the value.

# Examples
```rust
use sentinel_int::int_sentinel::IntSentinel;
// Convert an option into an IntSentinel
let sentinel = IntSentinel::from(Some(42u64)); // The sentinel is "just a u64"
// [...]
// Convert back the sentinel into an Option
let from_sentinel = Option::<u64>::from(sentinel);
assert_eq!(from_sentinel, Some(42u64));
```

```rust
use sentinel_int::int_sentinel::IntSentinel;
// Convert an option into an IntSentinel
let sentinel = IntSentinel::from(None); // The sentinel is "just a u64"
// [...]
// Convert back the sentinel into an Option
let from_sentinel = Option::<u64>::from(sentinel);
assert_eq!(from_sentinel, None);
```
# Build
```
cargo build
```
