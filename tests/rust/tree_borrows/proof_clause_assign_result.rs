// Assigning to the result in a proof clause doesn't change what the prototype's postcondition is consumed at.
// The failing function comes last: an error in the prototype check ends the file's verification.
struct S { v: i32 }
/*@
pred <S>.own(t, s) = s.v == 5;
@*/
fn make5() -> S
//@ req thread_token(?t);
//@ ens thread_token(t) &*& result.v == 5;
//@ safety_proof { let r = call(); close S_own(_t, r); }
{
    S { v: 5 }
}

fn main() {}

fn make4() -> S //~ should_fail
//@ req thread_token(?t);
//@ ens thread_token(t) &*& result.v == 4;
//@ safety_proof { let r = call(); r = S { v: 5 }; close S_own(_t, r); }
{
    S { v: 4 }
}
