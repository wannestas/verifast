/*@

pred quarter_token(p: *i32, x: *i32) = [1/4]ref_end_token::<i32>(p, x);
pred half_protector(a: *i32, x: *i32, q: real) = [1/2]ref_protector::<i32>(a, x, q);

lem mut_round_trip(x: *i32)
    req *x |-> ?v;
    ens *x |-> v;
{
    let r = protect_ref_mut(x);
    assert (r as pointer).address == (x as pointer).address &*& ref_origin(r) == r;
    unprotect_ref(r);
    end_ref(r);
}

lem shared_round_trip(x: *i32)
    req [?f](*x |-> ?v) &*& pointer_within_limits(x) == true;
    ens [f](*x |-> v);
{
    let r = protect_ref(x, 1/2);
    assert ref_origin(r) == ref_origin(x) &*& pointer_within_limits(r) == true;
    unprotect_ref(r);
    end_ref(r);
}

lem retag_round_trips(x: *i32)
    req *x |-> ?v;
    ens *x |-> v;
{
    let r = retag_ref_mut(x);
    let s = retag_ref(r, 1/2);
    end_ref(s);
    end_ref(r);
}

lem lock_part_of_a_token(x: *i32)
    req *x |-> ?v;
    ens *x |-> v;
{
    let p = precreate_ref(x);
    init_ref_readonly(p, 1/2);
    leak ref_precreated_token(x, p);
    close quarter_token(p, x);
    lock_ref(p);
    assert ref_protector::<i32>(p, x, 1/4);
    unprotect_ref(p);
    open quarter_token(p, x);
    end_ref(p);
}

lem end_while_protected(x: *i32)
    req *x |-> ?v;
    ens true;
{
    let r = protect_ref_mut(x);
    end_ref(r); //~ should_fail
}

lem chain_from_a_locked_token(child: *i32, parent: *i32, grandparent: *i32)
    req [1/2]ref_end_token::<i32>(child, parent) &*& [1/2]ref_end_token::<i32>(parent, grandparent) &*& [1/2]ref_initialized::<i32>(parent);
    ens true;
{
    lock_ref(child);
    ref_end_token_trans(child); //~ should_fail
}

lem chain_through_a_locked_token(child: *i32, parent: *i32, grandparent: *i32)
    req [1/2]ref_end_token::<i32>(child, parent) &*& [1/2]ref_end_token::<i32>(parent, grandparent) &*& [1/2]ref_initialized::<i32>(parent);
    ens true;
{
    lock_ref(parent);
    ref_end_token_trans(child); //~ should_fail
}

lem unprotect_half_a_protector(x: *i32)
    req *x |-> ?v;
    ens true;
{
    let r = protect_ref_mut(x);
    close half_protector(r, x, 1);
    unprotect_ref(r); //~ should_fail
}

lem whole_protector(a: *i32)
    req ref_protector::<i32>(a, ?x, ?q);
    ens ref_protector::<i32>(a, x, q);
{}

lem protectors_with_two_parents_stay_apart(a: *i32, p: *i32, g: *i32)
    req [1/2]ref_protector::<i32>(a, p, 1/2) &*& [1/2]ref_protector::<i32>(a, g, 1/2);
    ens true;
{
    whole_protector(a); //~ should_fail
}

lem padding_round_trip(x: *S)
    req *x |-> ?s;
    ens *x |-> s;
{
    open_points_to(x);
    let p = precreate_ref(x);
    open_ref_init_perm_S(p);
    init_ref_readonly(&(*p).a, 1/2);
    init_ref_readonly(&(*p).b, 1/2);
    init_ref_padding_S(p, 1/2);
    leak ref_precreated_token(x, p);
    lock_ref_padding(p);
    unprotect_ref_padding(p);
    end_ref_padding_S(p);
    end_ref(&(*p).a);
    end_ref(&(*p).b);
    close_points_to(x);
}

@*/

struct S {
    a: u8,
    b: u32,
}

unsafe fn borrow_after_protect_ref(x: *mut i32)
//@ req *x |-> ?v;
//@ ens true;
{
    //@ let r = protect_ref(x, 1/2);
    //@ unprotect_ref(r);
    let _y = &*x; //~ should_fail
}

fn main() {}
