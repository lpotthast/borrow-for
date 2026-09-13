use crate::BorrowFor;
#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
use alloc::sync::Arc;
#[cfg(feature = "alloc")]
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    ffi::CString,
    rc::Rc,
    string::String,
    vec::Vec,
};

#[cfg(feature = "alloc")]
use core::ffi::CStr;
#[cfg(feature = "std")]
use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

// Generic contexts. These also cover identical container types.
impl<T: ?Sized> BorrowFor<T> for T {
    type View = T;
}
impl<T: ?Sized> BorrowFor<T> for &T {
    type View = T;
}
impl<T: ?Sized> BorrowFor<T> for &mut T {
    type View = T;
}
#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
impl<T: ?Sized> BorrowFor<T> for Arc<T> {
    type View = T;
}
#[cfg(feature = "alloc")]
impl<T: ?Sized> BorrowFor<T> for Box<T> {
    type View = T;
}
#[cfg(feature = "alloc")]
impl<T: ToOwned + ?Sized> BorrowFor<T> for Cow<'_, T> {
    type View = T;
}
#[cfg(feature = "alloc")]
impl<T: ?Sized> BorrowFor<T> for Rc<T> {
    type View = T;
}

// Keep this policy in sync with the owned/borrowed table in COVERAGE.md.
#[cfg(feature = "alloc")]
macro_rules! owned_borrowed {
    ($owned:ty, $borrowed:ty) => {
        impl BorrowFor<$borrowed> for $owned {
            type View = $borrowed;
        }
        impl BorrowFor<$borrowed> for &$owned {
            type View = $owned;
        }
        impl BorrowFor<$borrowed> for &mut $owned {
            type View = $owned;
        }

        impl BorrowFor<&$borrowed> for $owned {
            type View = $owned;
        }
        impl BorrowFor<&$borrowed> for &$owned {
            type View = $owned;
        }
        impl BorrowFor<&$borrowed> for &mut $owned {
            type View = $owned;
        }

        impl BorrowFor<$owned> for $borrowed {
            type View = $borrowed;
        }
        impl BorrowFor<$owned> for &$borrowed {
            type View = $borrowed;
        }
        impl BorrowFor<$owned> for &mut $borrowed {
            type View = $borrowed;
        }
        impl<'s> BorrowFor<$owned> for &&'s $borrowed {
            type View = &'s $borrowed;
        }
        impl<'s> BorrowFor<&$owned> for &'s $borrowed {
            type View = &'s $borrowed;
        }

        impl BorrowFor<$owned> for Box<$borrowed> {
            type View = $borrowed;
        }
        impl BorrowFor<$owned> for Rc<$borrowed> {
            type View = $borrowed;
        }
        #[cfg(target_has_atomic = "ptr")]
        impl BorrowFor<$owned> for Arc<$borrowed> {
            type View = $borrowed;
        }
        impl BorrowFor<$owned> for Cow<'_, $borrowed> {
            type View = $borrowed;
        }
        impl<'a> BorrowFor<$owned> for &Cow<'a, $borrowed> {
            type View = Cow<'a, $borrowed>;
        }
        impl<'a> BorrowFor<$owned> for &mut Cow<'a, $borrowed> {
            type View = Cow<'a, $borrowed>;
        }

        impl BorrowFor<Cow<'_, $borrowed>> for $borrowed {
            type View = $borrowed;
        }
        impl BorrowFor<Cow<'_, $borrowed>> for &$borrowed {
            type View = $borrowed;
        }
        impl BorrowFor<Cow<'_, $borrowed>> for &mut $borrowed {
            type View = $borrowed;
        }
        impl BorrowFor<Cow<'_, $borrowed>> for $owned {
            type View = $borrowed;
        }
        impl BorrowFor<Cow<'_, $borrowed>> for &$owned {
            type View = $owned;
        }
        impl BorrowFor<Cow<'_, $borrowed>> for &mut $owned {
            type View = $owned;
        }
        impl BorrowFor<Cow<'_, $borrowed>> for Box<$borrowed> {
            type View = $borrowed;
        }
        impl BorrowFor<Cow<'_, $borrowed>> for Rc<$borrowed> {
            type View = $borrowed;
        }
        #[cfg(target_has_atomic = "ptr")]
        impl BorrowFor<Cow<'_, $borrowed>> for Arc<$borrowed> {
            type View = $borrowed;
        }
    };
}

