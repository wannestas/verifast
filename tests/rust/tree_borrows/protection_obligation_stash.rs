unsafe fn read_parent(x: &mut i32, p: *const i32) -> i32
//@ req *x |-> _ &*& [1/2]ref_end_token(x, p as *i32) &*& ref_initialized(x);
//@ ens [1/2](*x |-> ?w) &*& [1/2]ref_initialized(x) &*& [1/2](*(p as *i32) |-> w) &*& result == w;
{
    *x = 1;
    //@ end_ref(x);
    *p
}

/*@

lem read_parent_spec(x: *i32, p: *i32) -> i32
    req *x |-> _ &*& [1/2]ref_end_token(x, p) &*& ref_initialized(x);
    ens [1/2](*x |-> ?w) &*& [1/2]ref_initialized(x) &*& [1/2](*p |-> w) &*& result == w;
{ assume(false); }

// Stands for the framework's stash, which no user code can reach: only the obligation calls these.
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

lem chain_half(r: *i32)
    req [1/2]ref_end_token::<i32>(r, ?x) &*& [1/2]ref_end_token::<i32>(x, ?p) &*& [1/2]ref_initialized::<i32>(x);
    ens [1/2]ref_end_token::<i32>(r, p);
{ ref_end_token_trans(r); }

lem lock_half_to(r: *i32, x: *i32)
    req [1/2]ref_end_token::<i32>(r, x);
    ens ref_protector::<i32>(r, x, 1/2);
{ lock_ref(r); }

lem g2_without_guard(x: *i32, p: *i32) -> i32
    req *x |-> _ &*& [1/2]ref_end_token(x, p) &*& ref_initialized(x) &*& pointer_within_limits(x) == true &*& pointer_within_limits(p) == true;
    ens [1/2](*x |-> ?w) &*& [1/2]ref_initialized(x) &*& [1/2](*p |-> w) &*& result == w;
{
    let r = protect_ref_mut(x);
    unprotect_ref(r); chain_half(r); lock_half_to(r, x);
    let res = read_parent_spec(r, p);
    unprotect_ref(r); end_ref(r);
    return res;
}

lem g2_with_stash(x: *i32, p: *i32) -> i32
    req *x |-> _ &*& [1/2]ref_end_token(x, p) &*& ref_initialized(x) &*& pointer_within_limits(x) == true &*& pointer_within_limits(p) == true;
    ens [1/2](*x |-> ?w) &*& [1/2]ref_initialized(x) &*& [1/2](*p |-> w) &*& result == w;
{
    let r = protect_ref_mut(x); framework_stash(r, x, 1);
    unprotect_ref(r); //~ should_fail
    chain_half(r); lock_half_to(r, x);
    let res = read_parent_spec(r, p);
    framework_unstash(r, x, 1); framework_take_back(r, x, 1); unprotect_ref(r); end_ref(r);
    return res;
}

lem g2_with_takeback(x: *i32, p: *i32) -> i32
    req *x |-> _ &*& [1/2]ref_end_token(x, p) &*& ref_initialized(x) &*& pointer_within_limits(x) == true &*& pointer_within_limits(p) == true;
    ens [1/2](*x |-> ?w) &*& [1/2]ref_initialized(x) &*& [1/2](*p |-> w) &*& result == w;
{
    let r = protect_ref_mut(x);
    unprotect_ref(r); chain_half(r); lock_half_to(r, x);
    let res = read_parent_spec(r, p);
    framework_take_back(r, x, 1); //~ should_fail
    unprotect_ref(r); end_ref(r);
    return res;
}

@*/

fn main() {}
