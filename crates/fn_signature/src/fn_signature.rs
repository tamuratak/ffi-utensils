// MIT License
//
// Copyright (c) Steven Sheldon, Mads Marquart
//
// https://github.com/madsmtm/objc2/blob/master/crates/objc2/src/runtime/method_implementation.rs
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use crate::encode::{EncodeArgument, EncodeArguments, EncodeReturn};

// https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/
mod private {
    pub trait Sealed {}
}

/// Types that can be used as the implementation of an Objective-C method.
///
/// This is a sealed trait that is implemented for a lot of `extern "C"`
/// function pointer types.
//
// Note: `Sized` is intentionally added to make the trait not object safe.
pub trait FnSignature: private::Sealed + Sized {
    /// The argument types of the method.
    type Arguments: EncodeArguments;

    /// The return type of the method.
    type Return: EncodeReturn;
}

macro_rules! method_impl_inner {
    ($(($unsafe:ident))? $abi:literal; $($t:ident),*) => {

        impl<R, $($t),*> private::Sealed for $($unsafe)? extern $abi fn($($t,)*) -> R
        where
            R: EncodeReturn,
            $($t: EncodeArgument,)*
        { }

        impl<R, $($t),*> FnSignature for $($unsafe)? extern $abi fn($($t,)*) -> R
        where
            R: EncodeReturn,
            $($t: EncodeArgument,)*
        {
            type Arguments = ($($t,)*);
            type Return = R;
        }

        impl<R, $($t),*> private::Sealed for $($unsafe)? fn($($t,)*) -> R
        where
            R: EncodeReturn,
            $($t: EncodeArgument,)*
        { }

        impl<R, $($t),*> FnSignature for $($unsafe)? fn($($t,)*) -> R
        where
            R: EncodeReturn,
            $($t: EncodeArgument,)*
        {
            type Arguments = ($($t,)*);
            type Return = R;
        }
    };
}

macro_rules! method_impl {
    ($($t:ident),*) => {
        method_impl_inner!((unsafe) "C"; $($t),*);
        method_impl_inner!("C"; $($t),*);
        #[cfg(feature = "unstable-c-unwind")]
        method_impl_inner!((unsafe) "C-unwind"; $($t),*);
        #[cfg(feature = "unstable-c-unwind")]
        method_impl_inner!("C-unwind"; $($t),*);
    };
}

method_impl!();
method_impl!(A);
method_impl!(A, B);
method_impl!(A, B, C);
method_impl!(A, B, C, D);
method_impl!(A, B, C, D, E);
method_impl!(A, B, C, D, E, F);
method_impl!(A, B, C, D, E, F, G);
method_impl!(A, B, C, D, E, F, G, H);
method_impl!(A, B, C, D, E, F, G, H, I);
method_impl!(A, B, C, D, E, F, G, H, I, J);
method_impl!(A, B, C, D, E, F, G, H, I, J, K);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
