// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod component {
    #[allow(dead_code)]
    pub mod decoder {
        #[allow(dead_code, clippy::all)]
        pub mod host {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            #[allow(unused_unsafe, clippy::all)]
            pub fn log(message: &str) {
                unsafe {
                    let vec0 = message;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();

                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "component:decoder/host")]
                    extern "C" {
                        #[link_name = "log"]
                        fn wit_import(_: *mut u8, _: usize);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0);
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod decoder {
            #[allow(dead_code, clippy::all)]
            pub mod decoder {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_init_cabi<T: Guest>() {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    T::init();
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_name_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 = T::name();
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0.into_bytes()).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_name<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_format_signal_value_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::format_signal_value(_rt::string_lift(bytes0));
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec3 = (result1.into_bytes()).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr2.add(4).cast::<usize>() = len3;
                    *ptr2.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_format_signal_value<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                pub trait Guest {
                    fn init();
                    fn name() -> _rt::String;
                    fn format_signal_value(value: _rt::String) -> _rt::String;
                }
                #[doc(hidden)]

                macro_rules! __export_component_decoder_decoder_cabi{
    ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

      #[export_name = "component:decoder/decoder#init"]
      unsafe extern "C" fn export_init() {
        $($path_to_types)*::_export_init_cabi::<$ty>()
      }
      #[export_name = "component:decoder/decoder#name"]
      unsafe extern "C" fn export_name() -> *mut u8 {
        $($path_to_types)*::_export_name_cabi::<$ty>()
      }
      #[export_name = "cabi_post_component:decoder/decoder#name"]
      unsafe extern "C" fn _post_return_name(arg0: *mut u8,) {
        $($path_to_types)*::__post_return_name::<$ty>(arg0)
      }
      #[export_name = "component:decoder/decoder#format-signal-value"]
      unsafe extern "C" fn export_format_signal_value(arg0: *mut u8,arg1: usize,) -> *mut u8 {
        $($path_to_types)*::_export_format_signal_value_cabi::<$ty>(arg0, arg1)
      }
      #[export_name = "cabi_post_component:decoder/decoder#format-signal-value"]
      unsafe extern "C" fn _post_return_format_signal_value(arg0: *mut u8,) {
        $($path_to_types)*::__post_return_format_signal_value::<$ty>(arg0)
      }
    };);
  }
                #[doc(hidden)]
                pub(crate) use __export_component_decoder_decoder_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 8]);
            }
        }
    }
}
mod _rt {

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub use alloc_crate::alloc;
    extern crate alloc as alloc_crate;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_component_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::component::decoder::decoder::__export_component_decoder_decoder_cabi!($ty with_types_in $($path_to_types_root)*::exports::component::decoder::decoder);
  )
}
#[doc(inline)]
pub(crate) use __export_component_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:component:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 315] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xbb\x01\x01A\x02\x01\
A\x04\x01B\x02\x01@\x01\x07messages\x01\0\x04\0\x03log\x01\0\x03\x01\x16componen\
t:decoder/host\x05\0\x01B\x06\x01@\0\x01\0\x04\0\x04init\x01\0\x01@\0\0s\x04\0\x04\
name\x01\x01\x01@\x01\x05values\0s\x04\0\x13format-signal-value\x01\x02\x04\x01\x19\
component:decoder/decoder\x05\x01\x04\x01\x1bcomponent:decoder/component\x04\0\x0b\
\x0f\x01\0\x09component\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-c\
omponent\x070.208.1\x10wit-bindgen-rust\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}