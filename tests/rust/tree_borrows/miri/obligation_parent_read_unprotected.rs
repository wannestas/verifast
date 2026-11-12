// miri: ok
// Control for obligation_parent_read.rs: the same accesses, with the &mut a local reborrow inside the
// callee instead of a parameter, so it is not protected.
unsafe fn read_parent(x: *mut i32, p: *const i32) -> i32 {
    let x: &mut i32 = unsafe { &mut *x };
    *x = 1;
    unsafe { *p }
}

fn main() {
    let mut v = 0;
    let p = &raw mut v;
    unsafe { read_parent(p, p) };
}
