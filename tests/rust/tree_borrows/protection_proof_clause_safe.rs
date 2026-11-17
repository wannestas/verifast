// A protection_proof clause after a safety_proof clause. Without -check_protectors it is dropped, and the safety proof still runs:
// the spec holds x's memory in a predicate, so the prototype check needs the safety proof's close and open.
/*@
pred Cell(p: *mut i32, v: i32) = *p |-> v;
@*/

fn get(x: &mut i32) -> i32
//@ req thread_token(?t) &*& Cell(x, ?v);
//@ ens thread_token(t) &*& Cell(x, v) &*& result == v;
//@ safety_proof { close Cell(x, _); let r = call(); open Cell(x, _); }
//@ protection_proof { open Cell(x, _); protect(); close Cell(x, _); let r = call(); open Cell(x, _); unprotect(); close Cell(x, _); }
{
    //@ open Cell(x, _);
    let r = *x;
    //@ close Cell(x, r);
    r
}

fn main() {}
