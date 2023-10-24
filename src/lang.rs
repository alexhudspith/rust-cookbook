#[allow(dead_code)]
#[allow(unused_variables)]

#[cfg(test)]
mod tests {
    use ptr::NonNull;
    use std::{mem, ptr};
    use std::fmt::Debug;
    use std::ops::{Index, IndexMut, RangeTo};

    #[test]
    fn eq_deref() {
        let a1 = String::from("Hello");
        let a2 = a1.clone();
        assert_eq!(&a1, &a2);
    }

    #[test]
    fn eq_deref_mut() {
        let mut a1 = String::from("Hello");
        let mut a2 = a1.clone();
        assert_eq!(&mut a1, &mut a2);
    }

    #[test]
    fn eq_no_deref_ptr() {
        let a1 = String::from("Hello");
        let a2 = a1.clone();
        assert_ne!(&a1 as *const _, &a2 as *const _);
    }

    #[test]
    fn eq_no_deref_ptr_mut() {
        let mut a1 = String::from("Hello");
        let mut a2 = a1.clone();
        assert_ne!(&mut a1 as *mut _, &mut a2 as *mut _);
    }

    #[test]
    fn identical() {
        let a1 = String::from("Hello");
        let a2 = a1.clone();
        // Just (wide) pointer equality again, but worse
        assert!(ptr::eq(&a1, &a1));
        assert!(!ptr::eq(&a1, &a2));

        let a1 = &a1[..3];
        let a2 = &a1[..2];
        // Address equal but...
        assert!(ptr::addr_eq(a1, a2));
        // ...wide pointers unequal
        assert!(!ptr::eq(a1, a2));
    }

    #[test]
    fn eq_option() {
        assert_eq!(Some(&1), Some(&1));
        assert_eq!(None::<i32>, None::<i32>);
        assert_ne!(Some(&0), None);
    }

    #[test]
    fn cmp_option() {
        assert!(None < Some(0));
        assert!(Some(1) < Some(2));
    }

    #[test]
    fn cmp_result() {
        assert!(Ok(i32::MAX) < Err(i32::MIN));
    }

    #[test]
    fn eq_result() {
        assert_eq!(Ok::<_, i32>(1), Ok(1));
        assert_eq!(Err::<i32, _>(1), Err(1));
        assert_ne!(Ok(1), Err(1));
    }

    #[test]
    fn param_coercion() {
        // https://doc.rust-lang.org/reference/type-coercions.html

        fn f(_s1: &[i32]) {}
        fn g(_s1: &[i32], _s2: &[i32]) {}

        let arr = [1];
        let arr_ref_1: &[i32; 1] = &arr;
        #[allow(unused_variables)]
        let arr_ref_2: &&[i32; 1] = &arr_ref_1;
        let slice_ref_1: &[i32] = &arr;
        let slice_ref_2: &&[i32] = &slice_ref_1;
        let slice_ref_3: &&&[i32] = &slice_ref_2;

        // none
        f(slice_ref_1);
        // unsizing
        f(arr_ref_1);
        // deref
        f(slice_ref_2);
        // deref > deref
        f(slice_ref_3);
        // No: Needs deref > unsizing, but should work via transitivity
        // https://github.com/rust-lang/rust/issues/103259
        // f(arr_ref_2);

        // none
        g(slice_ref_1, slice_ref_1);
        // unsizing
        g(arr_ref_1, arr_ref_1);
        // deref
        g(slice_ref_2, slice_ref_2);
        // deref > deref
        g(slice_ref_3, slice_ref_3);
        // No: Needs deref > unsizing, but should work via transitivity
        // https://github.com/rust-lang/rust/issues/103259
        // g(arr_ref_2, arr_ref_2);
    }

