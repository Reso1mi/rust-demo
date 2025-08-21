use std::marker::PhantomPinned;
use std::ops::DerefMut;
use std::{pin::Pin, ptr::swap};

fn main() {}

fn pin_heap() {
    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl Test {
        fn new(txt: &str) -> Pin<Box<Self>> {
            let t = Test {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned,
            };
            let mut boxed = Box::pin(t);
            let self_ptr: *const String = &boxed.as_ref().a;
            unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

            boxed
        }

        fn a<'a>(self: Pin<&'a Self>) -> &'a str {
            &self.get_ref().a
        }

        fn b<'a>(self: Pin<&'a Self>) -> &'a String {
            unsafe { &*(self.b) }
        }
    }

    let mut test1 = Test::new("test1");
    let mut test2 = Test::new("test2");

    // Pin<Box<Test>> -> Pin<&mut Test>
    let mut test1 = test1.as_mut();
    let mut test2 = test2.as_mut();

    // std::mem::swap(test1.get_mut(), test2.get_mut());
}
fn pin_stack() {
    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl Test {
        fn new(txt: &str) -> Self {
            Test {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned, // This makes our type `!Unpin`
            }
        }

        fn init<'a>(self: Pin<&'a mut Self>) {
            let self_ptr: *const String = &self.a;
            let this = unsafe { self.get_unchecked_mut() };
            this.b = self_ptr;
        }

        fn a<'a>(self: Pin<&'a Self>) -> &'a str {
            &self.get_ref().a
        }

        fn b<'a>(self: Pin<&'a Self>) -> &'a String {
            unsafe { &*(self.b) }
        }
    }

    let mut test1 = Test::new("123");
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());

    let mut test2 = Test::new("234");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    Test::init(test2.as_mut());

    println!(
        "a: {}, b: {}",
        Test::a(test1.as_ref()),
        Test::b(test1.as_ref())
    );
    // std::mem::swap(test1.get_mut(), test2.get_mut());
    println!(
        "a: {}, b: {}",
        Test::a(test2.as_ref()),
        Test::b(test2.as_ref())
    );
}

fn swap_move() {
    let mut x = String::from("xxx");
    let mut y = String::from("yyy");

    std::mem::swap(&mut x, &mut y);

    assert_eq!("yyy", &x);
    assert_eq!("xxx", &y);
}

fn move_pinned_ref<T>(mut a: T, mut b: T) {
    unsafe {
        let a: Pin<&mut T> = Pin::new_unchecked(&mut a);
        // This should mean the pointee `a` can never move again.
        // 报错，编译器无法确认T是否是Unpin,只有Unpin才能安全的取出mut
        // std::mem::swap(a.get_mut(), &mut b);
        std::mem::swap(a.get_unchecked_mut(), &mut b);
    }
    std::mem::swap(&mut a, &mut b);
    // The address of `a` changed to `b`'s stack slot, so `a` got moved even
    // though we have previously pinned it! We have violated the pinning API contract.
}

fn move_pinned_ref_unpin<T: Unpin>(mut a: T, mut b: T) {
    unsafe {
        let a: Pin<&mut T> = Pin::new_unchecked(&mut a);
        // This should mean the pointee `a` can never move again.
        std::mem::swap(a.get_mut(), &mut b);
    }
    std::mem::swap(&mut a, &mut b);
    // The address of `a` changed to `b`'s stack slot, so `a` got moved even
    // though we have previously pinned it! We have violated the pinning API contract.
}

fn move_pined_ref_2() {
    let mut s = String::from("123");
    let mut b = String::from("213");
    // unsafe {
    let s = unsafe { Pin::new_unchecked(&mut s) };
    std::mem::swap(s.get_mut(), &mut b);

    // }
    // std::mem::swap(&mut s, &mut b);
}
