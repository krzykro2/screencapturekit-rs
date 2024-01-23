use objc::*;

use crate::{core_media::cm_sample_buffer::CMSampleBuffer, stream::sc_stream::SCStream};

mod internal {

    #![allow(non_snake_case)]

    use std::{error::Error, ffi::c_void, ptr::addr_of, sync::Once};

    use core_foundation::{base::*, declare_TCFType, impl_TCFType};
    use objc::{
        declare::ClassDecl,
        runtime::{Class, Object, Sel},
        *,
    };

    use crate::{
        core_media::cm_sample_buffer::{CMSampleBuffer, CMSampleBufferRef},
        output::sc_stream_output::SCStreamOutputTrait,
        stream::sc_stream::{SCStream, SCStreamRef},
        utils::objc::{create_concrete_from_void, get_concrete_from_void, impl_objc_compatability},
    };

    #[repr(C)]
    pub struct __SCStreamOutputRef(c_void);
    extern "C" {
        pub fn SCStreamOutputGetTypeID() -> CFTypeID;
    }

    pub type SCStreamOutputRef = *mut __SCStreamOutputRef;

    declare_TCFType! {SCStreamOutput, SCStreamOutputRef}
    impl_TCFType!(SCStreamOutput, SCStreamOutputRef, SCStreamOutputGetTypeID);
    impl_objc_compatability!(SCStreamOutput, __SCStreamOutputRef);
    #[repr(C)]
    pub enum SCStreamOutputType {
        Screen,
        Audio,
    }
    unsafe impl objc::Encode for SCStreamOutputType {
        fn encode() -> Encoding {
            i8::encode()
        }
    }

    fn register_objc_class() -> Result<&'static Class, Box<dyn Error>> {
        let mut decl =
            ClassDecl::new("SCStreamOutput", class!(NSObject)).ok_or("Could not register class")?;
        decl.add_ivar::<usize>("_trait");

        extern "C" fn trait_setter(this: &mut Object, _cmd: Sel, sc_stream_delegate_trait: usize) {
            unsafe {
                this.set_ivar::<usize>("_trait", sc_stream_delegate_trait);
            }
        }
        extern "C" fn trait_getter(this: &Object, _cmd: Sel) -> usize {
            unsafe { *this.get_ivar::<usize>("_trait") }
        }
        unsafe {
            let set_trait: extern "C" fn(&mut Object, Sel, usize) = trait_setter;
            let get_trait: extern "C" fn(&Object, Sel) -> usize = trait_getter;
            decl.add_method(sel!(setTrait:), set_trait);
            decl.add_method(sel!(trait), get_trait);

            extern "C" fn stream_output(
                this: &mut Object,
                _cmd: Sel,
                stream_ref: *const c_void,
                sample_buffer_ref: *const c_void,
                of_type: SCStreamOutputType,
            ) {
                unsafe {
                    let ptr = this.get_ivar::<usize>("_trait");
                    let stream: SCStream = get_concrete_from_void(stream_ref);
                    let sample_buffer: CMSampleBuffer = get_concrete_from_void(sample_buffer_ref);
                    let stream_output = addr_of!(ptr) as *mut Box<&dyn SCStreamOutputTrait>;
                    (*stream_output).did_output_sample_buffer(stream, sample_buffer, of_type)
                };
            }
            let stream_output_method: extern "C" fn(
                &mut Object,
                Sel,
                *const c_void,
                *const c_void,
                SCStreamOutputType,
            ) = stream_output;

            decl.add_method(
                sel!(stream:didOutputSampleBuffer:ofType:),
                stream_output_method,
            );
            decl.register();

            Ok(class!(SCStreamOutput))
        }
    }
    pub fn new(sc_stream_output_trait: impl SCStreamOutputTrait) -> SCStreamOutput {
        static REGISTER_CLASS: Once = Once::new();
        REGISTER_CLASS.call_once(|| {
            register_objc_class().expect("Should register SCStreamOutput class");
        });
        unsafe {
            let obj: *mut Object = runtime::class_createInstance(class!(SCStreamOutput), 0);
            let stream_output: &dyn SCStreamOutputTrait = &sc_stream_output_trait;
            let trait_ptr = Box::into_raw(Box::new(stream_output));
            let _: () = msg_send![obj, setTrait: trait_ptr];
            create_concrete_from_void(obj)
        }
    }
}
pub use internal::{SCStreamOutput, SCStreamOutputRef};

pub use internal::SCStreamOutputType;
pub trait SCStreamOutputTrait {
    fn did_output_sample_buffer(
        &self,
        stream: SCStream,
        sample_buffer: CMSampleBuffer,
        of_type: SCStreamOutputType,
    );
}

impl SCStreamOutput {
    pub fn new(stream_output: impl SCStreamOutputTrait) -> Self {
        internal::new(stream_output)
    }
}