#[cfg(feature = "alloc")]
owned_borrowed!(String, str);
#[cfg(feature = "alloc")]
owned_borrowed!(CString, CStr);
#[cfg(feature = "std")]
owned_borrowed!(PathBuf, Path);
#[cfg(feature = "std")]
owned_borrowed!(OsString, OsStr);

// Cross-container sequence mappings intentionally impose no relationship between T and U.
impl<T, U, const N: usize> BorrowFor<[T]> for [U; N] {
    type View = [U];
}
impl<T, U, const N: usize> BorrowFor<[T]> for &[U; N] {
    type View = [U; N];
}
impl<T, U, const N: usize> BorrowFor<[T]> for &mut [U; N] {
    type View = [U; N];
}
#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<[T]> for Vec<U> {
    type View = [U];
}
#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<[T]> for &Vec<U> {
    type View = Vec<U>;
}
#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<[T]> for &mut Vec<U> {
    type View = Vec<U>;
}

impl<T, U, const N: usize> BorrowFor<&[T]> for [U; N] {
    type View = [U; N];
}
impl<T, U, const N: usize> BorrowFor<&[T]> for &[U; N] {
    type View = [U; N];
}
impl<T, U, const N: usize> BorrowFor<&[T]> for &mut [U; N] {
    type View = [U; N];
}
#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<&[T]> for Vec<U> {
    type View = Vec<U>;
}
#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<&[T]> for &Vec<U> {
    type View = Vec<U>;
}
#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<&[T]> for &mut Vec<U> {
    type View = Vec<U>;
}

impl<T, U, const N: usize> BorrowFor<[T; N]> for [U] {
    type View = [U];
}
impl<T, U, const N: usize> BorrowFor<[T; N]> for &[U] {
    type View = [U];
}
impl<T, U, const N: usize> BorrowFor<[T; N]> for &mut [U] {
    type View = [U];
}
#[cfg(feature = "alloc")]
impl<T, U, const N: usize> BorrowFor<[T; N]> for Vec<U> {
    type View = [U];
}

#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<Vec<T>> for [U] {
    type View = [U];
}
#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<Vec<T>> for &[U] {
    type View = [U];
}
#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<Vec<T>> for &mut [U] {
    type View = [U];
}
#[cfg(feature = "alloc")]
impl<T, U, const N: usize> BorrowFor<Vec<T>> for [U; N] {
    type View = [U; N];
}
#[cfg(feature = "alloc")]
impl<T, U, const N: usize> BorrowFor<Vec<T>> for &[U; N] {
    type View = [U; N];
}
#[cfg(feature = "alloc")]
impl<T, U, const N: usize> BorrowFor<Vec<T>> for &mut [U; N] {
    type View = [U; N];
}
#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<Vec<T>> for Box<[U]> {
    type View = [U];
}
#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<Vec<T>> for Rc<[U]> {
    type View = [U];
}
#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
impl<T, U> BorrowFor<Vec<T>> for Arc<[U]> {
    type View = [U];
}
#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<Vec<T>> for Cow<'_, [U]>
where
    [U]: ToOwned,
{
    type View = [U];
}
#[cfg(feature = "alloc")]
impl<'s, T, U> BorrowFor<&Vec<T>> for &'s [U] {
    type View = &'s [U];
}

// Cow<[T]> compares with slice references and Vec<U> on the MSRV.
#[cfg(feature = "alloc")]
impl<'s, T, U> BorrowFor<Cow<'_, [T]>> for &'s [U]
where
    [T]: ToOwned,
{
    type View = &'s [U];
}
#[cfg(feature = "alloc")]
impl<'s, T, U> BorrowFor<Cow<'_, [T]>> for &'s mut [U]
where
    [T]: ToOwned,
{
    type View = &'s mut [U];
}
#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<Cow<'_, [T]>> for Vec<U>
where
    [T]: ToOwned,
{
    type View = Vec<U>;
}
#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<Cow<'_, [T]>> for &Vec<U>
where
    [T]: ToOwned,
{
    type View = Vec<U>;
}
#[cfg(feature = "alloc")]
impl<T, U> BorrowFor<Cow<'_, [T]>> for &mut Vec<U>
where
    [T]: ToOwned,
{
    type View = Vec<U>;
}
