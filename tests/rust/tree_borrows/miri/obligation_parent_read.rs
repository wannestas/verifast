// miri: ub
// Twin of read_parent in protection_obligation_fails.rs and protection_obligation_stash.rs, whose
// protection obligations fail (h1's twin): the callee writes its protected &mut (Active), then reads
// through the parent p during the call.
unsafe fn read_parent(x: &mut i32, p: *const i32) -> i32 {
    *x = 1;
    unsafe { *p }
}

fn main() {
    let mut v = 0;
    let p = &raw mut v;
    unsafe { read_parent(&mut *p, p) };
}
