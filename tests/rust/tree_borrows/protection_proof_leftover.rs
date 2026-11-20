// consume's postcondition doesn't give x's memory back, so unprotect() can neither chain nor end the fresh pointer:
// its ref_end_token and ref_initialized reach the leak check, named after the parameter. A leak in the clause is
// allowed. The failing function comes last: an error in the obligation ends the file's verification.
unsafe fn consume_leak(x: &mut i32)
//@ req *x |-> _;
//@ ens true;
//@ protection_proof { protect(); let x1 = x; call(); unprotect(); leak ref_end_token(x1, x) &*& ref_initialized(x1); }
{
    //@ leak *x |-> _;
}

fn main() {}

unsafe fn consume(x: &mut i32) //~ should_fail
//@ req *x |-> _;
//@ ens true;
//@ protection_proof { protect(); call(); unprotect(); }
{
    //@ leak *x |-> _;
}
