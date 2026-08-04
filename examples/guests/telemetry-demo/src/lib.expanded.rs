#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
#[allow(dead_code, clippy::all)]
pub mod exports {
    pub mod local {
        pub mod telemetry_demo {
            #[allow(dead_code, async_fn_in_trait, unused_imports,
            clippy::all)]
            pub mod processor {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                /// A structured data payload
                pub struct SensorReading {
                    pub sensor_id: _rt::String,
                    pub value: f64,
                    pub timestamp: u64,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for SensorReading {
                    #[inline]
                    fn clone(&self) -> SensorReading {
                        SensorReading {
                            sensor_id: ::core::clone::Clone::clone(&self.sensor_id),
                            value: ::core::clone::Clone::clone(&self.value),
                            timestamp: ::core::clone::Clone::clone(&self.timestamp),
                        }
                    }
                }
                impl ::core::fmt::Debug for SensorReading {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>)
                        -> ::core::fmt::Result {
                        f.debug_struct("SensorReading").field("sensor-id",
                                        &self.sensor_id).field("value",
                                    &self.value).field("timestamp", &self.timestamp).finish()
                    }
                }
                /// A structured error definition
                pub enum ProcessingError {
                    CorruptedData(_rt::String),
                    SensorOffline,
                    LimitExceeded,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ProcessingError {
                    #[inline]
                    fn clone(&self) -> ProcessingError {
                        match self {
                            ProcessingError::CorruptedData(__self_0) =>
                                ProcessingError::CorruptedData(::core::clone::Clone::clone(__self_0)),
                            ProcessingError::SensorOffline =>
                                ProcessingError::SensorOffline,
                            ProcessingError::LimitExceeded =>
                                ProcessingError::LimitExceeded,
                        }
                    }
                }
                impl ::core::fmt::Debug for ProcessingError {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>)
                        -> ::core::fmt::Result {
                        match self {
                            ProcessingError::CorruptedData(e) => {
                                f.debug_tuple("ProcessingError::CorruptedData").field(e).finish()
                            }
                            ProcessingError::SensorOffline => {
                                f.debug_tuple("ProcessingError::SensorOffline").finish()
                            }
                            ProcessingError::LimitExceeded => {
                                f.debug_tuple("ProcessingError::LimitExceeded").finish()
                            }
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case, unused_unsafe)]
                pub unsafe fn _export_analyze_batch_cabi<T_: Guest>(arg0: i32)
                    -> i32 {
                    unsafe {
                        _rt::run_ctors_once();
                        wit_bindgen::rt::async_support::start_task(async move
                                {
                                let _task_cancel =
                                    wit_bindgen::rt::async_support::TaskCancelOnDrop::new();
                                let result0 =
                                    &{
                                            T_::analyze_batch(wit_bindgen::rt::async_support::StreamReader::new(arg0
                                                            as u32,
                                                        &<SensorReading as
                                                                super::super::super::super::wit_stream::StreamPayload>::VTABLE)).await
                                        };
                                #[link(wasm_import_module =
                                "[export]local:telemetry-demo/processor@0.1.0")]
                                unsafe extern "C" {
                                    #[link_name = "[task-return]analyze-batch"]
                                    fn wit_import1(_: i32);
                                }
                                _task_cancel.forget();
                                wit_import1((result0).take_handle() as i32);
                            })
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __callback_analyze_batch(event0: u32,
                    event1: u32, event2: u32) -> u32 {
                    unsafe {
                        wit_bindgen::rt::async_support::callback(event0, event1,
                            event2)
                    }
                }
                pub trait Guest {
                    /// Streams a batch of readings, returns a future containing a Result
                    #[allow(async_fn_in_trait)]
                    async fn analyze_batch(readings:
                        wit_bindgen::rt::async_support::StreamReader<SensorReading>)
                    ->
                        wit_bindgen::rt::async_support::FutureReader<Result<_rt::String,
                        ProcessingError>>;
                }
                #[doc(hidden)]
                macro_rules! __export_local_telemetry_demo_processor_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) =>
                    (const _: () =
                    {
                        #[unsafe(export_name =
                        "[async-lift]local:telemetry-demo/processor@0.1.0#analyze-batch")]
                        unsafe extern "C" fn export_analyze_batch(arg0: i32,) -> i32
                        {
                            unsafe
                            {
                                $($path_to_types)*::_export_analyze_batch_cabi::<$ty>(arg0)
                            }
                        }
                        #[unsafe(export_name =
                        "[callback][async-lift]local:telemetry-demo/processor@0.1.0#analyze-batch")]
                        unsafe extern "C" fn
                        _callback_analyze_batch(event0: u32, event1: u32, event2:
                        u32) -> u32
                        {
                            unsafe
                            {
                                $($path_to_types)*::__callback_analyze_batch(event0, event1,
                                event2)
                            }
                        }
                    };);
                }
                #[doc(hidden)]
                pub(crate) use __export_local_telemetry_demo_processor_0_1_0_cabi;
            }
        }
    }
}
mod _rt {
    #![allow(dead_code, unused_imports, clippy::all)]
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if true {
            String::from_utf8(bytes).unwrap()
        } else { unsafe { String::from_utf8_unchecked(bytes) } }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 { return; }
        unsafe {
            let layout =
                alloc::Layout::from_size_align_unchecked(size, align);
            alloc::dealloc(ptr, layout);
        }
    }
    pub fn as_f64<T: AsF64>(t: T) -> f64 { t.as_f64() }
    pub trait AsF64 {
        fn as_f64(self)
        -> f64;
    }
    impl<'a, T: Copy + AsF64> AsF64 for &'a T {
        fn as_f64(self) -> f64 { (*self).as_f64() }
    }
    impl AsF64 for f64 {
        #[inline]
        fn as_f64(self) -> f64 { self as f64 }
    }
    pub fn as_i64<T: AsI64>(t: T) -> i64 { t.as_i64() }
    pub trait AsI64 {
        fn as_i64(self)
        -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 { (*self).as_i64() }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 { self as i64 }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 { self as i64 }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if true {



            // Check for custom error triggers




            {
                ::core::panicking::panic_fmt(format_args!("invalid enum discriminant"));
            }
        } else { unsafe { core::hint::unreachable_unchecked() } }
    }
    pub fn run_ctors_once() { wit_bindgen::rt::run_ctors_once(); }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
