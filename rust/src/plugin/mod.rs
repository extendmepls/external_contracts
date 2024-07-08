pub const PLUGIN_ENTRYPOINT_SYMBOL: &'static str = "_lagrappe_plugin_entrypoint";

pub struct PluginContext {
    pub data_example: String,
}

pub trait Plugin: std::any::Any + Send + Sync {
    fn name(&self) -> &str;
    fn run(&self, ctx: &PluginContext);
}

pub struct PluginWrapper(pub Box<dyn Plugin>);

#[macro_export]
macro_rules! declare_app_extend {
    ($app:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _lagrappe_plugin_entrypoint() -> *mut $crate::plugin::PluginWrapper {
            let constructor: fn() -> $app = $constructor;
            let object = constructor();
            let boxed: Box<dyn $crate::plugin::Plugin> = Box::new(object);
            let wrapper = $crate::plugin::PluginWrapper(boxed);
            Box::into_raw(Box::new(wrapper))
        }
    };
}
