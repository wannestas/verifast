// miri: ub
// Twin of `reborrow_after_end` in ../end_ref_ri.rs.

fn main() {
    let mut v = 0;
    let x = &raw mut v;
    let r: &i32 = unsafe { &*x };
    unsafe { *x = 5; }
    let _r2: &i32 = &*r;
}
