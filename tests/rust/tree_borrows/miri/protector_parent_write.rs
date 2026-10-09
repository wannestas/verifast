// miri: ub
// Writing through a parent raw pointer while the `&mut` argument derived from it
// is protected (Tree Borrows: foreign write to a protected reference).

fn f(x: &mut i32, p: *mut i32) {
    unsafe { *p = 1; }
    *x = 2;
}

fn main() {
    let mut v = 0;
    let p = &raw mut v;
    f(unsafe { &mut *p }, p);
}
