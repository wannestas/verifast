// Assigning to a parameter in a proof clause doesn't change what `call()` consumes the precondition at.
fn get(x: &mut i32) -> i32
//@ req thread_token(?t) &*& *x |-> ?v;
//@ ens thread_token(t) &*& *x |-> v &*& result == v;
//@ safety_proof { let x0 = x; x = 0 as *mut i32; assert x != x0; let r = call(); }
{
    *x
}

fn main() {}
