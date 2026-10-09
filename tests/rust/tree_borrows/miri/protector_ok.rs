// miri: ok
// Twin of protector_parent_write.rs without the write through the parent pointer.

fn f(x: &mut i32, _p: *mut i32) {
    *x = 2;
}

fn main() {
    let mut v = 0;
    let p = &raw mut v;
    f(unsafe { &mut *p }, p);
    unsafe { assert!(*p == 2); }
}
