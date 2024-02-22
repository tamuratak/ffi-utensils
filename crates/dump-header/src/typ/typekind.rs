use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(remote = "clang::TypeKind")]
pub enum TypeKindDef {
    /// A type whose specific kind is not exposed via this interface.
    Unexposed = 1,
    /// `void`
    Void = 2,
    /// `bool` (C++) or `_Bool` (C99)
    Bool = 3,
    /// The `char` type when it is signed by default.
    CharS = 13,
    /// The `char` type when it is unsigned by default.
    CharU = 4,
    /// `signed char`
    SChar = 14,
    /// `unsigned char`
    UChar = 5,
    /// `wchar_t`
    WChar = 15,
    /// `char16_t`
    Char16 = 6,
    /// `char32_t`
    Char32 = 7,
    /// `short`
    Short = 16,
    /// `unsigned short`
    UShort = 8,
    /// `int`
    Int = 17,
    /// `unsigned int`
    UInt = 9,
    /// `long`
    Long = 18,
    /// `unsigned long`
    ULong = 10,
    /// `long long`
    LongLong = 19,
    /// `unsigned long long`
    ULongLong = 11,
    /// `__int128_t`
    Int128 = 20,
    /// `__uint128_t`
    UInt128 = 12,
    /// A half-precision (16-bit) floating point type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    Half = 31,
    /// A half-precision (16-bit) floating point type.
    ///
    /// Only produced by `libclang` 6.0 and later.
    Float16 = 32,
    /// `short _Accum`
    ///
    /// Only produced by `libclang` 7.0 and later.
    ShortAccum = 33,
    /// `_Accum`
    ///
    /// Only produced by `libclang` 7.0 and later.
    Accum = 34,
    /// `long _Accum`
    ///
    /// Only produced by `libclang` 7.0 and later.
    LongAccum = 35,
    /// `unsigned short _Accum`
    ///
    /// Only produced by `libclang` 7.0 and later.
    UShortAccum = 36,
    /// `unsigned _Accum`
    ///
    /// Only produced by `libclang` 7.0 and later.
    UAccum = 37,
    /// `unsigned long _Accum`
    ///
    /// Only produced by `libclang` 7.0 and later.
    ULongAccum = 38,
    /// `float`
    Float = 21,
    /// `double`
    Double = 22,
    /// `long double`
    LongDouble = 23,
    /// `nullptr_t` (C++11)
    Nullptr = 24,
    /// A C99 complex type (e.g., `_Complex float`).
    Complex = 100,
    /// An unknown dependent type.
    Dependent = 26,
    /// The type of an unresolved overload set.
    Overload = 25,
    /// `id` (Objective-C)
    ObjCId = 27,
    /// `Class` (Objective-C)
    ObjCClass = 28,
    /// `SEL` (Objective-C)
    ObjCSel = 29,
    /// `__float128`
    ///
    /// Only produced by `libclang` 3.9 and later.
    Float128 = 30,
    /// An Objective-C interface type.
    ObjCInterface = 108,
    /// An Objective-C pointer to object type.
    ObjCObjectPointer = 109,
    /// A pointer type.
    Pointer = 101,
    /// A block pointer type (e.g., `void (^)(int)`).
    BlockPointer = 102,
    /// A pointer to a record member type.
    MemberPointer = 117,
    /// An l-value reference (e.g. `int&`).
    LValueReference = 103,
    /// An r-value reference (e.g. `int&&`).
    RValueReference = 104,
    /// An enum type.
    Enum = 106,
    /// A record type such as a struct or a class.
    Record = 105,
    /// A typedef.
    Typedef = 107,
    /// A function prototype with parameter type information (e.g., `void foo(int)`).
    FunctionPrototype = 111,
    /// A function prototype without parameter type information (e.g., `void foo()`).
    FunctionNoPrototype = 110,
    /// An array type with a specified size that is an integer constant expression.
    ConstantArray = 112,
    /// An array type with a specified size that is a dependent value.
    DependentSizedArray = 116,
    /// An array type without a specified size.
    IncompleteArray = 114,
    /// An array type with a specified size that is not an integer constant expression.
    VariableArray = 115,
    /// A GCC generic vector type.
    Vector = 113,
    /// A C++11 `decltype(auto)` type.
    ///
    /// Only produced by `libclang` 3.8 and later.
    Auto = 118,
    /// A type that was referred to using an elaborated type keyword (e.g., `struct S`).
    ///
    /// Only produced by `libclang` 3.9 and later.
    Elaborated = 119,
    /// An OpenCL pipe type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    Pipe = 120,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage1dRO = 121,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage1dArrayRO = 122,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage1dBufferRO = 123,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dRO = 124,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dArrayRO = 125,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dDepthRO = 126,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dArrayDepthRO = 127,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dMSAARO = 128,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dArrayMSAARO = 129,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dMSAADepthRO = 130,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dArrayMSAADepthRO = 131,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage3dRO = 132,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage1dWO = 133,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage1dArrayWO = 134,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage1dBufferWO = 135,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dWO = 136,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dArrayWO = 137,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dDepthWO = 138,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dArrayDepthWO = 139,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dMSAAWO = 140,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dArrayMSAAWO = 141,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dMSAADepthWO = 142,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dArrayMSAADepthWO = 143,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage3dWO = 144,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage1dRW = 145,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage1dArrayRW = 146,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage1dBufferRW = 147,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dRW = 148,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dArrayRW = 149,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dDepthRW = 150,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dArrayDepthRW = 151,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dMSAARW = 152,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dArrayMSAARW = 153,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dMSAADepthRW = 154,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage2dArrayMSAADepthRW = 155,
    /// An OpenCL image type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLImage3dRW = 156,
    /// An OpenCL sampler type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLSampler = 157,
    /// An OpenCL event type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLEvent = 158,
    /// An OpenCL queue type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLQueue = 159,
    /// An OpenCL reserve ID type.
    ///
    /// Only produced by `libclang` 5.0 and later.
    OCLReserveID = 160,
    /// An Objective-C object type.
    ///
    /// Only produced by `libclang` 8.0 and later.
    ObjCObject = 161,
    /// An Objective-C type param.
    ///
    /// Only produced by `libclang` 8.0 and later.
    ObjCTypeParam = 162,
    /// An attributed type.
    ///
    /// Only produced by `libclang` 8.0 and later.
    Attributed = 163,
    /// An Intel OpenCL extension type for the AVC VME media sampler in Intel graphics processors.
    ///
    /// Only produced by `libclang` 8.0 and later.
    OCLIntelSubgroupAVCMcePayload = 164,
    /// An Intel OpenCL extension type for the AVC VME media sampler in Intel graphics processors.
    ///
    /// Only produced by `libclang` 8.0 and later.
    OCLIntelSubgroupAVCImePayload = 165,
    /// An Intel OpenCL extension type for the AVC VME media sampler in Intel graphics processors.
    ///
    /// Only produced by `libclang` 8.0 and later.
    OCLIntelSubgroupAVCRefPayload = 166,
    /// An Intel OpenCL extension type for the AVC VME media sampler in Intel graphics processors.
    ///
    /// Only produced by `libclang` 8.0 and later.
    OCLIntelSubgroupAVCSicPayload = 167,
    /// An Intel OpenCL extension type for the AVC VME media sampler in Intel graphics processors.
    ///
    /// Only produced by `libclang` 8.0 and later.
    OCLIntelSubgroupAVCMceResult = 168,
    /// An Intel OpenCL extension type for the AVC VME media sampler in Intel graphics processors.
    ///
    /// Only produced by `libclang` 8.0 and later.
    OCLIntelSubgroupAVCImeResult = 169,
    /// An Intel OpenCL extension type for the AVC VME media sampler in Intel graphics processors.
    ///
    /// Only produced by `libclang` 8.0 and later.
    OCLIntelSubgroupAVCRefResult = 170,
    /// An Intel OpenCL extension type for the AVC VME media sampler in Intel graphics processors.
    ///
    /// Only produced by `libclang` 8.0 and later.
    OCLIntelSubgroupAVCSicResult = 171,
    /// An Intel OpenCL extension type for the AVC VME media sampler in Intel graphics processors.
    ///
    /// Only produced by `libclang` 8.0 and later.
    OCLIntelSubgroupAVCImeResultSingleRefStreamout = 172,
    /// An Intel OpenCL extension type for the AVC VME media sampler in Intel graphics processors.
    ///
    /// Only produced by `libclang` 8.0 and later.
    OCLIntelSubgroupAVCImeResultDualRefStreamout = 173,
    /// An Intel OpenCL extension type for the AVC VME media sampler in Intel graphics processors.
    ///
    /// Only produced by `libclang` 8.0 and later.
    OCLIntelSubgroupAVCImeSingleRefStreamin = 174,
    /// An Intel OpenCL extension type for the AVC VME media sampler in Intel graphics processors.
    ///
    /// Only produced by `libclang` 8.0 and later.
    OCLIntelSubgroupAVCImeDualRefStreamin = 175,
    /// Extended vector type, created using `attribute((ext_vector_type(n)))`.
    ///
    /// Only produced by `libclang` 9.0 and later.
    ExtVector = 176,
}
