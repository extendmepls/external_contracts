use std::ffi::OsStr;

pub const PLUGIN_ENTRYPOINT_SYMBOL: &[u8; 27] = b"_lagrappe_plugin_entrypoint";

pub struct PluginContext {
    pub data_example: String,
}

pub trait Plugin: std::any::Any + Send + Sync {
    fn name(&self) -> &str;
    fn run(&self, ctx: &PluginContext);
}

#[macro_export]
macro_rules! declare_app_extend {
    ($app:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _lagrappe_plugin_entrypoint() -> *mut dyn $crate::plugin::Plugin {
            let constructor: fn() -> $app = $constructor;
            let object = constructor();
            let boxed: Box<dyn $crate::plugin::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}

pub type OnChangedCallback = fn();

pub trait AppExtendManager: Sized {
    fn new() -> Self;
    unsafe fn load_extend(&mut self, filename: &OsStr);
}