pub mod wit_future {
    #![allow(dead_code, unused_variables, clippy::all)]
    #[doc(hidden)]
    pub trait FuturePayload: Unpin + Sized + 'static {
        const VTABLE:
            &'static wit_bindgen::rt::async_support::FutureVtable<Self>;
    }
    #[doc(hidden)]
    #[allow(unused_unsafe)]
    pub mod vtable0 {
        #[link(wasm_import_module =
        "[export]local:telemetry-demo/processor@0.1.0")]
        unsafe extern "C" {
            #[link_name = "[future-new-1]analyze-batch"]
            fn new()
            -> u64;
            #[link_name = "[future-cancel-write-1]analyze-batch"]
            fn cancel_write(_: u32)
            -> u32;
            #[link_name = "[future-cancel-read-1]analyze-batch"]
            fn cancel_read(_: u32)
            -> u32;
            #[link_name = "[future-drop-writable-1]analyze-batch"]
            fn drop_writable(_: u32);
            #[link_name = "[future-drop-readable-1]analyze-batch"]
            fn drop_readable(_: u32);
            #[link_name = "[async-lower][future-read-1]analyze-batch"]
            fn start_read(_: u32, _: *mut u8)
            -> u32;
            #[link_name = "[async-lower][future-write-1]analyze-batch"]
            fn start_write(_: u32, _: *const u8)
            -> u32;
        }
        unsafe fn lift(ptr: *mut u8)
            ->
                Result<super::super::_rt::String,
                super::super::exports::local::telemetry_demo::processor::ProcessingError> {
            unsafe {
                let l0 = i32::from(*ptr.add(0).cast::<u8>());
                match l0 {
                    0 => {
                        let e =
                            {
                                let l1 =
                                    *ptr.add(::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                                let l2 =
                                    *ptr.add(2 *
                                                    ::core::mem::size_of::<*const u8>()).cast::<usize>();
                                let len3 = l2;
                                let bytes3 =
                                    super::super::_rt::Vec::from_raw_parts(l1.cast(), len3,
                                        len3);
                                super::super::_rt::string_lift(bytes3)
                            };
                        Ok(e)
                    }
                    1 => {
                        let e =
                            {
                                let l4 =
                                    i32::from(*ptr.add(::core::mem::size_of::<*const u8>()).cast::<u8>());
                                use super::super::exports::local::telemetry_demo::processor::ProcessingError as V8;
                                let v8 =
                                    match l4 {
                                        0 => {
                                            let e8 =
                                                {
                                                    let l5 =
                                                        *ptr.add(2 *
                                                                        ::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                                                    let l6 =
                                                        *ptr.add(3 *
                                                                        ::core::mem::size_of::<*const u8>()).cast::<usize>();
                                                    let len7 = l6;
                                                    let bytes7 =
                                                        super::super::_rt::Vec::from_raw_parts(l5.cast(), len7,
                                                            len7);
                                                    super::super::_rt::string_lift(bytes7)
                                                };
                                            V8::CorruptedData(e8)
                                        }
                                        1 => { V8::SensorOffline }
                                        n => {
                                            if true {
                                                {
                                                    match (&n, &2) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(kind, &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(format_args!("invalid enum discriminant")));
                                                            }
                                                        }
                                                    }
                                                };
                                            };
                                            V8::LimitExceeded
                                        }
                                    };
                                v8
                            };
                        Err(e)
                    }
                    _ => super::super::_rt::invalid_enum_discriminant(),
                }
            }
        }
        unsafe fn lower(value:
                Result<super::super::_rt::String,
                super::super::exports::local::telemetry_demo::processor::ProcessingError>,
            ptr: *mut u8) {
            unsafe {
                match value {
                    Ok(e) => {
                        {
                            *ptr.add(0).cast::<u8>() = (0i32) as u8;
                            let vec0 = (e.into_bytes()).into_boxed_slice();
                            let ptr0 = vec0.as_ptr().cast::<u8>();
                            let len0 = vec0.len();
                            ::core::mem::forget(vec0);
                            *ptr.add(2 *
                                                ::core::mem::size_of::<*const u8>()).cast::<usize>() = len0;
                            *ptr.add(::core::mem::size_of::<*const u8>()).cast::<*mut u8>()
                                = ptr0.cast_mut();
                        }
                    }
                    Err(e) => {
                        {
                            *ptr.add(0).cast::<u8>() = (1i32) as u8;
                            use super::super::exports::local::telemetry_demo::processor::ProcessingError as V2;
                            match e {
                                V2::CorruptedData(e) => {
                                    *ptr.add(::core::mem::size_of::<*const u8>()).cast::<u8>() =
                                        (0i32) as u8;
                                    let vec1 = (e.into_bytes()).into_boxed_slice();
                                    let ptr1 = vec1.as_ptr().cast::<u8>();
                                    let len1 = vec1.len();
                                    ::core::mem::forget(vec1);
                                    *ptr.add(3 *
                                                        ::core::mem::size_of::<*const u8>()).cast::<usize>() = len1;
                                    *ptr.add(2 *
                                                        ::core::mem::size_of::<*const u8>()).cast::<*mut u8>() =
                                        ptr1.cast_mut();
                                }
                                V2::SensorOffline => {
                                    {
                                        *ptr.add(::core::mem::size_of::<*const u8>()).cast::<u8>() =
                                            (1i32) as u8;
                                    }
                                }
                                V2::LimitExceeded => {
                                    {
                                        *ptr.add(::core::mem::size_of::<*const u8>()).cast::<u8>() =
                                            (2i32) as u8;
                                    }
                                }
                            }
                        }
                    }
                };
            }
        }
        unsafe fn dealloc_lists(ptr: *mut u8) {
            unsafe {
                let l0 = i32::from(*ptr.add(0).cast::<u8>());
                match l0 {
                    0 => {
                        let l1 =
                            *ptr.add(::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                        let l2 =
                            *ptr.add(2 *
                                            ::core::mem::size_of::<*const u8>()).cast::<usize>();
                        super::super::_rt::cabi_dealloc(l1, l2, 1);
                    }
                    _ => {
                        let l3 =
                            i32::from(*ptr.add(::core::mem::size_of::<*const u8>()).cast::<u8>());
                        match l3 {
                            0 => {
                                let l4 =
                                    *ptr.add(2 *
                                                    ::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                                let l5 =
                                    *ptr.add(3 *
                                                    ::core::mem::size_of::<*const u8>()).cast::<usize>();
                                super::super::_rt::cabi_dealloc(l4, l5, 1);
                            }
                            1 => (),
                            _ => (),
                        }
                    }
                }
            }
        }
        pub static VTABLE:
            wit_bindgen::rt::async_support::FutureVtable<Result<super::super::_rt::String,
            super::super::exports::local::telemetry_demo::processor::ProcessingError>>
            =
            wit_bindgen::rt::async_support::FutureVtable::<Result<super::super::_rt::String,
                super::super::exports::local::telemetry_demo::processor::ProcessingError>> {
                cancel_write,
                cancel_read,
                drop_writable,
                drop_readable,
                dealloc_lists,
                layout: unsafe {
                    ::core::alloc::Layout::from_size_align_unchecked(16, 4)
                },
                lift,
                lower,
                new,
                start_read,
                start_write,
            };
        impl super::FuturePayload for
            Result<super::super::_rt::String,
            super::super::exports::local::telemetry_demo::processor::ProcessingError>
            {
            const VTABLE:
                &'static wit_bindgen::rt::async_support::FutureVtable<Self> =
                &VTABLE;
        }
    }
    /// Creates a new Component Model `future` with the specified payload type.
    ///
    /// The `default` function provided computes the default value to be sent in
    /// this future if no other value was otherwise sent.
    pub fn new<T: FuturePayload>(default: fn() -> T)
        ->
            (wit_bindgen::rt::async_support::FutureWriter<T>,
            wit_bindgen::rt::async_support::FutureReader<T>) {
        unsafe {
            wit_bindgen::rt::async_support::future_new::<T>(default,
                T::VTABLE)
        }
    }
}
pub mod wit_stream {
    #![allow(dead_code, unused_variables, clippy::all)]
    pub trait StreamPayload: Unpin + Sized + 'static {
        const VTABLE:
            &'static wit_bindgen::rt::async_support::StreamVtable<Self>;
    }
    #[doc(hidden)]
    #[allow(unused_unsafe)]
    pub mod vtable0 {
        #[link(wasm_import_module =
        "[export]local:telemetry-demo/processor@0.1.0")]
        unsafe extern "C" {
            #[link_name = "[stream-new-0]analyze-batch"]
            fn new()
            -> u64;
            #[link_name = "[stream-cancel-write-0]analyze-batch"]
            fn cancel_write(_: u32)
            -> u32;
            #[link_name = "[stream-cancel-read-0]analyze-batch"]
            fn cancel_read(_: u32)
            -> u32;
            #[link_name = "[stream-drop-writable-0]analyze-batch"]
            fn drop_writable(_: u32);
            #[link_name = "[stream-drop-readable-0]analyze-batch"]
            fn drop_readable(_: u32);
            #[link_name = "[async-lower][stream-read-0]analyze-batch"]
            fn start_read(_: u32, _: *mut u8, _: usize)
            -> u32;
            #[link_name = "[async-lower][stream-write-0]analyze-batch"]
            fn start_write(_: u32, _: *const u8, _: usize)
            -> u32;
        }
        unsafe fn lift(ptr: *mut u8)
            ->
                super::super::exports::local::telemetry_demo::processor::SensorReading {
            unsafe {
                let l0 = *ptr.add(0).cast::<*mut u8>();
                let l1 =
                    *ptr.add(::core::mem::size_of::<*const u8>()).cast::<usize>();
                let len2 = l1;
                let bytes2 =
                    super::super::_rt::Vec::from_raw_parts(l0.cast(), len2,
                        len2);
                let l3 =
                    *ptr.add(2 *
                                    ::core::mem::size_of::<*const u8>()).cast::<f64>();
                let l4 =
                    *ptr.add(8 +
                                    2 * ::core::mem::size_of::<*const u8>()).cast::<i64>();
                super::super::exports::local::telemetry_demo::processor::SensorReading {
                    sensor_id: super::super::_rt::string_lift(bytes2),
                    value: l3,
                    timestamp: l4 as u64,
                }
            }
        }
        unsafe fn lower(value:
                super::super::exports::local::telemetry_demo::processor::SensorReading,
            ptr: *mut u8) {
            unsafe {
                let super::super::exports::local::telemetry_demo::processor::SensorReading {
                        sensor_id: sensor_id0, value: value0, timestamp: timestamp0
                        } = value;
                let vec1 = (sensor_id0.into_bytes()).into_boxed_slice();
                let ptr1 = vec1.as_ptr().cast::<u8>();
                let len1 = vec1.len();
                ::core::mem::forget(vec1);
                *ptr.add(::core::mem::size_of::<*const u8>()).cast::<usize>()
                    = len1;
                *ptr.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                *ptr.add(2 *
                                    ::core::mem::size_of::<*const u8>()).cast::<f64>() =
                    super::super::_rt::as_f64(value0);
                *ptr.add(8 +
                                    2 * ::core::mem::size_of::<*const u8>()).cast::<i64>() =
                    super::super::_rt::as_i64(timestamp0);
            }
        }
        unsafe fn dealloc_lists(ptr: *mut u8) {
            unsafe {
                let l0 = *ptr.add(0).cast::<*mut u8>();
                let l1 =
                    *ptr.add(::core::mem::size_of::<*const u8>()).cast::<usize>();
                super::super::_rt::cabi_dealloc(l0, l1, 1);
            }
        }
        pub static VTABLE:
            wit_bindgen::rt::async_support::StreamVtable<super::super::exports::local::telemetry_demo::processor::SensorReading>
            =
            wit_bindgen::rt::async_support::StreamVtable::<super::super::exports::local::telemetry_demo::processor::SensorReading> {
                cancel_write,
                cancel_read,
                drop_writable,
                drop_readable,
                dealloc_lists: Some(dealloc_lists),
                layout: unsafe {
                    ::core::alloc::Layout::from_size_align_unchecked(24, 8)
                },
                lift: Some(lift),
                lower: Some(lower),
                new,
                start_read,
                start_write,
            };
        impl super::StreamPayload for
            super::super::exports::local::telemetry_demo::processor::SensorReading
            {
            const VTABLE:
                &'static wit_bindgen::rt::async_support::StreamVtable<Self> =
                &VTABLE;
        }
    }
    /// Creates a new Component Model `stream` with the specified payload type.
    pub fn new<T: StreamPayload>()
        ->
            (wit_bindgen::rt::async_support::StreamWriter<T>,
            wit_bindgen::rt::async_support::StreamReader<T>) {
        unsafe { wit_bindgen::rt::async_support::stream_new::<T>(T::VTABLE) }
    }
}
/// Generates `#[unsafe(no_mangle)]` functions to export the specified type as
/// the root implementation of all generated traits.
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
macro_rules! __export_telemetry_world_impl {
    ($ty:ident) => (self::export!($ty with_types_in self););
    ($ty:ident with_types_in $($path_to_types_root:tt)*) =>
    ($($path_to_types_root)*::exports::local::telemetry_demo::processor::__export_local_telemetry_demo_processor_0_1_0_cabi!($ty
    with_types_in
    $($path_to_types_root)*::exports::local::telemetry_demo::processor);)
}
#[doc(inline)]
pub(crate) use __export_telemetry_world_impl as export;
#[unsafe(link_section =
"component-type:wit-bindgen:0.60.0:local:telemetry-demo@0.1.0:telemetry-world:encoded world")]
#[doc(hidden)]
#[allow(clippy::octal_escapes)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 407] =
    *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x91\x02\x01A\x02\x01\