    #[test]
    fn self_ref_coercion() {
        // https://doc.rust-lang.org/nomicon/dot-operator.html
        // https://doc.rust-lang.org/reference/expressions/method-call-expr.html

        trait T {
            fn f(&self) -> String;
        }

        impl T for [i32] {
            fn f(&self) -> String {
                "[i32]".to_string()
            }
        }

        impl T for i32 {
            fn f(&self) -> String {
                "i32".to_string()
            }
        }

        let unsize_me: [i32; 2] = [0; 2];
        // self: &Self = &[i32; 2] -> &[i32]
        assert_eq!(unsize_me.f(), "[i32]");

        let deref_unsize_me: &[i32; 2] = &[0; 2];
        // self: &Self = &[i32; 2] -> &&[i32; 2] -> &[i32]
        assert_eq!(deref_unsize_me.f(), "[i32]");

        let deref_me: &&[i32] = &(&[0; 2] as _);
        // self: &Self = &&[i32] -> &&&[i32] -> &[i32]
        assert_eq!(deref_me.f(), "[i32]");

        let deref_and_unsize_me: &&[i32; 2] = &&[0; 2];
        // self: &Self = &&[i32; 2] -> &&&[i32; 2] -> &[i32; 2] -> &[i32]
        assert_eq!(deref_and_unsize_me.f(), "[i32]");

        let ref_me = 0;
        assert_eq!(ref_me.f(), "i32");
    }


    #[test]
    fn self_value_coercion() {
        // https://doc.rust-lang.org/nomicon/dot-operator.html
        // https://doc.rust-lang.org/reference/expressions/method-call-expr.html

        trait T {
            fn f(self) -> String;
        }

        impl T for i32 {
            fn f(self) -> String {
                "i32".to_string()
            }
        }

        impl T for [i32; 2] {
            fn f(self) -> String {
                "[i32; 2]".to_string()
            }
        }

        let me: [i32; 2] = [0; 2];
        // self: Self = [i32; 2] (found) -> &[i32; 2]
        assert_eq!(me.f(), "[i32; 2]");

        let deref_me: &[i32; 2] = &[0; 2];
        // self: Self = &[i32; 2] -> &&[i32; 2] -> [i32; 2]
        assert_eq!(deref_me.f(), "[i32; 2]");

        let deref_deref_me: &&[i32; 2] = &&[0; 2];
        // self: Self = &&[i32; 2] -> &&&[i32; 2] -> &[i32; 2] -> [i32; 2]
        assert_eq!(deref_deref_me.f(), "[i32; 2]");

        let me = 0;
        assert_eq!(me.f(), "i32");
    }

    #[test]
    fn traity_goodness() {
        // Default implementations are allowed
        pub trait TraitA {
            fn method_impl_a(&self) -> &str;
            // Impossible, needs impl
            // fn method_struct_a(&self) -> &str;
            fn method_impl_struct_a(&self) -> &str;

            fn method_trait_a(&self) -> &str {
                "TraitA::method_trait_a"
            }

            fn method_trait_impl_a(&self) -> &str {
                unreachable!("TraitA::method_trait_impl_a")
            }

            fn method_trait_struct_a(&self) -> &str {
                "TraitA::method_trait_struct_a"
            }

            fn method_trait_impl_struct_a(&self) -> &str {
                unreachable!("TraitA::method_trait_impl_struct_a")
            }

            fn assoc_impl_a() -> &'static str;
            // Impossible, needs impl
            // fn assoc_struct_a() -> &'static str;
            fn assoc_impl_struct_a() -> &'static str;

            fn assoc_trait_a() -> &'static str {
                "TraitA::assoc_trait_a"
            }

            fn assoc_trait_impl_a() -> &'static str {
                unreachable!("TraitA::assoc_trait_impl_a")
            }

