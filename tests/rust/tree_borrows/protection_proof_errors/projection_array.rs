// A projection under an array pointee is not yet supported; decode_ty drops an array's element type, so the
// translated type is checked.
trait Tr {
    type A;
}

unsafe fn touch<T: Tr>(x: &mut [T::A; 2])
//@ req *x |-> ?v;
//@ ens *x |-> v;
//@ protection_proof { protect(); call(); unprotect(); }
{
}

fn main() {}