A\x02\x01B\x09\x01r\x03\x09sensor-ids\x05valueu\x09timestampw\x04\0\x0esensor-re\
ading\x03\0\0\x01q\x03\x0ecorrupted-data\x01s\0\x0esensor-offline\0\0\x0elimit-e\
xceeded\0\0\x04\0\x10processing-error\x03\0\x02\x01f\x01\x01\x01j\x01s\x01\x03\x01\
e\x01\x05\x01C\x01\x08readings\x04\0\x06\x04\0\x0danalyze-batch\x01\x07\x04\0$lo\
cal:telemetry-demo/processor@0.1.0\x05\0\x04\0*local:telemetry-demo/telemetry-wo\
rld@0.1.0\x04\0\x0b\x15\x01\0\x0ftelemetry-world\x03\0\0\0G\x09producers\x01\x0c\
processed-by\x02\x0dwit-component\x070.254.0\x10wit-bindgen-rust\x060.60.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen::rt::maybe_link_cabi_realloc();
}
const _: &[u8] =
    b"package local:telemetry-demo@0.1.0;\n\ninterface processor {\n    // A structured data payload\n    record sensor-reading {\n        sensor-id: string,\n        value: f64,\n        timestamp: u64,\n    }\n\n    // A structured error definition\n    variant processing-error {\n        corrupted-data(string),\n        sensor-offline,\n        limit-exceeded,\n    }\n\n    // Streams a batch of readings, returns a future containing a Result\n    analyze-batch: async func(\n        readings: stream<sensor-reading>\n    ) -> future<result<string, processing-error>>;\n}\n\nworld telemetry-world {\n    export processor;\n}\n";
