// The protection_proof clause comes after the safety_proof clause. The parse error is reported at the body.
fn get(x: &mut i32) -> i32
//@ req thread_token(?t) &*& *x |-> ?v;
//@ ens thread_token(t) &*& *x |-> v &*& result == v;
//@ protection_proof { protect(); let r = call(); unprotect(); }
//@ safety_proof { let r = call(); }
{ //~ should_fail
    *x
}

fn main() {}
