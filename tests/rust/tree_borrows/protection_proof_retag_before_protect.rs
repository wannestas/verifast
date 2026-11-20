// The code before protect() retags x itself, chains half of the new pointer's ref_end_token to x's parent, and locks
// the rest, so that a callee given the new pointer could end half of its own borrow and read the parent mid-call
// (Miri twin miri/obligation_half_borrow_parent_read.rs: UB). But protect() always protects the argument itself, whose
// memory has moved to the new pointer.
/*@

pred Foo(p: *i32, y: *i32) = *p |-> _ &*& [1/2]ref_end_token::<i32>(p, y) &*& ref_initialized::<i32>(p);

lem chain_half(r: *i32)
    req [1/2]ref_end_token::<i32>(r, ?x) &*& [1/2]ref_end_token::<i32>(x, ?p) &*& [1/2]ref_initialized::<i32>(x);
    ens [1/2]ref_end_token::<i32>(r, p);
{ ref_end_token_trans(r); }

lem lock_half_to(r: *i32, x: *i32)
    req [1/2]ref_end_token::<i32>(r, x);
    ens ref_protector::<i32>(r, x, 1/2);
{ lock_ref(r); }

lem end_half(p: *i32)
    req [1/2]ref_end_token::<i32>(p, ?x) &*& [1/2](*p |-> ?v) &*& [1/2]ref_initialized::<i32>(p);
    ens [1/2](*x |-> v);
{ end_ref(p); }

@*/

unsafe fn callee(p: &mut i32, yp: *const i32)
//@ req Foo(p, yp as *i32);
//@ ens [1/2](*p |-> ?w) &*& [1/2]ref_initialized(p) &*& [1/2](*(yp as *i32) |-> w);
/*@
protection_proof {
    open Foo(p, yp as *i32);
    let r0 = retag_ref_mut(p);
    chain_half(r0);
    lock_half_to(r0, p);
    close Foo(r0, yp as *i32);
    protect(); //~ should_fail
    call();
    unprotect();
}
@*/
{
    //@ open Foo(p, yp as *i32);
    *p = 1;
    //@ end_half(p);
    let v = *yp;
    //@ assert v == 1;
}

fn main() {}