use exports::local::telemetry_demo::processor::{
    Guest, SensorReading, ProcessingError,
};
use futures::StreamExt;
struct Component;
impl Guest for Component {
    async fn analyze_batch(mut readings:
            wit_bindgen::rt::Stream<SensorReading>)
        -> Result<String, ProcessingError> {
        let mut count = 0;
        let mut total_value = 0.0;
        while let Some(reading) = readings.next().await {
            if reading.value < 0.0 {
                return Err(ProcessingError::CorruptedData(::alloc::__export::must_use({
                                    ::alloc::fmt::format(format_args!("Negative reading from sensor {0}",
                                            reading.sensor_id))
                                })));
            }
            if reading.value > 1000.0 {
                return Err(ProcessingError::LimitExceeded);
            }
            total_value += reading.value;
            count += 1;
        }
        if count == 0 { return Err(ProcessingError::SensorOffline); }
        let average = total_value / (count as f64);
        Ok(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("Successfully analyzed {0} items. Avg value: {1:.2}",
                            count, average))
                }))
    }
}
const _: () =
    {
        #[unsafe(export_name =
        "[async-lift]local:telemetry-demo/processor@0.1.0#analyze-batch")]
        unsafe extern "C" fn export_analyze_batch(arg0: i32) -> i32 {
            unsafe {
                self::exports::local::telemetry_demo::processor::_export_analyze_batch_cabi::<Component>(arg0)
            }
        }
        #[unsafe(export_name =
        "[callback][async-lift]local:telemetry-demo/processor@0.1.0#analyze-batch")]
        unsafe extern "C" fn _callback_analyze_batch(event0: u32, event1: u32,
            event2: u32) -> u32 {
            unsafe {
                self::exports::local::telemetry_demo::processor::__callback_analyze_batch(event0,
                    event1, event2)
            }
        }
    };
