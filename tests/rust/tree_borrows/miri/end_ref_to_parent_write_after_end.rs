// miri: ok
// Twin of `parent_write_after_end` in ../end_ref_to.rs.

fn main() {
    let mut v = 0;
    let x = &raw mut v;
    let r: &i32 = unsafe { &*x };
    let _result = *r;
    unsafe { *x = 5; }
}
