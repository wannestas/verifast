/*@

pred eighth_token(a: *i32, p: *i32) = [1/8]ref_end_token::<i32>(a, p);

lem chain_through_the_named_middle(r: *i32, x1: *i32, x2: *i32, p: *i32)
    req [1/2]ref_end_token::<i32>(r, x1) &*& [1/2]ref_end_token::<i32>(x1, p) &*& [1/2]ref_initialized::<i32>(x1) &*&
        [1/2]ref_end_token::<i32>(r, x2);
    ens [1/2]ref_end_token::<i32>(r, p) &*& [1/2]ref_end_token::<i32>(r, x2);
{
    ref_end_token_trans_to(r, x1, p);
}

lem partial_chain_keeps_two_parents(child: *i32, parent: *i32, grandparent: *i32)
    req [1/4]ref_end_token::<i32>(child, parent) &*& [1/2]ref_end_token::<i32>(parent, grandparent) &*& [1/2]ref_initialized::<i32>(parent);
    ens true;
{
    close eighth_token(child, parent);
    ref_end_token_trans(child);
    open eighth_token(child, parent);
    assert parent == grandparent; //~ should_fail
}

@*/

unsafe fn chain_then_end(g: *mut i32) -> i32
//@ req *g |-> ?v;
//@ ens *g |-> 5 &*& result == v;
{
    //@ let p = precreate_ref(g);
    //@ init_ref_readonly(p, 1/2);
    let rp = &*g;
    let rawp = rp as *const i32;
    //@ let a = precreate_ref(rawp);
    //@ init_ref_readonly(a, 1/2);
    let ra = &*rawp;
    let result = *ra;
    //@ ref_end_token_trans(a);
    //@ end_ref(a);
    //@ end_ref(p);
    *g = 5;
    result
}

unsafe fn partial_chain(g: *mut i32) -> i32
//@ req *g |-> ?v;
//@ ens *g |-> 5 &*& result == v;
{
    //@ let p = precreate_ref(g);
    //@ init_ref_readonly(p, 1/2);
    let rp = &*g;
    let rawp = rp as *const i32;
    //@ let a = precreate_ref(rawp);
    //@ init_ref_readonly(a, 1/2);
    let ra = &*rawp;
    let result = *ra;
    //@ close eighth_token(a, p);
    //@ ref_end_token_trans(a);
    //@ open eighth_token(a, p);
    //@ end_ref_to(a, p);
    //@ end_ref_to(a, g);
    //@ end_ref(p);
    *g = 5;
    result
}

unsafe fn read_middle_after_parent_write(g: *mut i32)
//@ req *g |-> ?v;
//@ ens true;
{
    //@ let p = precreate_ref(g);
    //@ init_ref_readonly(p, 1/2);
    let rp = &*g;
    let rawp = rp as *const i32;
    //@ let a = precreate_ref(rawp);
    //@ init_ref_readonly(a, 1/2);
    let _ra = &*rawp;
    //@ close eighth_token(a, p);
    //@ ref_end_token_trans(a);
    //@ open eighth_token(a, p);
    //@ end_ref_to(a, p);
    //@ end_ref_to(a, g);
    //@ end_ref(p);
    *g = 5;
    let _v = *rp; //~ should_fail
}

unsafe fn reborrow_middle_after_parent_write(g: *mut i32)
//@ req *g |-> ?v;
//@ ens true;
{
    //@ let p = precreate_ref(g);
    //@ init_ref_readonly(p, 1/2);
    let rp = &*g;
    let rawp = rp as *const i32;
    //@ let a = precreate_ref(rawp);
    //@ init_ref_readonly(a, 1/2);
    let _ra = &*rawp;
    //@ ref_end_token_trans(a);
    //@ end_ref(a);
    //@ end_ref(p);
    *g = 5;
    let _r2 = &*rp; //~ should_fail
}

fn main() {}
