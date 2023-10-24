use std::fmt::Debug;

#[repr(C)]
struct VTable<const N: usize> {
    drop_fn: fn(*mut ()) -> (),
    size: usize,
    align: usize,
    funcs: [fn(*const ()) -> usize; N],
}

#[repr(C)]
struct TraitObject<const N: usize> {
    data_ptr: *const (),
    vtable_ptr: *const VTable<N>,
}

const METHODS: usize = 2;

trait Trait {
    fn foo(&self) -> usize;
    fn bar(&self) -> usize;
}

#[derive(Debug)]
struct S {
    env: usize,
}

impl Trait for S {
    fn foo(&self) -> usize {
        self.env + 11
    }

    fn bar(&self) -> usize {
        self.env + 12
    }
}

unsafe fn run() {
    let s = S { env: 5 };
    let x: &dyn Trait = &s;

    let wide_ptr: *const TraitObject<METHODS> = &x as *const _ as *const _;
    let wide_ptr = wide_ptr.as_ref().unwrap();

    println!("  data = {:p}", wide_ptr.data_ptr);
    let data = wide_ptr.data_ptr as *const S;
    println!(" *data = {:?}", (*data).env);
    println!("vtable = {:p}", wide_ptr.vtable_ptr);
    let vtable = wide_ptr.vtable_ptr.as_ref().unwrap();
    println!("  drop = {:p}", vtable.drop_fn as *const ());
    println!("  size = {}", vtable.size);
    println!(" align = {}", vtable.align);

    println!();
    println!("Funcs:");
    for f in vtable.funcs {
        println!("{:p}", f as *const ());
    }

    println!();
    println!("Calls:");
    for f in vtable.funcs {
        let y = (f)(wide_ptr.data_ptr);
        println!("{}", y);
    }
}

fn main() {
    unsafe { run() };
}
