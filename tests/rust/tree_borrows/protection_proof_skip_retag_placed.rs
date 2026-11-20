// As protection_proof_skip_retag.rs, with retag() placed and the code after it assigning the pre-retag result to r2:
// the postcondition is still consumed at the retagged result, which the obligation tracks itself.
unsafe fn id_alias(x: &mut i32, slot: *mut *mut i32) -> &mut i32
//@ req *x |-> ?v &*& *slot |-> _;
/*@
ens *result |-> v &*&
    *slot |-> result; //~ should_fail
@*/
//@ protection_proof { protect(); let x1 = x; let r = call(); unprotect(); leak ref_end_token(x1, x) &*& ref_initialized(x1); let r2 = retag(); r2 = r; }
{
    *slot = &raw mut *x;
    x
}

fn main() {}
