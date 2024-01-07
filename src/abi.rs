//! The documentation for these bindings is a mix from GNUStep's and Apple's
//! sources, but the [ABI specification][ABI] is really the place you should
//! be looking!
//!
//! [ABI]: https://clang.llvm.org/docs/Block-ABI-Apple.html
#![allow(unused)]

use core::{ffi::c_void, mem::MaybeUninit};
use std::os::raw::{c_char, c_int, c_ulong};

use crate::ffi::Class;

/// Block descriptor flags.
#[allow(non_camel_case_types)]
pub(crate) type BlockFlags = c_int;

/// Note: Not public ABI.
pub(crate) const BLOCK_DEALLOCATING: BlockFlags = 0x0001;

/// Note: Not public ABI.
pub(crate) const BLOCK_REFCOUNT_MASK: BlockFlags = if cfg!(feature = "gnustep-1-7") {
    // Mask for the reference count in byref structure's flags field. The low
    // 3 bytes are reserved for the reference count, the top byte for the flags.
    0x00ffffff
} else if cfg!(any(feature = "compiler-rt", feature = "objfw")) {
    0xffff
} else if cfg!(feature = "apple") {
    0xfffe // runtime
} else {
    0
};

/// Note: Not public ABI.
pub(crate) const BLOCK_INLINE_LAYOUT_STRING: BlockFlags = 1 << 21;

/// Note: Not public ABI.
pub(crate) const BLOCK_SMALL_DESCRIPTOR: BlockFlags = 1 << 22;

pub(crate) const BLOCK_IS_NOESCAPE: BlockFlags = 1 << 23;

/// Note: Not public ABI.
pub(crate) const BLOCK_NEEDS_FREE: BlockFlags = 1 << 24;

/// The block descriptor contains copy and dispose helpers.
pub(crate) const BLOCK_HAS_COPY_DISPOSE: BlockFlags = 1 << 25;

/// Helpers have C++ code.
#[doc(alias = "BLOCK_HAS_CXX_OBJ")]
pub(crate) const BLOCK_HAS_CTOR: BlockFlags = 1 << 26;

/// Note: Not public ABI.
pub(crate) const BLOCK_IS_GC: BlockFlags = 1 << 27;

/// Block is stored in global memory and does not need to be copied.
pub(crate) const BLOCK_IS_GLOBAL: BlockFlags = 1 << 28;

/// Block function uses a calling convention that returns a structure via a
/// pointer passed in by the caller.
///
/// match (BLOCK_USE_STRET, BLOCK_HAS_SIGNATURE) {
///     (false, false) => 10.6.ABI, no signature field available
///     (true, false)  => 10.6.ABI, no signature field available
///     (false, true)  => ABI.2010.3.16, regular calling convention, presence of signature field
///     (true, true)   => ABI.2010.3.16, stret calling convention, presence of signature field,
/// }
///
/// See <https://clang.llvm.org/docs/Block-ABI-Apple.html#high-level>
#[doc(alias = "BLOCK_USE_SRET")]
#[doc(alias = "BLOCK_HAS_DESCRIPTOR")]
pub(crate) const BLOCK_USE_STRET: BlockFlags = 1 << 29;

/// Block has an Objective-C type encoding.
pub(crate) const BLOCK_HAS_SIGNATURE: BlockFlags = 1 << 30;

/// Note: Not public ABI.
pub(crate) const BLOCK_HAS_EXTENDED_LAYOUT: BlockFlags = 1 << 31;

/// The value is of some id-like type, and should be copied as an Objective-C
/// object: i.e. by sending -retain or via the GC assign functions in GC mode
/// (not yet supported).
///
/// id, NSObject, __attribute__((NSObject)), block, ...
pub(crate) const BLOCK_FIELD_IS_OBJECT: c_int = 3;

/// The field is a block.  This must be copied by the block copy functions.
///
/// a block variable
pub(crate) const BLOCK_FIELD_IS_BLOCK: c_int = 7;

/// The field is an indirect reference to a variable declared with the __block
/// storage qualifier.
///
/// the on stack structure holding the __block variable
pub(crate) const BLOCK_FIELD_IS_BYREF: c_int = 8;

/// The field is an indirect reference to a variable declared with the __block
/// storage qualifier.
///
/// declared __weak, only used in byref copy helpers
pub(crate) const BLOCK_FIELD_IS_WEAK: c_int = 16;

/// The field is an indirect reference to a variable declared with the __block
/// storage qualifier.
///
/// called from __block (byref) copy/dispose support routines.
pub(crate) const BLOCK_BYREF_CALLER: c_int = 128;

