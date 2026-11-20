// A by-value tuple containing a reference is not yet supported. Box parameters take the same path: under the flag,
// a type outside the supported ones is an error, never a skip.
unsafe fn set_first(p: (&mut i32, i32))
//@ req *p.0 |-> _;
//@ ens *p.0 |-> p.1;
//@ protection_proof { protect(); call(); unprotect(); }
{
    *p.0 = p.1;
}

fn main() {}
