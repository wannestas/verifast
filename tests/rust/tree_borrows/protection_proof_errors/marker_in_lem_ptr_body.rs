// A lemma-pointer chunk's body runs in the clause, so a marker there is not at the top level (its proof keeps its
// own call(), protection_proof_nested_call.rs).
/*@
pred True_() = true;
@*/

unsafe fn set(x: &mut i32)
//@ req *x |-> _;
//@ ens *x |-> 5;
//@ protection_proof { produce_lem_ptr_chunk implies(True_, True_)() { open True_(); close True_(); } { protect(); } call(); unprotect(); }
{
    *x = 5;
}

fn main() {}
