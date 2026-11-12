// miri: ok
// Control for obligation_parent_write.rs: the same accesses, with the &mut a local reborrow inside the
// callee instead of a parameter, so it is not protected.
unsafe fn write_parent(x: *mut i32, p: *mut i32) {
    let x: &mut i32 = unsafe { &mut *x };
    *x = 0;
    unsafe { *p = 1; }
}

fn main() {
    let mut v = 0;
    let p = &raw mut v;
    unsafe { write_parent(p, p) };
}
