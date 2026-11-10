/*@

pred quarter(p: *i32, x: *i32, v: i32) = [1/4]ref_end_token(p, x) &*& [1/4]ref_initialized(p) &*& [1/4](*p |-> v);

@*/

unsafe fn reborrow_with_remaining_ri(x: *mut i32) -> i32
//@ req *x |-> ?v;
//@ ens *x |-> 5 &*& result == v;
{
    //@ let p = precreate_ref(x);
    //@ init_ref_readonly(p, 1/2);
    let r = &*x;
    //@ close quarter(p, x, v);
    //@ end_ref(p);
    //@ open quarter(p, x, v);
    let r2 = &*r;
    let result = *r2;
    //@ end_ref(p);
    *x = 5;
    result
}

unsafe fn reborrow_after_end(x: *mut i32)
//@ req *x |-> ?v;
//@ ens *x |-> 5;
{
    //@ let p = precreate_ref(x);
    //@ init_ref_readonly(p, 1/2);
    let r = &*x;
    //@ end_ref(p);
    *x = 5;
    let _r2 = &*r; //~ should_fail
}

fn main() {}
