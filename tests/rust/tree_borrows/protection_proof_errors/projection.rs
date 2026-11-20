// A projection pointee is not yet supported.
trait Tr {
    type A;
}

unsafe fn touch<T: Tr>(x: &mut T::A)
//@ req *x |-> ?v;
//@ ens *x |-> v;
//@ protection_proof { protect(); call(); unprotect(); }
{
}

fn main() {}
