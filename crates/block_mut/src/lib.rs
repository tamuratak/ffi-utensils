//! # Apple's C language extension of blocks
//!
//! C Blocks are functions which capture their environments, i.e. the
//! C-equivalent of Rust's [`FnMut`] closures. As they were originally developed
//! by Apple, they're often used in Objective-C code. This crate provides
//! capabilities to create, manage and invoke these blocks, in an ergonomic,
//! "Rust-centric" fashion.
//!
//! At a high level, this crate contains four types, each representing
//! different kinds of blocks, and different kinds of ownership.
//!
//! | `block_mut` type                            | Equivalent Rust type  |
//! | ---------------------------------------- | --------------------- |
//! | `&Block<dyn FnMut() + 'a>`                  | `&dyn FnMut() + 'a`      |
//! | `BoxBlock<dyn FnMut() + 'a>`                 | `Box<dyn FnMut() + 'a>`  |
//! | `StackBlock<'a, (), (), impl FnMut() + 'a>` | `impl FnMut() + 'a`      |
//! | `GlobalBlock<dyn FnMut()>`                  | [`fn` item]           |
//!
//! For more information on the specifics of the block implementation, see the
//! [C language specification][lang] and the [ABI specification][ABI].
//!
//! [lang]: https://clang.llvm.org/docs/BlockLanguageSpec.html
//! [ABI]: http://clang.llvm.org/docs/Block-ABI-Apple.html
//! [`fn` item]: https://doc.rust-lang.org/reference/types/function-item.html
//!
//!
//! ## External functions using blocks
//!
//! To declare external functions or methods that takes blocks, use
//! `&Block<dyn FnMut(Params) -> R>` or `Option<&Block<dyn FnMut(Args) -> R>>`,
//! where `Params` is the parameter types, and `R` is the return type.
//!
//! In the next few examples, we're going to work with a function
//! `check_addition`, that takes as parameter a block that adds two integers,
//! and checks that the addition is correct.
//!
//! Such a function could be written in C like in the following.
//!
//! ```objc
//! #include <cassert>
//! #include <stdint.h>
//! #include <Block.h>
//!
//! void check_addition(int32_t (^block)(int32_t, int32_t)) {
//!     assert(block(5, 8) == 13);
//! }
//! ```
//!
//! An `extern "C" { ... }` declaration for that function would then be:
//!
//! ```
//! use block_mut::Block;
//!
//! extern "C" {
//!     fn check_addition(block: &Block<dyn FnMut(i32, i32) -> i32>);
//! }
//! ```
//!
//! If the function/method allowed passing `NULL` blocks, the type would be
//! `Option<&Block<dyn FnMut(i32, i32) -> i32>>` instead.
//!
//!
//! ## Invoking blocks
//!
//! We can also define the external function in Rust, and expose it to
//! Objective-C. To do this, we can use [`Block::call`] to invoke the block
//! inside the function.
//!
//! ```
//! use block_mut::Block;
//!
//! #[no_mangle]
//! extern "C" fn check_addition(block: &Block<dyn FnMut(i32, i32) -> i32>) {
//!     assert_eq!(block.call((5, 8)), 13);
//! }
//! ```
//!
//! Note the extra parentheses in the `call` method, since the arguments must
//! be passed as a tuple.
//!
//!
//! ## Creating blocks
//!
//! Creating a block to pass to Objective-C can be done with [`BoxBlock`] or
//! [`StackBlock`], depending on if you want to move the block to the heap,
//! or let the callee decide if it needs to do that.
//!
//! To call such a function / method, we could create a new block from a
//! closure using [`BoxBlock::new`].
//!
//! ```
//! use block_mut::BoxBlock;
//! #
//! # extern "C" fn check_addition(block: &block_mut::Block<dyn FnMut(i32, i32) -> i32>) {
//! #     assert_eq!(block.call((5, 8)), 13);
//! # }
//!
//! let block = BoxBlock::new(|a, b| a + b);
//! check_addition(&block);
//! ```
//!
//! This creates the block on the heap. If the external function you're
//! calling is not going to copy the block, it may be more performant if you
//! construct a [`StackBlock`] directly, using [`StackBlock::new`].
//!
//! Note that this requires that the closure is [`Clone`], as the external
//! code is allowed to copy the block to the heap in the future.
//!
//! ```
//! use block_mut::StackBlock;
//! #
//! # extern "C" fn check_addition(block: &block_mut::Block<dyn FnMut(i32, i32) -> i32>) {
//! #     assert_eq!(block.call((5, 8)), 13);
//! # }
//!
//! let block = StackBlock::new(|a, b| a + b);
//! check_addition(&block);
//! ```
//!
//! As an optimization, if your closure doesn't capture any variables (as in
//! the above examples), you can use the [`global_block!`] macro to create a
//! static block.
//!
//! ```
//! use block_mut::global_block;
//! #
//! # extern "C" fn check_addition(block: &block_mut::Block<dyn FnMut(i32, i32) -> i32>) {
//! #     assert_eq!(block.call((5, 8)), 13);
//! # }
//!
//! global_block! {
//!     static BLOCK = |a: i32, b: i32| -> i32 {
//!         a + b
//!     };
//! }
//!
//! check_addition(&BLOCK);
//! ```
//!
//!
//! ## Lifetimes
//!
//! When dealing with blocks, there can be quite a few lifetimes to keep in
//! mind.
//!
//! The most important one is the lifetime of the block's data, i.e. the
//! lifetime of the data in the closure contained in the block. This lifetime
//! can be specified as `'f` in `&Block<dyn FnMut() + 'f>`.
//!
//! Note that `&Block<dyn FnMut()>`, without any lifetime specifier, can be a bit
//! confusing, as the default depends on where it is typed. In function/method
//! signatures, it defaults to `'static`, but as the type of e.g. a `let`
//! binding, the lifetime may be inferred to be something smaller, see [the
//! reference][ref-dyn-lifetime] for details. If in doubt, either add a
//! `+ 'static` or `+ '_` to force an escaping or non-escaping block.
//!
//! Another lifetime is the lifetime of the currently held pointer, i.e. `'b`
//! in `&'b Block<dyn FnMut()>`. This lifetime can be safely extended using
//! [`Block::copy`], so should prove to be little trouble (of course the
//! lifetime still can't be extended past the lifetime of the captured data
//! above).
//!
//! Finally, the block's parameter and return types can also contain
//! lifetimes, as `'a` and `'r` in `&Block<dyn FnMut(&'a i32) -> &'r u32>`.
//! Unfortunately, these lifetimes are quite problematic and unsupported at
//! the moment, due to Rust trait limitations regarding higher-ranked trait
//! bounds. If you run into problems with this in a block that takes or
//! returns a reference, consider using the ABI-compatible `NonNull<T>`, or
//! transmute to a `'static` lifetime.
//!
//! [ref-dyn-lifetime]: https://doc.rust-lang.org/reference/lifetime-elision.html#default-trait-object-lifetimes
//!
//!
//! ## Thread safety
//!
//! Thread-safe blocks are not yet representable in `block_mut`, and as such any
//! function that requires a thread-safe block must be marked `unsafe`.
//!
//!
//! ## Mutability
//!
//! Blocks are generally assumed to be shareable, and as such can only very
//! rarely be made mutable. In particular, there is no good way to prevent
//! re-entrancy.
//!
//! You will likely have to use interior mutability instead.
//!
//!
//! ## Specifying a runtime
//!
//! Different runtime implementations exist and act in slightly different ways
//! (and have several different helper functions), the most important aspect
//! being that the libraries are named differently, so we must take that into
//! account when linking.
//!
//! You can choose the desired runtime by using the relevant cargo feature
//! flags, see the following sections (you might have to disable the default
//! `"apple"` feature first).
//!
//!
//! ### Apple's [`libclosure`](https://github.com/apple-oss-distributions/libclosure)
//!
//! - Feature flag: `apple`.
//!
//! This is the most common and most sophisticated runtime, and it has quite a
//! lot more features than the specification mandates.
//!
//! The minimum required operating system versions are as follows (though in
//! practice Rust itself requires higher versions than this):
//!
//! - macOS: `10.6`
//! - iOS/iPadOS: `3.2`
//! - tvOS: `1.0`
//! - watchOS: `1.0`
//!
//! **This is used by default**, so you do not need to specify a runtime if
//! you're using this crate on of these platforms.
//!
//! ## C Compiler configuration
//!
//! To our knowledge, only Clang supports blocks. To compile C or Objective-C
//! sources using these features, you should set [the `-fblocks` flag][flag].
//!
//! [flag]: https://clang.llvm.org/docs/ClangCommandLineReference.html#cmdoption-clang-fblocks

