// miri: ub
// Twin of `read_after_end` in ../end_ref_to.rs.

fn main() {
    let mut v = 0;
    let x = &raw mut v;
    let r: &i32 = unsafe { &*x };
    unsafe { *x = 5; }
    let _result = *r;
}