            fn assoc_trait_struct_a() -> &'static str {
                "TraitA::assoc_trait_struct_a"
            }

            fn assoc_trait_impl_struct_a() -> &'static str {
                unreachable!("TraitA::assoc_trait_impl_struct_a")
            }

            fn method_impl_ab(&self) -> &str;
            // Impossible, needs impl
            // fn method_struct_ab(&self) -> &str;
            fn method_impl_struct_ab(&self) -> &str;

            fn method_trait_ab(&self) -> &str {
                "TraitA::method_trait_ab"
            }

            fn method_trait_impl_ab(&self) -> &str {
                "TraitA::method_trait_impl_ab"
            }

            fn method_trait_struct_ab(&self) -> &str {
                "TraitA::method_trait_struct_ab"
            }

            fn method_trait_impl_struct_ab(&self) -> &str {
                unreachable!("TraitA::method_trait_impl_struct_ab")
            }

            fn assoc_impl_ab() -> &'static str;
            // Impossible, needs impl
            // fn assoc_struct_ab() -> &'static str;
            fn assoc_impl_struct_ab() -> &'static str;

            fn assoc_trait_ab() -> &'static str {
                "TraitA::assoc_trait_ab"
            }

            fn assoc_trait_impl_ab() -> &'static str {
                unreachable!("TraitA::assoc_trait_impl_ab")
            }

            fn assoc_trait_struct_ab() -> &'static str {
                "TraitA::assoc_trait_struct_ab"
            }

            fn assoc_trait_impl_struct_ab() -> &'static str {
                unreachable!("TraitA::assoc_trait_impl_struct_ab")
            }
        }

        pub trait TraitB {
            fn method_impl_ab(&self) -> &str;
            // Impossible, needs impl
            // fn method_struct_ab(&self) -> &str;
            fn method_impl_struct_ab(&self) -> &str;

            fn method_trait_ab(&self) -> &str {
                "TraitB::method_trait_ab"
            }

            fn method_trait_impl_ab(&self) -> &str {
                unreachable!("TraitB::method_trait_impl_ab")
            }

            fn method_trait_struct_ab(&self) -> &str {
                "TraitB::method_trait_struct_ab"
            }

            fn method_trait_impl_struct_ab(&self) -> &str {
                unreachable!("TraitB::method_trait_impl_struct_ab")
            }

            fn assoc_impl_ab() -> &'static str;
            // Impossible, needs impl
            // fn assoc_struct_ab() -> &'static str;
            fn assoc_impl_struct_ab() -> &'static str;

            fn assoc_trait_ab() -> &'static str {
                "TraitB::assoc_trait_ab"
            }

            fn assoc_trait_impl_ab() -> &'static str {
                unreachable!("TraitB::assoc_trait_impl_ab")
            }

            fn assoc_trait_struct_ab() -> &'static str {
                "TraitB::assoc_trait_struct_ab"
            }

            fn assoc_trait_impl_struct_ab() -> &'static str {
                unreachable!("TraitB::assoc_trait_impl_struct_ab")
            }
        }

        pub struct S {}

        // All functions must be members of TraitB, no extra 'helpers'
        impl TraitA for S {
            fn method_impl_a(&self) -> &str {
                "<S as TraitA>::method_impl_a"
            }

            fn method_impl_struct_a(&self) -> &str {
                "<S as TraitA>::method_impl_struct_a"
            }

            fn method_trait_impl_a(&self) -> &str {
                "<S as TraitA>::method_trait_impl_a"
            }

            fn method_trait_impl_struct_a(&self) -> &str {
                "<S as TraitA>::method_trait_impl_struct_a"
            }

            fn assoc_impl_a() -> &'static str {
                "<S as TraitA>::assoc_impl_a"
            }

            fn assoc_impl_struct_a() -> &'static str {
                "<S as TraitA>::assoc_impl_struct_a"
            }

            fn assoc_trait_impl_a() -> &'static str {
                "<S as TraitA>::assoc_trait_impl_a"
            }

            fn assoc_trait_impl_struct_a() -> &'static str {
                "<S as TraitA>::assoc_trait_impl_struct_a"
            }

            fn method_impl_ab(&self) -> &str {
                "<S as TraitA>::method_impl_ab"
            }

            fn method_impl_struct_ab(&self) -> &str {
                "<S as TraitA>::method_impl_struct_ab"
            }

            fn method_trait_impl_ab(&self) -> &str {
                "<S as TraitA>::method_trait_impl_ab"
            }

            fn method_trait_impl_struct_ab(&self) -> &str {
                "<S as TraitA>::method_trait_impl_struct_ab"
            }

            fn assoc_impl_ab() -> &'static str {
                "<S as TraitA>::assoc_impl_ab"
            }

            fn assoc_impl_struct_ab() -> &'static str {
                "<S as TraitA>::assoc_impl_struct_ab"
            }

            fn assoc_trait_impl_ab() -> &'static str {
                "<S as TraitA>::assoc_trait_impl_ab"
            }

            fn assoc_trait_impl_struct_ab() -> &'static str {
                "<S as TraitA>::assoc_trait_impl_struct_ab"
            }
        }

        impl TraitB for S {
            fn method_impl_ab(&self) -> &str {
                "<S as TraitB>::method_impl_ab"
            }

            fn method_impl_struct_ab(&self) -> &str {
                "<S as TraitB>::method_impl_struct_ab"
            }

            fn method_trait_impl_ab(&self) -> &str {
                "<S as TraitB>::method_trait_impl_ab"
            }

            fn method_trait_impl_struct_ab(&self) -> &str {
                "<S as TraitB>::method_trait_impl_struct_ab"
            }

            fn assoc_impl_ab() -> &'static str {
                "<S as TraitB>::assoc_impl_ab"
            }

            fn assoc_impl_struct_ab() -> &'static str {
                "<S as TraitB>::assoc_impl_struct_ab"
            }

            fn assoc_trait_impl_ab() -> &'static str {
                "<S as TraitB>::assoc_trait_impl_ab"
            }

            fn assoc_trait_impl_struct_ab() -> &'static str {
                "<S as TraitB>::assoc_trait_impl_struct_ab"
            }
        }

        impl S {
            fn method_struct(&self) -> &str {
                "S::method_struct"
            }

            fn assoc_struct() -> &'static str {
                "S::assoc_struct"
            }

            fn method_impl_struct_a(&self) -> &str {
                "S::method_impl_struct_a"
            }

            fn method_trait_struct_a(&self) -> &str {
                "S::method_trait_struct_a"
            }

            fn method_trait_impl_struct_a(&self) -> &str {
                "S::method_trait_impl_struct_a"
            }

            fn assoc_impl_struct_a() -> &'static str {
                "S::assoc_impl_struct_a"
            }

            fn assoc_trait_struct_a() -> &'static str {
                "S::assoc_trait_struct_a"
            }

            fn assoc_trait_impl_struct_a() -> &'static str {
                "S::assoc_trait_impl_struct_a"
            }

            fn method_impl_struct_ab(&self) -> &str {
                "S::method_impl_struct_ab"
            }

            fn method_trait_struct_ab(&self) -> &str {
                "S::method_trait_struct_ab"
            }

            fn method_trait_impl_struct_ab(&self) -> &str {
                "S::method_trait_impl_struct_ab"
            }

            fn assoc_impl_struct_ab() -> &'static str {
                "S::assoc_impl_struct_ab"
            }

            fn assoc_trait_struct_ab() -> &'static str {
                "S::assoc_trait_struct_ab"
            }

            fn assoc_trait_impl_struct_ab() -> &'static str {
                "S::assoc_trait_impl_struct_ab"
            }
        }

        let s = S {};

        // TraitA:
        // Assoc functions not available: must be accessed via S (direct or 'as')
        // Any method impls TraitA for S cannot be bypassed to use TraitA body (*)
        assert_eq!(TraitA::method_impl_a(&s), "<S as TraitA>::method_impl_a");
        assert_eq!(TraitA::method_impl_struct_a(&s), "<S as TraitA>::method_impl_struct_a");
        assert_eq!(TraitA::method_trait_a(&s), "TraitA::method_trait_a");
        // (*) TraitA and impl S for TraitA, so impl preferred
        assert_eq!(TraitA::method_trait_impl_a(&s), "<S as TraitA>::method_trait_impl_a");
        assert_eq!(TraitA::method_trait_struct_a(&s), "TraitA::method_trait_struct_a");
        assert_eq!(TraitA::method_trait_impl_struct_a(&s), "<S as TraitA>::method_trait_impl_struct_a");
        assert_eq!(TraitA::method_impl_ab(&s), "<S as TraitA>::method_impl_ab");
        assert_eq!(TraitA::method_impl_struct_ab(&s), "<S as TraitA>::method_impl_struct_ab");
        assert_eq!(TraitA::method_trait_ab(&s), "TraitA::method_trait_ab");
        // (*) TraitA and impl S for TraitA, so impl preferred
        assert_eq!(TraitA::method_trait_impl_ab(&s), "<S as TraitA>::method_trait_impl_ab");
        assert_eq!(TraitA::method_trait_struct_ab(&s), "TraitA::method_trait_struct_ab");
        // (*) TraitA and impl S for TraitA, so impl preferred
        assert_eq!(TraitA::method_trait_impl_struct_ab(&s), "<S as TraitA>::method_trait_impl_struct_ab");

        // TraitB:
        // Assoc functions not available: must be accessed via S (direct or 'as')
        // Any method impls TraitB for S cannot be bypassed to use TraitB body (*)
        assert_eq!(TraitB::method_impl_ab(&s), "<S as TraitB>::method_impl_ab");
        assert_eq!(TraitB::method_impl_struct_ab(&s), "<S as TraitB>::method_impl_struct_ab");
        assert_eq!(TraitB::method_trait_ab(&s), "TraitB::method_trait_ab");
        // (*) TraitB and impl S for TraitB, so impl preferred
        assert_eq!(TraitB::method_trait_impl_ab(&s), "<S as TraitB>::method_trait_impl_ab");
        assert_eq!(TraitB::method_trait_struct_ab(&s), "TraitB::method_trait_struct_ab");
        // (*) TraitB and impl S for TraitB, so impl preferred
        assert_eq!(TraitB::method_trait_impl_struct_ab(&s), "<S as TraitB>::method_trait_impl_struct_ab");

        // <S as TraitA> - always refers to impl TraitA for S, except if no impl exists when it refers to TraitA itself
        // struct S-only functions obviously inaccessible
        // No ambiguity
        assert_eq!(<S as TraitA>::method_impl_a(&s), "<S as TraitA>::method_impl_a");
        assert_eq!(<S as TraitA>::method_impl_struct_a(&s), "<S as TraitA>::method_impl_struct_a");
        assert_eq!(<S as TraitA>::method_trait_a(&s), "TraitA::method_trait_a");
        assert_eq!(<S as TraitA>::method_trait_impl_a(&s), "<S as TraitA>::method_trait_impl_a");
        assert_eq!(<S as TraitA>::method_trait_struct_a(&s), "TraitA::method_trait_struct_a");
        assert_eq!(<S as TraitA>::method_trait_impl_struct_a(&s), "<S as TraitA>::method_trait_impl_struct_a");
        assert_eq!(<S as TraitA>::assoc_impl_a(), "<S as TraitA>::assoc_impl_a");
        assert_eq!(<S as TraitA>::assoc_impl_struct_a(), "<S as TraitA>::assoc_impl_struct_a");
        assert_eq!(<S as TraitA>::assoc_trait_a(), "TraitA::assoc_trait_a");
        assert_eq!(<S as TraitA>::assoc_trait_impl_a(), "<S as TraitA>::assoc_trait_impl_a");
        assert_eq!(<S as TraitA>::assoc_trait_struct_a(), "TraitA::assoc_trait_struct_a");
        assert_eq!(<S as TraitA>::assoc_trait_impl_struct_a(), "<S as TraitA>::assoc_trait_impl_struct_a");
        assert_eq!(<S as TraitA>::method_impl_ab(&s), "<S as TraitA>::method_impl_ab");
        assert_eq!(<S as TraitA>::method_impl_struct_ab(&s), "<S as TraitA>::method_impl_struct_ab");
        assert_eq!(<S as TraitA>::method_trait_ab(&s), "TraitA::method_trait_ab");
        assert_eq!(<S as TraitA>::method_trait_impl_ab(&s), "<S as TraitA>::method_trait_impl_ab");
        assert_eq!(<S as TraitA>::method_trait_struct_ab(&s), "TraitA::method_trait_struct_ab");
        assert_eq!(<S as TraitA>::method_trait_impl_struct_ab(&s), "<S as TraitA>::method_trait_impl_struct_ab");
        assert_eq!(<S as TraitA>::assoc_impl_ab(), "<S as TraitA>::assoc_impl_ab");
        assert_eq!(<S as TraitA>::assoc_impl_struct_ab(), "<S as TraitA>::assoc_impl_struct_ab");
        assert_eq!(<S as TraitA>::assoc_trait_ab(), "TraitA::assoc_trait_ab");
        assert_eq!(<S as TraitA>::assoc_trait_impl_ab(), "<S as TraitA>::assoc_trait_impl_ab");
        assert_eq!(<S as TraitA>::assoc_trait_struct_ab(), "TraitA::assoc_trait_struct_ab");
        assert_eq!(<S as TraitA>::assoc_trait_impl_struct_ab(), "<S as TraitA>::assoc_trait_impl_struct_ab");

        // <S as TraitB> - always refers to impl TraitB for S, except if no impl exists when it refers to TraitB itself
        // TraitA's _a functions and struct S-only functions obviously inaccessible
        // No ambiguity
        assert_eq!(<S as TraitB>::method_impl_ab(&s), "<S as TraitB>::method_impl_ab");
        assert_eq!(<S as TraitB>::method_impl_struct_ab(&s), "<S as TraitB>::method_impl_struct_ab");
        assert_eq!(<S as TraitB>::method_trait_ab(&s), "TraitB::method_trait_ab");
        assert_eq!(<S as TraitB>::method_trait_impl_ab(&s), "<S as TraitB>::method_trait_impl_ab");
        assert_eq!(<S as TraitB>::method_trait_struct_ab(&s), "TraitB::method_trait_struct_ab");
        assert_eq!(<S as TraitB>::method_trait_impl_struct_ab(&s), "<S as TraitB>::method_trait_impl_struct_ab");
        assert_eq!(<S as TraitB>::assoc_impl_ab(), "<S as TraitB>::assoc_impl_ab");
        assert_eq!(<S as TraitB>::assoc_impl_struct_ab(), "<S as TraitB>::assoc_impl_struct_ab");
        assert_eq!(<S as TraitB>::assoc_trait_ab(), "TraitB::assoc_trait_ab");
        assert_eq!(<S as TraitB>::assoc_trait_impl_ab(), "<S as TraitB>::assoc_trait_impl_ab");
        assert_eq!(<S as TraitB>::assoc_trait_struct_ab(), "TraitB::assoc_trait_struct_ab");
        assert_eq!(<S as TraitB>::assoc_trait_impl_struct_ab(), "<S as TraitB>::assoc_trait_impl_struct_ab");

        // S
        // 8 non-'*_struct_ab' calls omitted for ambiguity (unrelated impls for both TraitA and TraitB)
        assert_eq!(S::method_struct(&s), "S::method_struct");
        assert_eq!(S::assoc_struct(), "S::assoc_struct");
        assert_eq!(S::method_impl_a(&s), "<S as TraitA>::method_impl_a");
        assert_eq!(S::method_impl_struct_a(&s), "S::method_impl_struct_a");
        assert_eq!(S::method_trait_a(&s), "TraitA::method_trait_a");
        assert_eq!(S::method_trait_impl_a(&s), "<S as TraitA>::method_trait_impl_a");
        assert_eq!(S::method_trait_struct_a(&s), "S::method_trait_struct_a");
        assert_eq!(S::method_trait_impl_struct_a(&s), "S::method_trait_impl_struct_a");
        assert_eq!(S::assoc_impl_a(), "<S as TraitA>::assoc_impl_a");
        assert_eq!(S::assoc_impl_struct_a(), "S::assoc_impl_struct_a");
        assert_eq!(S::assoc_trait_a(), "TraitA::assoc_trait_a");
        assert_eq!(S::assoc_trait_impl_a(), "<S as TraitA>::assoc_trait_impl_a");
        assert_eq!(S::assoc_trait_struct_a(), "S::assoc_trait_struct_a");
        assert_eq!(S::assoc_trait_impl_struct_a(), "S::assoc_trait_impl_struct_a");
        assert_eq!(S::method_impl_struct_ab(&s), "S::method_impl_struct_ab");
        assert_eq!(S::method_trait_struct_ab(&s), "S::method_trait_struct_ab");
        assert_eq!(S::method_trait_impl_struct_ab(&s), "S::method_trait_impl_struct_ab");
        assert_eq!(S::assoc_impl_struct_ab(), "S::assoc_impl_struct_ab");
        assert_eq!(S::assoc_trait_struct_ab(), "S::assoc_trait_struct_ab");
        assert_eq!(S::assoc_trait_impl_struct_ab(), "S::assoc_trait_impl_struct_ab");

        // s
        assert_eq!(s.method_struct(), "S::method_struct");
        assert_eq!(s.method_impl_a(), "<S as TraitA>::method_impl_a");
        assert_eq!(s.method_impl_struct_a(), "S::method_impl_struct_a");
        assert_eq!(s.method_trait_a(), "TraitA::method_trait_a");
        assert_eq!(s.method_trait_impl_a(), "<S as TraitA>::method_trait_impl_a");
        assert_eq!(s.method_trait_struct_a(), "S::method_trait_struct_a");
        assert_eq!(s.method_trait_impl_struct_a(), "S::method_trait_impl_struct_a");
        assert_eq!(s.method_impl_struct_ab(), "S::method_impl_struct_ab");
        assert_eq!(s.method_trait_struct_ab(), "S::method_trait_struct_ab");
        assert_eq!(s.method_trait_impl_struct_ab(), "S::method_trait_impl_struct_ab");
    }

    #[test]
    fn visibility() {
        mod inner {
            // All trait functions & methods take visibility from trait
            pub trait PublicTrait {
                fn public_function() {}
                fn public_method(&self) {}
            }

            trait PrivateTrait {
                fn private_function() {}
                fn private_method(&self) {}
            }

            pub struct PublicStruct;

            impl PublicTrait for PublicStruct {
                fn public_function() {
                }

                fn public_method(&self) {
                }
            }

            impl PrivateTrait for PublicStruct {
                fn private_function() {
                }

                fn private_method(&self) {
                }
            }

            struct PrivateStruct;

            pub fn demo() {
                let s = PublicStruct;

                s.public_method();
                s.private_method();
                PublicStruct::public_function();
                PublicStruct::private_function();
            }

            // Not really public due to private return type
            #[allow(dead_code)]
            pub fn get_private_struct() -> PrivateStruct {
                PrivateStruct
            }
        }

        inner::demo();

        // No: public method but returns private type
        // let _ = inner::get_private_struct();

        let s = inner::PublicStruct;
        // Use of fully-qualified syntax is fine...
        inner::PublicTrait::public_method(&s);
        //   (same as above)
        <inner::PublicStruct as inner::PublicTrait>::public_method(&s);
        //   (same as above because not defined directly in S)
        inner::PublicStruct::public_method(&s);
        //   (associated function)
        <inner::PublicStruct as inner::PublicTrait>::public_function();
        //   (same as above because not defined directly in S)
        inner::PublicStruct::public_function();

        // ...but traits need to be in scope to use their methods
        use inner::PublicTrait;
        s.public_method();
    }

    #[test]
    #[allow(clippy::size_of_ref)]
    fn unsized_tail() {
        // https://doc.rust-lang.org/nomicon/exotic-sizes.html
        // https://doc.rust-lang.org/std/ptr/trait.Pointee.html

        #[derive(Debug, PartialEq, Eq)]
        // Rust generally doesn't guarantee struct order or layout but
        // does place 'value' last to account for the possibility of T being a DST
        // https://rust-lang.github.io/unsafe-code-guidelines/layout/structs-and-tuples.html#default-layout-repr-rust
        pub struct S<T: ?Sized> {
            head: i16,
            value: T
        }

        impl<T> S<[T]> {
            pub fn len(&self) -> usize {
                self.value.len()
            }

            fn fat_slice(&self, index: RangeTo<usize>) -> NonNull<S<[T]>> {
                let (ptr, len) = NonNull::from(self).to_raw_parts();
                let end = index.end;
                if end > len {
                    panic!("range end index {end} out of range for slice of length {len}");
                }

                NonNull::from_raw_parts(ptr, end)
            }
        }

        impl<T> Index<usize> for S<[T]> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                &self.value[index]
            }
        }

        impl<T> IndexMut<usize> for S<[T]> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.value[index]
            }
        }

        impl<T> Index<RangeTo<usize>> for S<[T]> {
            type Output = S<[T]>;

            fn index(&self, index: RangeTo<usize>) -> &Self::Output {
                // Safety:
                // 1) Pointer came from an initialized instance of S<[T]>
                //    and no further use of the above local pointers occurs.
                // 2) Length is <= the embedded slice length, preventing out-of-bounds read/write
                // 3) Aliasing rules will be enforced by Rust as normal for this reference.
                // 4) Lifetime of the reference is tied to self by method signature.
                unsafe { self.fat_slice(index).as_ref() }
            }
        }

        impl<T> IndexMut<RangeTo<usize>> for S<[T]> {
            fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
                // Safety: see comments in Index::index
                unsafe { self.fat_slice(index).as_mut() }
            }
        }

        // Compiler correctly rejects this: sa1 and sa2 would alias mutably (lifetimes overlap)
        // let sa2: &mut S<[i32]> = &mut S { head: 0, value: [1, 2] };
        // let sa1: &mut S<[i32]> = &mut sa2[..1];
        // sa2[0] = 1;
        // sa1[0] = 1;

        let sa2: &S<[i32]> = &S { head: 0, value: [1, 2] };
        let sa1: &S<[i32]> = &sa2[..1];
        let sb1: &S<[i32]> = &S { head: 0, value: [1] };

        // Length of embedded slice
        assert_eq!(sa2.len(), 2);
        assert_eq!(sa1.len(), 1);
        assert_eq!(sb1.len(), 1);

        assert_eq!(sa2, sa2);
        assert_eq!(sa1, sa1);
        assert_eq!(sa1, sb1);
        // Different slice lengths
        assert_ne!(sa1, sa2);

        // Assuming usual alignment/padding
        // Struct: 2-byte pad (somewhere) + i16 + i32
        assert_eq!(mem::size_of_val(sa1), 8);
         // Fat ref: 2 x usize
        assert_eq!(mem::size_of_val(&sa1), 16);
        // Thin ref: 1 x usize
        assert_eq!(mem::size_of_val(&&sa1), 8);

        // Equal values but different structs entirely
        assert!(!ptr::addr_eq(sa1, sb1));
        // Same address...
        assert!(ptr::addr_eq(sa2, sa1));
        // ...but different lengths in the fat pointers so unequal
        assert!(!ptr::eq(sa2, sa1));
    }
}
