pub mod output;
pub mod rng;
pub mod time;

#[macro_export]
macro_rules! scope {
    ($name:ident, $ty:ty) => {
        use std::cell::RefCell;

        thread_local! {
            static SCOPE: RefCell<Option<$ty>> = RefCell::new(None);
        }

        pub fn set($name: Option<$ty>) -> Option<$ty> {
            try_borrow_mut(|r| core::mem::replace(r, $name))
        }

        pub fn with<F: FnOnce() -> R, R>($name: $ty, f: F) -> R {
            let prev = set(Some($name));
            let res = f();
            let _ = set(prev);
            res
        }

        pub fn try_borrow_mut<F: FnOnce(&mut Option<$ty>) -> R, R>(f: F) -> R {
            SCOPE.with(|r| f(&mut *r.borrow_mut()))
        }

        pub fn borrow<F: FnOnce(&$ty) -> R, R>(f: F) -> R {
            SCOPE.with(|r| {
                f(&*r.borrow().as_ref().expect(concat!(
                    "missing ",
                    stringify!($name),
                    " in thread scope"
                )))
            })
        }

        pub fn borrow_mut<F: FnOnce(&mut $ty) -> R, R>(f: F) -> R {
            SCOPE.with(|r| {
                f(&mut *r.borrow_mut().as_mut().expect(concat!(
                    "missing ",
                    stringify!($name),
                    " in thread scope"
                )))
            })
        }
    };
}
