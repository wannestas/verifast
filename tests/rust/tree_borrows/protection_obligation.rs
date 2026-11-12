use std::cell::UnsafeCell;

struct Pair {
    a: i32,
    b: i32,
}

struct S {
    a: i32,
    c: UnsafeCell<i32>,
}

unsafe fn set(x: &mut i32)
//@ req *x |-> _;
//@ ens *x |-> 5;
{
    *x = 5;
}

unsafe fn get(x: &i32) -> i32
//@ req [?f](*x |-> ?v);
//@ ens [f](*x |-> v) &*& result == v;
{
    *x
}

unsafe fn first(x: &Pair) -> i32
//@ req [?f](*x).a |-> ?a &*& [f](*x).b |-> ?b &*& [f]struct_Pair_padding(x);
//@ ens [f](*x).a |-> a &*& [f](*x).b |-> b &*& [f]struct_Pair_padding(x) &*& result == a;
{
    (*x).a
}

unsafe fn bump(x: &S)
//@ req [?f](*x).a |-> ?a &*& [f]struct_S_padding(x) &*& (*ref_origin(x)).c |-> _ &*& pointer_within_limits(&(*x).c) == true;
//@ ens [f](*x).a |-> a &*& [f]struct_S_padding(x) &*& (*ref_origin(x)).c |-> a;
{
    *(*x).c.get() = (*x).a;
}

unsafe fn id_shared<'a>(x: &'a i32) -> &'a i32
//@ req [?f](*x |-> ?v);
//@ ens [?g](*result |-> v) &*& [g]ref_end_token(result, x) &*& [g]ref_initialized(result) &*& 0 < g &*& g < f &*& [f - g](*x |-> v);
{
    let raw = x as *const i32;
    //@ let c = precreate_ref(raw);
    //@ init_ref_readonly(c, 1/2);
    &*raw
}

