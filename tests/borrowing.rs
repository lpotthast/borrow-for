// Reference forms intentionally exercise different selections in the public API.
#![allow(clippy::needless_borrows_for_generic_args)]

use borrow_for::{BorrowFor, borrow_for};
use core::{borrow::Borrow, cell::Cell};
#[cfg(feature = "alloc")]
use std::{borrow::Cow, rc::Rc};

// No Clone, comparison, or formatting implementations.
struct Opaque {
    _byte: u8,
}

#[track_caller]
fn assert_view<Context: ?Sized, View: ?Sized>(
    input: &(impl BorrowFor<Context, View = View> + ?Sized),
    expected: &View,
) {
    // Checks the exact associated type and address, including str/slice metadata.
    assert!(core::ptr::eq(borrow_for::<Context, _>(input), expected));
}

fn equal<L: ?Sized + PartialEq<R::View>, R: BorrowFor<L>>(left: &L, right: R) -> bool {
    left.eq(right.borrow())
}

#[test]
fn generic_views_preserve_reference_layers_without_extra_bounds() {
    let mut value = Opaque { _byte: 7 };
    assert_view::<Opaque, Opaque>(&value, &value);
    assert_view::<Opaque, Opaque>(&&value, &value);
    let shared = &value;
    assert_view::<&Opaque, &Opaque>(&shared, &shared);
    let mutable = &mut value;
    assert_view::<Opaque, Opaque>(&mutable, mutable);
    assert_view::<&mut Opaque, &mut Opaque>(&mutable, &mutable);

    let array = [Opaque { _byte: 8 }];
    assert_view::<[u8], [Opaque]>(&array, &array);
    assert_view::<[u8], [Opaque; 1]>(&&array, &array);
}

#[test]
fn custom_unsized_view_is_borrowed_once() {
    struct Context;
    struct Text<'a> {
        text: &'a str,
        calls: Cell<usize>,
    }
    impl Borrow<str> for Text<'_> {
        fn borrow(&self) -> &str {
            self.calls.set(self.calls.get() + 1);
            self.text
        }
    }
    impl BorrowFor<Context> for Text<'_> {
        type View = str;
    }
    let value = Text {
        text: "Grüße 🦀",
        calls: Cell::new(0),
    };
    assert_view::<Context, str>(&value, value.text);
    assert_eq!(value.calls.get(), 1);
}

#[test]
fn slices_and_arrays_support_ownership_forms_and_preserve_lengths() {
    let array = [1, 2, 3];
    let slice = &array[..];
    let mut expected = [1, 2, 3];
    assert!(equal(&array, slice));
    assert!(equal(&slice, expected));
    assert!(equal(&slice, &expected));
    assert!(equal(&slice, &mut expected));
    assert!(!equal(&array, &[1, 2][..]));
    for range in [0..0, 0..1, 1..3] {
        let part = &mut expected[range];
        assert_view::<[u8; 3], [i32]>(part, part);
        assert_view::<[u8; 3], [i32]>(&part, part);
    }
}

#[cfg(feature = "alloc")]
#[test]
fn heterogeneous_sequences_support_owned_and_borrowed_inputs() {
    let actual = vec![String::from("hello"), String::from("world")];
    let mut expected = ["hello", "world"];
    assert!(equal(&actual, expected));
    assert!(equal(&actual, &expected));
    assert!(equal(&actual, &mut expected));
    assert!(equal(&actual, &expected[..]));
    assert!(equal(&actual, &mut expected[..]));
    assert!(equal(&&actual, &expected[..]));
    assert!(equal(&actual[..], expected));
    assert!(equal(&actual[..], &expected));
    assert!(equal(&actual[..], &mut expected));
    assert!(!equal(&actual, &["other", "world"][..]));
    assert!(!equal(&actual, &expected[..1]));

    let mut vector = expected.to_vec();
    assert!(equal(&actual[..], vector.clone()));
    assert!(equal(&actual[..], &vector));
    assert!(equal(&actual[..], &mut vector));
    assert!(equal(&&actual[..], vector.clone()));
    assert!(equal(&&actual[..], &vector));
    assert!(equal(&&actual[..], &mut vector));
    assert!(equal(&["hello", "world"], actual));
}

