// A shared reference to a struct is not yet supported.
struct Pair { a: i32, b: i32 }

unsafe fn first(x: &Pair) -> i32
//@ req [?f](*x).a |-> ?a &*& [f](*x).b |-> ?b &*& [f]struct_Pair_padding(x);
//@ ens [f](*x).a |-> a &*& [f](*x).b |-> b &*& [f]struct_Pair_padding(x) &*& result == a;
//@ protection_proof { protect(); let r = call(); unprotect(); }
{
    (*x).a
}

fn main() {}