/// The expected layout of every block.
#[repr(C)]
#[doc(alias = "__block_literal")]
#[doc(alias = "Block_layout")]
#[doc(alias = "Block_basic")]
#[allow(missing_debug_implementations)]
pub struct BlockLayout {
    /// Class pointer.
    ///
    /// Always initialised to &_NSConcreteStackBlock for blocks that are
    /// created on the stack or &_NSConcreteGlobalBlock for blocks that are
    /// created in global storage.
    pub isa: *const Class,
    /// Flags.
    ///
    /// See the `BlockFlags` enumerated type for possible values.
    ///
    /// Contains reference count in Apple's and ObjFW's runtime.
    #[doc(alias = "Block_flags")]
    pub(crate) flags: BlockFlags,
    /// Reserved.
    ///
    /// Initialized to 0 by the compiler, but is said to be uninitialized in
    /// the specification.
    ///
    /// Used for the reference count in GNUStep's and WinObjC's runtime.
    #[doc(alias = "Block_size")]
    pub(crate) reserved: MaybeUninit<c_int>,
    /// The function that implements the block.
    ///
    /// The first argument is a pointer to this structure, the subsequent
    /// arguments are the block's explicit parameters.
    ///
    /// If the BLOCK_USE_SRET & BLOCK_HAS_SIGNATURE flag is set, there is an
    /// additional hidden argument, which is a pointer to the space on the
    /// stack allocated to hold the return value.
    pub invoke: Option<unsafe extern "C" fn()>,
    /// The block's descriptor. The actual type of this is:
    /// ```pseudo-code
    /// match (BLOCK_HAS_COPY_DISPOSE, BLOCK_HAS_SIGNATURE) {
    ///     (false, false) => BlockDescriptor,
    ///     (true, false) => BlockDescriptorCopyDispose,
    ///     (false, true) => BlockDescriptorSignature,
    ///     (true, true) => BlockDescriptorCopyDisposeSignature,
    /// }
    /// ```
    ///
    /// Since all of these start with `BlockDescriptor`, it is always safe to
    /// reinterpret this pointer as that.
    ///
    /// Note: We don't use a `union` here, since that would be forced to have
    /// a greater size than is actually required.
    pub(crate) descriptor: *const c_void,
}

/// Basic block descriptor.
#[repr(C)]
#[doc(alias = "__block_descriptor")]
#[doc(alias = "Block_descriptor_1")]
pub(crate) struct BlockDescriptor {
    /// Reserved for future use. Currently always 0.
    pub(crate) reserved: c_ulong,
    /// Size of the block.
    pub(crate) size: c_ulong,
}

/// Block descriptor that contains copy and dispose operations.
///
/// Requires BLOCK_HAS_COPY_DISPOSE.
#[repr(C)]
#[doc(alias = "__block_descriptor")]
#[doc(alias = "Block_descriptor_2")]
pub(crate) struct BlockDescriptorCopyDispose {
    /// Reserved for future use. Currently always 0.
    pub(crate) reserved: c_ulong,
    /// Size of the block.
    pub(crate) size: c_ulong,

    /// Helper to copy the block if it contains nontrivial copy operations.
    pub(crate) copy: Option<unsafe extern "C" fn(dst: *mut c_void, src: *mut c_void)>,
    /// Helper to destroy the block after being copied.
    pub(crate) dispose: Option<unsafe extern "C" fn(src: *mut c_void)>,
}

/// Block descriptor that has an encoding / a signature.
///
/// Requires BLOCK_HAS_SIGNATURE.
#[repr(C)]
#[doc(alias = "__block_descriptor")]
#[doc(alias = "Block_descriptor_3")]
pub(crate) struct BlockDescriptorSignature {
    /// Reserved for future use. Currently always 0.
    pub(crate) reserved: c_ulong,
    /// Size of the block.
    pub(crate) size: c_ulong,

    /// Objective-C type encoding of the block.
    #[doc(alias = "signature")]
    pub(crate) encoding: *const c_char,
}

/// Block descriptor that contains copy and dispose operations, and which
/// has an encoding / a signature.
///
/// Requires BLOCK_HAS_COPY_DISPOSE and BLOCK_HAS_SIGNATURE.
#[repr(C)]
#[doc(alias = "__block_descriptor")]
#[doc(alias = "Block_descriptor_2")]
#[doc(alias = "Block_descriptor_3")]
pub(crate) struct BlockDescriptorCopyDisposeSignature {
    /// Reserved for future use. Currently always 0.
    pub(crate) reserved: c_ulong,
    /// Size of the block.
    pub(crate) size: c_ulong,

    /// Helper to copy the block if it contains nontrivial copy operations.
    pub(crate) copy: Option<unsafe extern "C" fn(dst: *mut c_void, src: *mut c_void)>,
    /// Helper to destroy the block if required.
    pub(crate) dispose: Option<unsafe extern "C" fn(src: *mut c_void)>,

    /// Objective-C type encoding of the block.
    #[doc(alias = "signature")]
    pub(crate) encoding: *const c_char,
}
