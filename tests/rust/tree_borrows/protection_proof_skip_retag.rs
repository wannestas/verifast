// id_alias returns x and keeps a raw alias of it in slot. The obligation retags the result, so the postcondition's
// `*slot |-> result` fails at the retagged result (Miri twin miri/obligation_alias_result.rs: UB). Without a placed
// retag(), the obligation retags at the end. The clause leaks the fresh pointer's ref_end_token and ref_initialized,
// whose memory went to the result, so that only the retag stands between the specification and success.
unsafe fn id_alias(x: &mut i32, slot: *mut *mut i32) -> &mut i32
//@ req *x |-> ?v &*& *slot |-> _;
/*@
ens *result |-> v &*&
    *slot |-> result; //~ should_fail
@*/
//@ protection_proof { protect(); let x1 = x; let r = call(); unprotect(); leak ref_end_token(x1, x) &*& ref_initialized(x1); }
{
    *slot = &raw mut *x;
    x
}

fn main() {}
