/*@

lem end_named_target(a: *i32, p: *i32, g: *i32)
    req [1/2]ref_end_token(a, p) &*& [1/2]ref_end_token(a, g) &*& *a |-> ?v &*& ref_initialized(a);
    ens [1/2](*p |-> v) &*& [1/2]ref_end_token(a, g) &*& [1/2](*a |-> v) &*& [1/2]ref_initialized(a);
{
    end_ref_to(a, p);
}

lem end_other_target(a: *i32, p: *i32, g: *i32)
    req [1/2]ref_end_token(a, p) &*& *a |-> ?v &*& ref_initialized(a) &*& p != g;
    ens true;
{
    end_ref_to(a, g); //~ should_fail
}

@*/

unsafe fn parent_write_after_end(x: *mut i32) -> i32
//@ req *x |-> ?v;
//@ ens *x |-> 5 &*& result == v;
{
    //@ let p = precreate_ref(x);
    //@ init_ref_readonly(p, 1/2);
    let r = &*x;
    let result = *r;
    //@ end_ref_to(p, x);
    *x = 5;
    result
}

unsafe fn read_after_end(x: *mut i32) -> i32
//@ req *x |-> ?v;
//@ ens *x |-> 5;
{
    //@ let p = precreate_ref(x);
    //@ init_ref_readonly(p, 1/2);
    let r = &*x;
    //@ end_ref_to(p, x);
    *x = 5;
    *r //~ should_fail
}

fn main() {}
