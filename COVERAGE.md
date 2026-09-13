# Implementation coverage

Use these tables to look up which borrowed view an input type provides for a context.
`Input` is the type that implements `BorrowFor<Context>`. The `borrow_for` helper takes
`&Input` and returns `&View`. For example, a row with `&String` as its input means you pass
an `&&String` to the helper. These choices of view are part of the crate's public API.

A pair in the table means you can borrow that view. The function using it may need other
bounds, such as `PartialEq` for comparisons or `Debug` for formatting. Whether two different
types can be compared also depends on the implementations available in your Rust version.

## Generic contexts

`T` can be any type, including an unsized type such as `str` or `[u8]`. It can also be a
reference or a container, so these rules cover borrowing a container as its own type too.

| Context | Input                       | View | Feature               |
|---------|-----------------------------|------|-----------------------|
| `T`     | `T`, `&T`, `&mut T`         | `T`  | Core only             |
| `T`     | `Box<T>`, `Rc<T>`, `Arc<T>` | `T`  | `alloc`               |
| `T`     | `Cow<'_, T>`                | `T`  | `alloc`; `T: ToOwned` |

`Arc` is supported only on targets with pointer-sized atomic operations
(`target_has_atomic = "ptr"`).

## Owned and borrowed families

Each owned type below has a corresponding borrowed type, and all four pairs use the same
rules. In the second table, replace `O` with the owned type, such as `String`, and `B` with
its borrowed type, such as `str`. These implementations exist for the pairs listed here.
Implementing `ToOwned` on another type does not automatically add it to this table.

| Owned type `O` | Borrowed type `B` | Feature |
|----------------|-------------------|---------|
| `String`       | `str`             | `alloc` |
| `CString`      | `CStr`            | `alloc` |
| `PathBuf`      | `Path`            | `std`   |
| `OsString`     | `OsStr`           | `std`   |

| Context      | Input                                     | View         |
|--------------|-------------------------------------------|--------------|
| `B`          | `O`                                       | `B`          |
| `B`          | `&O`, `&mut O`                            | `O`          |
| `&B`         | `O`, `&O`, `&mut O`                       | `O`          |
| `O`          | `B`, `&B`, `&mut B`                       | `B`          |
| `O`          | `&&'s B`                                  | `&'s B`      |
| `O`          | `Box<B>`, `Rc<B>`, `Arc<B>`, `Cow<'_, B>` | `B`          |
| `O`          | `&Cow<'a, B>`, `&mut Cow<'a, B>`          | `Cow<'a, B>` |
| `&O`         | `&'s B`                                   | `&'s B`      |
| `Cow<'_, B>` | `B`, `&B`, `&mut B`, `O`                  | `B`          |
| `Cow<'_, B>` | `&O`, `&mut O`                            | `O`          |
| `Cow<'_, B>` | `Box<B>`, `Rc<B>`, `Arc<B>`               | `B`          |

The generic implementations above also cover `O` in an `O` context and smart pointers in
a `B` context. The input's lifetime does not have to match a lifetime in the context type.
For example, a short-lived `&str` can use a `Cow<'static, str>` context without having to
live for `'static` itself.

## Sequences

`T` is the context's element type, and `U` is the input's element type. They can be different,
and the borrowed view keeps the input's element type. These implementations do not require
equality, cloning, or formatting support, apart from the `ToOwned` bounds shown for `Cow`.
`N` is the array length and can be zero.

| Context        | Input                              | View          | Feature                 |
|----------------|------------------------------------|---------------|-------------------------|
| `[T]`          | `[U; N]`                           | `[U]`         | Core only               |
| `[T]`          | `&[U; N]`, `&mut [U; N]`           | `[U; N]`      | Core only               |
| `[T]`          | `Vec<U>`                           | `[U]`         | `alloc`                 |
| `[T]`          | `&Vec<U>`, `&mut Vec<U>`           | `Vec<U>`      | `alloc`                 |
| `&[T]`         | `[U; N]`, `&[U; N]`, `&mut [U; N]` | `[U; N]`      | Core only               |
| `&[T]`         | `Vec<U>`, `&Vec<U>`, `&mut Vec<U>` | `Vec<U>`      | `alloc`                 |
| `[T; N]`       | `[U]`, `&[U]`, `&mut [U]`          | `[U]`         | Core only               |
| `[T; N]`       | `Vec<U>`                           | `[U]`         | `alloc`                 |
| `Vec<T>`       | `[U]`, `&[U]`, `&mut [U]`          | `[U]`         | `alloc`                 |
| `Vec<T>`       | `[U; N]`, `&[U; N]`, `&mut [U; N]` | `[U; N]`      | `alloc`                 |
| `Vec<T>`       | `Box<[U]>`, `Rc<[U]>`, `Arc<[U]>`  | `[U]`         | `alloc`                 |
| `Vec<T>`       | `Cow<'_, [U]>`                     | `[U]`         | `alloc`; `[U]: ToOwned` |
| `&Vec<T>`      | `&'s [U]`                          | `&'s [U]`     | `alloc`                 |
| `Cow<'_, [T]>` | `&'s [U]`                          | `&'s [U]`     | `alloc`; `[T]: ToOwned` |
| `Cow<'_, [T]>` | `&'s mut [U]`                      | `&'s mut [U]` | `alloc`; `[T]: ToOwned` |
| `Cow<'_, [T]>` | `Vec<U>`, `&Vec<U>`, `&mut Vec<U>` | `Vec<U>`      | `alloc`; `[T]: ToOwned` |

For a `Cow<[T]>` context, the views are slice references or vectors. These match the forms
supported by its `PartialEq` implementations on Rust 1.85.1. For example, an `&[U]` input
keeps `&[U]` as its view in a `Cow<[T]>` context, but uses `[U]` in a `Vec<T>` context.

## Coverage policy and limits

- The tables and generic rules list the supported combinations. Adding another reference
  layer does not automatically preserve an implementation. `Borrow` itself does not follow
  arbitrary chains of references: for example, `&String` borrows `String`.
- Each input type and context together choose exactly one view. Automatically carrying all
  implementations over to references would conflict with the generic implementations above.
  Rust rejects overlapping implementations, so reference support must be added explicitly.
- Two vectors with different element types need an explicit slice input, as shown below.
  An implementation for `Vec<U>` in a `Vec<T>` context would also apply when `T = U`, where
  the generic implementation already borrows the vector as its own type. Rust rejects that
  overlap. The same limitation applies to `[U]` inputs in `[T]` contexts.
- Custom views must preserve `Borrow`'s equality, ordering, and hashing behavior. Use `AsRef`
  or an accessor when the borrowed data behaves differently. You can extend the table with
  your own input or context types. Rust's orphan rules prevent you from adding an implementation
  when the trait and all types involved come from other crates.

To compare two vectors with different element types, borrow the expected vector as a slice:

```rust
use borrow_for::borrow_for;

# #[cfg(feature = "alloc")] {
let actual = vec![String::from("hello")];
let expected = vec!["hello"];
let view: &[&str] = borrow_for::<Vec<String>, _>(expected.as_slice());
assert!(actual.eq(view));
# }
```