#![no_std]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::cargo)]
#![warn(clippy::ptr_as_ptr)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/block_mut/0.4.0")]
#![cfg_attr(feature = "unstable-docsrs", feature(doc_auto_cfg, doc_cfg_hide))]
#![cfg_attr(feature = "unstable-docsrs", doc(cfg_hide(doc)))]

extern crate alloc;
extern crate std;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

// Link to `libclosure` (internally called `libsystem_blocks.dylib`), which is
// exported by `libSystem.dylib`.
//
// Note: Don't get confused by the presence of `System.framework`, it is a
// deprecated wrapper over the dynamic library, so we'd rather use the latter.
//
// Alternative: Only link to `libsystem_blocks.dylib`:
// println!("cargo:rustc-link-search=native=/usr/lib/system");
// println!("cargo:rustc-link-lib=dylib=system_blocks");
#[link(name = "System", kind = "dylib")]
extern "C" {}

mod abi;
mod block;
mod debug;
pub mod ffi;
mod global;
mod box_block;
mod stack;
mod traits;

pub use self::block::Block;
pub use self::global::GlobalBlock;
pub use self::box_block::BoxBlock;
pub use self::stack::StackBlock;
pub use self::traits::{BlockFn, IntoBlock};

// Note: We could use `_Block_object_assign` and `_Block_object_dispose` to
// implement a `ByRef<T>` wrapper, which would behave like `__block` marked
// variables and have roughly the same memory management scheme as blocks.
//
// But I've yet to see the value in such a wrapper in Rust code compared to
// just using `Box`, `Rc` or `Arc`, and since `__block` variables are
// basically never exposed as part of a (public) function's API, we won't
// implement such a thing yet.
