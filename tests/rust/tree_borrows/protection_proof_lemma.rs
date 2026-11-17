// A protection_proof clause is allowed only on a Rust function with a body.
/*@
lem foo()
    req true;
    ens true;
    protection_proof { protect(); call(); unprotect(); }
{
}
@*/

fn main() {}
