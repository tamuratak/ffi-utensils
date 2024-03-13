use core::mem;
use core::ptr;

use crate::{Block, StackBlock};

mod private {
    pub trait Sealed<A, R> {}
}

/// Types that represent closure parameters/arguments and return types in a
/// block.
///
/// This is implemented for [`dyn`] [`Fn`] closures with up to 12 parameters,
/// where each parameter implements [`EncodeArgument`] and the return type
/// implements [`EncodeReturn`].
///
/// [`dyn`]: https://doc.rust-lang.org/std/keyword.dyn.html
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait BlockFn: private::Sealed<Self::Args, Self::Output> {
    /// The parameters/arguments to the block.
    type Args;

    /// The return type of the block.
    type Output;

    /// Calls the given invoke function with the block and arguments.
    #[doc(hidden)]
    unsafe fn __call_block(
        invoke: unsafe extern "C" fn(),
        block: *mut Block<Self>,
        args: Self::Args,
    ) -> Self::Output;
}

/// Types that may be converted into a block.
///
/// This is implemented for [`Fn`] closures of up to 12 parameters, where each
/// parameter implements [`EncodeArgument`] and the return type implements
/// [`EncodeReturn`].
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait IntoBlock<'f, A, R>: private::Sealed<A, R>
// where
//     A: EncodeArguments,
//    R: EncodeReturn,
{
    /// The type-erased `dyn FnMut(...Args) -> R + 'f`.
    type Dyn: ?Sized + BlockFn<Args = A, Output = R>;

    #[doc(hidden)]
    fn __get_invoke_stack_block() -> unsafe extern "C" fn();
}

macro_rules! impl_traits {
    ($($a:ident: $t:ident),*) => (
        impl<$($t,)* R, Closure> private::Sealed<($($t,)*), R> for Closure
        where
            Closure: ?Sized + FnMut($($t),*) -> R,
        {}

        // TODO: Add `+ Send`, `+ Sync` and `+ Send + Sync` versions.
        unsafe impl<$($t,)* R> BlockFn for dyn FnMut($($t),*) -> R + '_ {
            type Args = ($($t,)*);
            type Output = R;

            #[inline]
            unsafe fn __call_block(
                invoke: unsafe extern "C" fn(),
                block: *mut Block<Self>,
                ($($a,)*): Self::Args,
            ) -> Self::Output {
                // Very similar to `MessageArguments::__invoke`
                let invoke: unsafe extern "C" fn(*mut Block<Self> $(, $t)*) -> R = unsafe {
                    mem::transmute(invoke)
                };

                unsafe { invoke(block $(, $a)*) }
            }
        }

        unsafe impl<'f, $($t,)* R, Closure> IntoBlock<'f, ($($t,)*), R> for Closure
        where
            Closure: FnMut($($t),*) -> R + 'f,
        {
            type Dyn = dyn FnMut($($t),*) -> R + 'f;

            #[inline]
            fn __get_invoke_stack_block() -> unsafe extern "C" fn() {
                unsafe extern "C" fn invoke<'f, $($t,)* R, Closure>(
                    block: *mut StackBlock<'f, ($($t,)*), R, Closure>,
                    $($a: $t,)*
                ) -> R
                where
                    Closure: FnMut($($t),*) -> R + 'f
                {
                    let closure = unsafe { &mut *(ptr::addr_of!((*block).closure) as *mut Closure)};
                    (closure)($($a),*)
                }

                unsafe {
                    mem::transmute::<
                        unsafe extern "C" fn(*mut StackBlock<'f, ($($t,)*), R, Closure>, $($t,)*) -> R,
                        unsafe extern "C" fn(),
                    >(invoke)
                }
            }
        }
    );
}

impl_traits!();
impl_traits!(t0: T0);
impl_traits!(t0: T0, t1: T1);
impl_traits!(t0: T0, t1: T1, t2: T2);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10, t11: T11);
