extern crate impl_macro;
use impl_macro::impl_protocol;

// https://docs.rs/objc2/0.5.0/objc2/declare/struct.ClassBuilder.html

use core::cell::Cell;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

use objc2::rc::Id;
use objc2::runtime::{AnyClass, AnyObject, Bool, Imp, MethodImplementation, NSObject, Sel};
use std::ffi::CString;

use objc2::encode::{Encode, EncodeArguments, EncodeReturn, Encoding};
use objc2::{msg_send, msg_send_id, sel, ClassType, Message};

fn method_type_encoding(ret: &Encoding, args: &[Encoding]) -> CString {
    // First two arguments are always self and the selector
    let mut types = format!("{ret}{}{}", <*mut AnyObject>::ENCODING, Sel::ENCODING);
    for enc in args {
        use core::fmt::Write;
        write!(&mut types, "{enc}").unwrap();
    }
    CString::new(types).unwrap()
}

trait Log2Alignment {
    const LOG2_ALIGNMENT: u8;
}

impl<T> Log2Alignment for T {
    const LOG2_ALIGNMENT: u8 = {
        let align = std::mem::align_of::<T>();
        assert!(
            align.count_ones() == 1,
            "alignment required to be a power of 2"
        );
        // log2 of a power of 2 is the number of trailing zeros
        align.trailing_zeros() as u8
    };
}

fn register(klass: *mut objc_sys::objc_class) -> &'static AnyClass {
    // Forget self, otherwise the class will be disposed in drop
    let this = ManuallyDrop::new(klass);
    unsafe { objc_sys::objc_registerClassPair(this.as_mut().unwrap()) };
    let cls = NonNull::new(klass).unwrap();
    unsafe { cls.cast::<AnyClass>().as_ref() }
}

pub unsafe fn add_method<T, F>(klass: *mut objc_sys::objc_class, sel: Sel, func: F)
where
    T: Message + ?Sized,
    F: MethodImplementation<Callee = T>,
{
    unsafe {
        add_method_inner(
            klass,
            sel,
            F::Arguments::ENCODINGS,
            &F::Return::ENCODING_RETURN,
            func.__imp(),
        )
    }
}

unsafe fn add_method_inner(
    klass: *mut objc_sys::objc_class,
    sel: Sel,
    enc_args: &[Encoding],
    enc_ret: &Encoding,
    func: Imp,
) {
    let types = method_type_encoding(enc_ret, enc_args);
    let success = Bool::from_raw(unsafe {
        objc_sys::class_addMethod(klass, sel.as_ptr(), Some(func), types.as_ptr())
    });
    assert!(success.as_bool(), "failed to add method {sel}");
}

pub unsafe fn add_ivar<T: Encode>(klass: *mut objc_sys::objc_class, name: &str) {
    unsafe { add_ivar_inner::<T>(klass, name, &T::ENCODING) }
}

fn metaclass_mut(klass: *const objc_sys::objc_class) -> *mut objc_sys::objc_class {
    unsafe {
        objc_sys::object_getClass(klass as *const objc_sys::objc_object)
            as *mut objc_sys::objc_class
    }
}

pub unsafe fn add_class_method<F>(klass: *mut objc_sys::objc_class, sel: Sel, func: F)
where
    F: MethodImplementation<Callee = AnyClass>,
{
    unsafe {
        add_class_method_inner(
            klass,
            sel,
            F::Arguments::ENCODINGS,
            &F::Return::ENCODING_RETURN,
            func.__imp(),
        )
    }
}

unsafe fn add_class_method_inner(
    klass: *mut objc_sys::objc_class,
    sel: Sel,
    enc_args: &[Encoding],
    enc_ret: &Encoding,
    func: Imp,
) {
    let types = method_type_encoding(enc_ret, enc_args);
    let success = Bool::from_raw(unsafe {
        objc_sys::class_addMethod(
            metaclass_mut(klass),
            sel.as_ptr(),
            Some(func),
            types.as_ptr(),
        )
    });
    assert!(success.as_bool(), "failed to add class method {sel}");
}

pub(crate) unsafe fn add_ivar_inner<T>(
    klass: *mut objc_sys::objc_class,
    name: &str,
    encoding: &Encoding,
) {
    unsafe {
        add_ivar_inner_mono(
            klass,
            name,
            std::mem::size_of::<T>(),
            T::LOG2_ALIGNMENT,
            encoding,
        )
    }
}

