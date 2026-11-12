// miri: ub
// Twin of write_parent in protection_obligation_fails.rs, whose protection obligation fails: the
// callee writes its protected &mut, then writes through the parent p during the call (a foreign
// write to a protected tag).
unsafe fn write_parent(x: &mut i32, p: *mut i32) {
    *x = 0;
    unsafe { *p = 1; }
}

fn main() {
    let mut v = 0;
    let p = &raw mut v;
    unsafe { write_parent(&mut *p, p) };
}
