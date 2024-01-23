mod internal {

    #![allow(non_snake_case)]
    use objc::{runtime::Object, *};

    use std::ffi::c_void;

    use core_foundation::{base::*, declare_TCFType, impl_TCFType};

    use crate::utils::objc::impl_objc_compatability;

    #[repr(C)]
    pub struct __SCConfigurationRef(c_void);
    extern "C" {
        pub fn SCConfigurationGetTypeID() -> CFTypeID;
    }

    pub type SCConfigurationRef = *mut __SCConfigurationRef;

    declare_TCFType! {SCConfiguration, SCConfigurationRef}
    impl_TCFType!(
        SCConfiguration,
        SCConfigurationRef,
        SCConfigurationGetTypeID
    );
    impl_objc_compatability!(SCConfiguration, __SCConfigurationRef);
    pub(crate) fn init() -> SCConfiguration {
        unsafe {
            let ptr: *mut Object = msg_send![class!(SCConfiguration), alloc];
            let ptr: SCConfigurationRef = msg_send![ptr, init];
            SCConfiguration::wrap_under_create_rule(ptr)
        }
    }
}
pub use internal::SCConfiguration;

impl SCConfiguration {
    pub fn new() -> Self {
        internal::init()
    }
}

impl Default for SCConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