#[cfg(feature = "alloc")]
#[test]
fn smart_pointers_and_cow_preserve_views_without_cloning() {
    struct CloneBomb {
        _byte: u8,
    }
    impl Clone for CloneBomb {
        fn clone(&self) -> Self {
            panic!("borrowing must not clone")
        }
    }

    let boxed = Box::<[Opaque]>::from([Opaque { _byte: 7 }]);
    assert_view::<[Opaque], [Opaque]>(&boxed, &boxed);
    assert_view::<Vec<u8>, [Opaque]>(&boxed, &boxed);
    let rc = Rc::<[Opaque]>::from(boxed);
    assert_view::<Vec<u8>, [Opaque]>(&rc, &rc);
    #[cfg(target_has_atomic = "ptr")]
    {
        let arc = std::sync::Arc::<[Opaque]>::from([Opaque { _byte: 8 }]);
        assert_view::<Vec<u8>, [Opaque]>(&arc, &arc);
    }

    let array = [CloneBomb { _byte: 7 }];
    for cow in [Cow::Borrowed(&array[..0]), Cow::Borrowed(&array[0..])] {
        assert_view::<Vec<u8>, [CloneBomb]>(&cow, cow.as_ref());
    }
    let cow = Cow::Owned::<[CloneBomb]>(Vec::from(array));
    assert_view::<Vec<u8>, [CloneBomb]>(&cow, cow.as_ref());
}

#[cfg(feature = "alloc")]
#[test]
fn strings_select_views_by_context_and_reference_form() {
    let mut expected = String::from("hello");
    assert_view::<str, str>(&expected, expected.as_str());
    assert_view::<str, String>(&&expected, &expected);
    assert_view::<&str, String>(&expected, &expected);
    let shared = expected.as_str();
    assert_view::<String, &str>(&&shared, &shared);
    assert_view::<&String, &str>(&shared, &shared);
    for actual in [Cow::Borrowed("hello"), Cow::Owned(String::from("hello"))] {
        assert!(equal(&actual, "hello"));
        assert!(equal(&actual, expected.clone()));
        assert!(equal(&actual, &expected));
        assert!(equal(&actual, &mut expected));
        assert!(equal(&actual, expected.as_mut_str()));
        assert!(equal(&actual, Box::<str>::from("hello")));
        assert!(!equal(&actual, String::from("other")));
        assert!(equal(&String::from("hello"), &actual));
    }
    assert_view::<Cow<'static, str>, str>(&expected.as_str(), expected.as_str());
}

#[cfg(feature = "alloc")]
#[test]
fn cow_slices_select_comparable_reference_and_vector_views() {
    let owned = vec![String::from("hello")];
    for actual in [Cow::Borrowed(&owned[..]), Cow::Owned(owned.clone())] {
        let mut expected = ["hello"];
        assert!(equal(&actual, &expected[..]));
        assert!(equal(&actual, &mut expected[..]));
        let mut vector = expected.to_vec();
        assert!(equal(&actual, vector.clone()));
        assert!(equal(&actual, &vector));
        assert!(equal(&actual, &mut vector));
        assert!(!equal(&actual, &["other"][..]));
        assert!(!equal(&actual, &[] as &[&str]));
    }
    let array = [Opaque { _byte: 7 }];
    let slice = &array[..];
    assert_view::<Cow<'static, [u8]>, &[Opaque]>(&slice, &slice);
}

#[cfg(feature = "alloc")]
#[test]
fn c_strings_support_owned_and_borrowed_contexts() {
    use std::ffi::{CStr, CString};
    let owned = CString::new("hello").unwrap();
    let borrowed = c"hello";
    assert_view::<CStr, CStr>(&owned, owned.as_c_str());
    assert_view::<CString, CStr>(borrowed, borrowed);
    assert_view::<CStr, CString>(&&owned, &owned);
    assert!(equal(borrowed, owned));
}

#[cfg(feature = "std")]
#[test]
fn paths_and_os_strings_support_owned_and_borrowed_contexts() {
    use std::{
        ffi::{OsStr, OsString},
        path::{Path, PathBuf},
    };
    let path = PathBuf::from("hello/world");
    assert_view::<Path, Path>(&path, path.as_path());
    assert_view::<Path, PathBuf>(&&path, &path);
    assert!(equal(&path, Path::new("hello/world")));
    assert!(equal(Path::new("hello/world"), path));
    let os_string = OsString::from("hello");
    assert_view::<OsStr, OsStr>(&os_string, os_string.as_os_str());
    assert_view::<OsStr, OsString>(&&os_string, &os_string);
    assert!(equal(&os_string, OsStr::new("hello")));
    assert!(equal(OsStr::new("hello"), os_string));
}