unsafe fn add_ivar_inner_mono(
    klass: *mut objc_sys::objc_class,
    name: &str,
    size: usize,
    align: u8,
    encoding: &Encoding,
) {
    let c_name = CString::new(name).unwrap();
    let encoding = CString::new(encoding.to_string()).unwrap();
    let success = Bool::from_raw(unsafe {
        objc_sys::class_addIvar(klass, c_name.as_ptr(), size, align, encoding.as_ptr())
    });
    assert!(success.as_bool(), "failed to add ivar {name}");
}

unsafe extern "C" fn init_with_number(
    this: &mut AnyObject,
    _cmd: Sel,
    number: u32,
) -> Option<&mut AnyObject> {
    let this: Option<&mut AnyObject> = msg_send![super(this, NSObject::class()), init];
    this.map(|this| {
        let ivar = AnyClass::get("MyNumber")
            .unwrap()
            .instance_variable("_number")
            .unwrap();
        // SAFETY: The ivar is added with the same type above
        *ivar.load_mut::<Cell<u32>>(this) = Cell::new(number);
        this
    })
}

extern "C" fn with_number(cls: &AnyClass, _cmd: Sel, number: u32) -> *mut NSObject {
    let obj: Option<Id<NSObject>> = unsafe {
        msg_send_id![
            msg_send_id![cls, alloc],
            initWithNumber: number,
        ]
    };
    obj.map(Id::autorelease_return)
        .unwrap_or(std::ptr::null_mut())
}

extern "C" fn my_number_set(this: &NSObject, _cmd: Sel, number: u32) {
    let ivar = AnyClass::get("MyNumber")
        .unwrap()
        .instance_variable("_number")
        .unwrap();
    // SAFETY: The ivar is added with the same type above
    unsafe { ivar.load::<Cell<u32>>(this) }.set(number);
}

extern "C" fn my_number_get(this: &NSObject, _cmd: Sel) -> u32 {
    let ivar = AnyClass::get("MyNumber")
        .unwrap()
        .instance_variable("_number")
        .unwrap();
    // SAFETY: The ivar is added with the same type above
    unsafe { ivar.load::<Cell<u32>>(this) }.get()
}

fn register_class() -> &'static AnyClass {
    {
        let superclass = Some(NSObject::class());
        let name = CString::new("MyNumber").unwrap();
        let super_ptr = superclass.map_or(std::ptr::null(), |c| c).cast();
        let klass = unsafe { objc_sys::objc_allocateClassPair(super_ptr, name.as_ptr(), 0) };
        unsafe { add_ivar::<Cell<u32>>(klass, "_number") };
        unsafe {
            add_method(
                klass,
                sel!(initWithNumber:),
                init_with_number as unsafe extern "C" fn(_, _, _) -> _,
            );
            add_class_method(
                klass,
                sel!(withNumber:),
                with_number as extern "C" fn(_, _, _) -> _,
            );
            add_method(klass, sel!(setNumber:), my_number_set as extern "C" fn(_, _, _));
            add_method(klass, sel!(number), my_number_get as extern "C" fn(_, _) -> _);
        }
        register(klass)
    }
    /*
    // Inherit from NSObject
    let mut builder = ClassBuilder::new("MyNumber", NSObject::class())
        .expect("a class with the name MyNumber likely already exists");

    // Add an instance variable of type `Cell<u32>`
    builder.add_ivar::<Cell<u32>>("_number");

    unsafe {
        builder.add_method(
            sel!(initWithNumber:),
            init_with_number as unsafe extern "C" fn(_, _, _) -> _,
        );
        builder.add_class_method(
            sel!(withNumber:),
            with_number as extern "C" fn(_, _, _) -> _,
        );
        builder.add_method(sel!(setNumber:), my_number_set as extern "C" fn(_, _, _));
        builder.add_method(sel!(number), my_number_get as extern "C" fn(_, _) -> _);
    }

    builder.register()
*/
}

fn main() {
    let cls = register_class();

    let obj: Id<NSObject> = unsafe { msg_send_id![cls, withNumber: 42u32] };

    let n: u32 = unsafe { msg_send![&obj, number] };
    assert_eq!(n, 42);

    let _: () = unsafe { msg_send![&obj, setNumber: 12u32] };
    let n: u32 = unsafe { msg_send![&obj, number] };
    assert_eq!(n, 12);
}
