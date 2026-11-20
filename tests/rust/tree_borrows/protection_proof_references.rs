// protection_proof clauses checked under -check_protectors, for &mut T parameters and results, and &T with T a scalar,
// reference or type parameter.

unsafe fn set(x: &mut i32)
//@ req *x |-> _;
//@ ens *x |-> 5;
//@ protection_proof { protect(); call(); unprotect(); }
{
    *x = 5;
}

// After protect(), the parameter's name means the fresh pointer, which has ref_initialized; after unprotect(), the
// argument again.
unsafe fn set_names(x: &mut i32)
//@ req *x |-> _;
//@ ens *x |-> 5;
//@ protection_proof { protect(); assert ref_initialized(x); call(); assert ref_initialized(x) &*& *x |-> 5; unprotect(); assert *x |-> 5; }
{
    *x = 5;
}

unsafe fn get(x: &i32) -> i32
//@ req [?f](*x |-> ?v);
//@ ens [f](*x |-> v) &*& result == v;
//@ protection_proof { protect(); let r = call(); unprotect(); }
{
    *x
}

unsafe fn mixed(n: i32, p: *mut i32, x: &mut i32, y: &i32)
//@ req *p |-> _ &*& *x |-> _ &*& [?f](*y |-> ?w);
//@ ens *p |-> n &*& *x |-> w &*& [f](*y |-> w);
//@ protection_proof { protect(); call(); unprotect(); }
{
    *p = n;
    *x = *y;
}

// A shared result: unprotect() chains the result's ref_end_token through the fresh pointer to x; after the retag, the
// retagged result's token is chained through the result to x.
unsafe fn id_shared<'a>(x: &'a i32) -> &'a i32
//@ req [?f](*x |-> ?v);
//@ ens [?g](*result |-> v) &*& [g]ref_end_token(result, x) &*& [g]ref_initialized(result) &*& 0 < g &*& g < f &*& [f - g](*x |-> v);
//@ protection_proof { protect(); let r = call(); unprotect(); }
{
    let raw = x as *const i32;
    //@ let c = precreate_ref(raw);
    //@ init_ref_readonly(c, 1/2);
    &*raw
}

// A mutable result. VeriFast doesn't track the creation of &mut references yet, so a body can't produce the result's
// ref_end_token; this body assumes its postcondition. The obligation depends only on the specification.
unsafe fn id(x: &mut i32) -> &mut i32
//@ req *x |-> ?v;
//@ ens *result |-> v &*& ref_end_token(result, x) &*& ref_initialized(result);
//@ protection_proof { protect(); let r = call(); unprotect(); }
{
    //@ assume(false);
    x
}

// The same with the retag placed: after it, r2 names the retagged result, whose token now goes to x.
unsafe fn id_retag(x: &mut i32) -> &mut i32
//@ req *x |-> ?v;
//@ ens *result |-> v &*& ref_end_token(result, x) &*& ref_initialized(result);
//@ protection_proof { protect(); let r = call(); unprotect(); let r2 = retag(); assert ref_end_token(r2, x) &*& *r2 |-> v; }
{
    //@ assume(false);
    x
}

fn main() {}