/*@

// Each f_spec is f's contract, which f's body satisfies at every argument.
// The stash stands for framework state that no user code can reach: only the obligations call it.

lem framework_stash<T>(r: *T, x: *T, q: real)
    req ref_protector::<T>(r, x, q);
    ens true;
{ assume(false); }

lem framework_unstash<T>(r: *T, x: *T, q: real)
    req true;
    ens ref_protector::<T>(r, x, q);
{ assume(false); }

lem framework_take_back<T>(r: *T, x: *T, q: real)
    req ref_protector::<T>(r, x, q);
    ens ref_protector::<T>(r, x, q);
{}

lem framework_stash_padding<T>(r: *T, x: *T, q: real)
    req ref_padding_protector::<T>(r, x, q);
    ens true;
{ assume(false); }

lem framework_unstash_padding<T>(r: *T, x: *T, q: real)
    req true;
    ens ref_padding_protector::<T>(r, x, q);
{ assume(false); }

lem framework_take_back_padding<T>(r: *T, x: *T, q: real)
    req ref_padding_protector::<T>(r, x, q);
    ens ref_padding_protector::<T>(r, x, q);
{}

lem set_spec(x: *i32)
    req *x |-> _;
    ens *x |-> 5;
{ assume(false); }

lem set_obligation(x: *i32)
    req *x |-> _ &*& pointer_within_limits(x) == true;
    ens *x |-> 5;
{
    let r = protect_ref_mut(x);
    framework_stash(r, x, 1);
    assert pointer_within_limits(r) == true;
    set_spec(r);
    framework_unstash(r, x, 1);
    framework_take_back(r, x, 1);
    unprotect_ref(r);
    end_ref(r);
}

lem get_spec(x: *i32) -> i32
    req [?f](*x |-> ?v);
    ens [f](*x |-> v) &*& result == v;
{ assume(false); }

lem get_obligation(x: *i32) -> i32
    req [?f](*x |-> ?v) &*& pointer_within_limits(x) == true;
    ens [f](*x |-> v) &*& result == v;
{
    let r = protect_ref(x, 1/2);
    framework_stash(r, x, f/2);
    assert pointer_within_limits(r) == true;
    let res = get_spec(r);
    framework_unstash(r, x, f/2);
    framework_take_back(r, x, f/2);
    unprotect_ref(r);
    end_ref(r);
    return res;
}

lem first_spec(x: *Pair) -> i32
    req [?f](*x).a |-> ?a &*& [f](*x).b |-> ?b &*& [f]struct_Pair_padding(x);
    ens [f](*x).a |-> a &*& [f](*x).b |-> b &*& [f]struct_Pair_padding(x) &*& result == a;
{ assume(false); }

lem first_obligation(x: *Pair) -> i32
    req [?f](*x).a |-> ?a &*& [f](*x).b |-> ?b &*& [f]struct_Pair_padding(x) &*& pointer_within_limits(x) == true;
    ens [f](*x).a |-> a &*& [f](*x).b |-> b &*& [f]struct_Pair_padding(x) &*& result == a;
{
    close_points_to(x, f);
    let r = protect_ref(x, 1/2);
    framework_stash(r, x, f/2);
    open_points_to(r);
    assert pointer_within_limits(r) == true;
    let res = first_spec(r);
    close_points_to(r, f/2);
    framework_unstash(r, x, f/2);
    framework_take_back(r, x, f/2);
    unprotect_ref(r);
    end_ref(r);
    open_points_to(x);
    return res;
}

lem bump_spec(x: *S)
    req [?f](*x).a |-> ?a &*& [f]struct_S_padding(x) &*& (*ref_origin(x)).c |-> _ &*& pointer_within_limits(&(*x).c) == true;
    ens [f](*x).a |-> a &*& [f]struct_S_padding(x) &*& (*ref_origin(x)).c |-> a;
{ assume(false); }

lem bump_obligation(x: *S)
    req [?f](*x).a |-> ?a &*& [f]struct_S_padding(x) &*& (*ref_origin(x)).c |-> _ &*& pointer_within_limits(&(*x).c) == true &*&
        pointer_within_limits(x) == true;
    ens [f](*x).a |-> a &*& [f]struct_S_padding(x) &*& (*ref_origin(x)).c |-> a;
{
    let p = precreate_ref(x);
    open_ref_init_perm_S(p);
    init_ref_readonly(&(*p).a, 1/2);
    lock_ref(&(*p).a);
    framework_stash(&(*p).a, &(*x).a, f/2);
    init_ref_padding_S(p, 1/2);
    lock_ref_padding(p);
    framework_stash_padding(p, x, f/2);
    leak ref_precreated_token(x, p);
    ref_origin_min_addr((p as pointer).provenance);
    ref_origin_max_addr((p as pointer).provenance);
    assert pointer_within_limits(p) == true;
    bump_spec(p);
    framework_unstash(&(*p).a, &(*x).a, f/2);
    framework_unstash_padding(p, x, f/2);
    framework_take_back(&(*p).a, &(*x).a, f/2);
    framework_take_back_padding(p, x, f/2);
    unprotect_ref(&(*p).a);
    unprotect_ref_padding(p);
    end_ref(&(*p).a);
    end_ref_padding_S(p);
}

lem id_shared_spec(x: *i32) -> *i32
    req [?f](*x |-> ?v);
    ens [?g](*result |-> v) &*& [g]ref_end_token(result, x) &*& [g]ref_initialized(result) &*& 0 < g &*& g < f &*& [f - g](*x |-> v);
{ assume(false); }

lem id_shared_obligation(x: *i32) -> *i32
    req [?f](*x |-> ?v) &*& pointer_within_limits(x) == true;
    ens [?g](*result |-> v) &*& [g]ref_end_token(result, x) &*& [g]ref_initialized(result) &*& 0 < g &*& g < f &*& [f - g](*x |-> v);
{
    let r = protect_ref(x, 1/2);
    framework_stash(r, x, f/2);
    assert pointer_within_limits(r) == true;
    let res = id_shared_spec(r);
    assume(pointer_within_limits(res) == true);
    framework_unstash(r, x, f/2);
    framework_take_back(r, x, f/2);
    unprotect_ref(r);
    ref_end_token_trans(res);
    end_ref(r);
    let res2 = retag_ref(res, 1/2);
    ref_end_token_trans(res2);
    end_ref(res);
    assert pointer_within_limits(res2) == true;
    return res2;
}

@*/

fn main() {}
