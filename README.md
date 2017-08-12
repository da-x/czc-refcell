# Conditional Zero-Cost RefCell

The [RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html) structure is often used in conjunction with the [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html) in order to allow multiple reference-counted references to mutable data in Rust, with the borrow checking pushed into run-time, using a bit of memory and performing run-time checks.

With `RefCell`, as long as the `try_borrow` and `try_borrow_mut` functions are not used, and as long as sufficiently done testing has empirically verified that the `borrow` and `borrow_mut` function don't panic on run-time, then the overhead of `RefCell` can be reduced to zero using a feature enablement in the compilation stage, treating it as an `UnsafeCell`, its underlying implementation, without changing program semantics.

Thus, the `czc-refcell` module makes a proxy to the standard library `RefCell` that allows it to function as an `UnsafeCell`, if the `unchecked_refcell` feature is enabled, using a different type for `RefCell`. Otherwise, the standard `std::cell::RefCell` is proxied as it is with all the guarantees it provides.

This provided compile-time conditional type has the same overhead of `UnsafeCell`, but mimics the interface of `RefCell`. Its implementation is based on a stripped-down `RefCell` from the standard library, where the `borrow` and `borrow_mut` are in-lined to a direct proxy into the `UnsafeCell` without any checking.

# Usage

```rust
extern czc_refcell;

use czc_refcell::cell::RefCell;

...
```

You should allow the users of your library to control whether you'd be giving them a checked or an unchecked RefCell types. At any case, if there is a mismatch of expectation, Rust's compiler will present a meaningful error.

```toml
[features]
unchecked_refcell = ["czc-refcell/unchecked_refcell"]
```

You can test the performance of a unchecked `RefCell` with a build feature flag, e.g:

```shell
cargo build --features 'fast_refcell'
```
