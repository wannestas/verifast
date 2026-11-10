unsafe fn option_payload_ri(x: *mut Option<i32>)
//@ req *x |-> std::option::Option::Some(?v);
//@ ens true;
{
    //@ let p = precreate_ref(x);
    //@ init_ref_readonly(p, 1/2);
    let r = &*x;
    //@ std::option::open_points_to_Option_Some(p);
    //@ std::option::end_ref_Option_Some(p); //~ should_fail
}

unsafe fn box_contents_ri(x: *mut Box<i32>)
//@ req *x |-> ?b &*& std::boxed::Box_in::<i32, std::alloc::Global>(?t, b, ?a, ?v);
//@ ens true;
{
    //@ let p = precreate_ref(x);
    //@ init_ref_readonly(p, 1/2);
    let r = &*x;
    //@ let cp = std::boxed::Box_separate_contents(p);
    //@ std::boxed::end_ref_Box_in(p); //~ should_fail
}

fn main() {}
