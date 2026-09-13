#![no_std]
#![doc = include_str!("../README.md")]
#![doc = include_str!("../COVERAGE.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use core::borrow::Borrow;

/// Chooses the type to borrow when a value is used with a particular context.
///
/// The context is a type your generic code already knows. Each implementation chooses a
/// [`View`](Self::View) for that context. Use [`borrow_for`] or [`Borrow::borrow`] to get a
/// reference to the chosen view.
///
/// `Context` and `View` can be different types. For example, with the `alloc` feature, a string
/// literal in a `String` context has a `str` view. You only need the context's type, so you do
/// not have to create a value of that type.
///
/// Both `Context` and `View` can be unsized, such as `str` or `[T]`. This trait does not require
/// cloning, comparison, or formatting support. Your code adds any bounds it needs to use the
/// borrowed view.
///
/// The crate provides implementations for values, references, and arrays. The `alloc`
/// feature adds strings, C strings, vectors, `Box`, `Rc`, and `Cow`. It also adds `Arc` on targets
/// with pointer-sized atomic operations. The default `std` feature enables `alloc` and adds
/// paths and OS strings. See the [crate documentation](crate) for the view each implementation
/// chooses.
///
/// # Implementing `BorrowFor`
///
/// First, implement [`Borrow<View>`](Borrow) to return a reference to your view. Then implement
/// `BorrowFor<Context>` to choose that view for the context:
///
/// ```
/// use borrow_for::{BorrowFor, borrow_for};
/// use core::borrow::Borrow;
///
/// struct Text<'a>(&'a str);
///
/// impl Borrow<str> for Text<'_> {
///     fn borrow(&self) -> &str {
///         self.0
///     }
/// }
///
/// impl BorrowFor<String> for Text<'_> {
///     type View = str;
/// }
///
/// let text = Text("hello");
/// assert_eq!(borrow_for::<String, _>(&text), "hello");
/// ```
///
/// [`Borrow`] requires your type and its view to agree on equality, ordering, and hashing
/// whenever they implement those operations. If the borrowed data would behave differently,
/// use [`AsRef`] or an accessor instead.
///
/// # References
///
/// Implementing `BorrowFor<Context>` for `T` does not automatically implement it for `&T` or
/// `&mut T`. Add those implementations explicitly if your API needs them. `BorrowFor` does not
/// strip reference layers to find an implementation.
///
/// A reference can also choose a different view from an owned value. In a `str` context, a
/// `String` is borrowed as `str`, but `&String` and `&mut String` are borrowed as `String`.
/// Pass `.as_str()` if you want a `str` view from a borrowed `String`.
pub trait BorrowFor<Context: ?Sized>: Borrow<Self::View> {
    /// The type to borrow when this value is used in this context.
    type View: ?Sized;
}

/// Borrows a value using the view chosen for `Context`.
///
/// [`BorrowFor<Context>`](BorrowFor) chooses the view, and this function calls [`Borrow::borrow`]
/// once to get a reference to it. Both the input and the view can be unsized, such as `str` or
/// `[T]`.
///
/// ```
/// use borrow_for::borrow_for;
///
/// let array = [1, 2, 3];
/// let slice: &[i32] = borrow_for::<[i32], _>(&array);
/// assert_eq!(slice, &[1, 2, 3]);
/// assert_eq!(borrow_for::<[i32], _>(slice), slice);
/// ```
#[must_use]
pub fn borrow_for<Context: ?Sized, V: BorrowFor<Context> + ?Sized>(value: &V) -> &V::View {
    Borrow::<V::View>::borrow(value)
}

mod impls;
