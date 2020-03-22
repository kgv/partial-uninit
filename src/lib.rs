#![feature(maybe_uninit_ref)]
#![feature(new_uninit)]
#![feature(specialization)]

use std::mem::MaybeUninit;

/// PartialUninit.
pub trait PartialUninit: Sized {
    fn partial_uninit() -> MaybeUninit<Self> {
        let mut uninit = MaybeUninit::<Self>::uninit();
        unsafe {
            uninit.get_mut().partial_init();
        }
        uninit
    }

    fn boxed_partial_uninit() -> Box<MaybeUninit<Self>> {
        let mut uninit = Box::<Self>::new_uninit();
        unsafe {
            uninit.get_mut().partial_init();
        }
        uninit
    }

    fn partial_init(&mut self);
}

impl<T> PartialUninit for T {
    default fn partial_init(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::PartialUninit;
    use std::mem::MaybeUninit;

    #[test]
    fn it_works() {
        struct Struct {
            a: u8,
            b: u8,
        }

        impl PartialUninit for Struct {
            fn partial_init(&mut self) {
                self.a = 9;
            }
        }

        let mut uninit: MaybeUninit<Struct> = PartialUninit::partial_uninit();
        unsafe {
            uninit.get_mut().b = 9;
        }
        let init = unsafe { uninit.assume_init() };
        assert_eq!(init.a, 9);
        assert_eq!(init.b, 9);
    }
}
