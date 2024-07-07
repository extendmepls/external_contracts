use std::any::Any;

pub struct Context {
    pub data_example: String,
}

pub trait Plugin: Any + Send + Sync {
    fn name(&self) -> &str;
    fn run(&self, ctx: &Context);
}

pub struct PluginWrapper(pub Box<dyn Plugin>);

#[macro_export]
macro_rules! declare_app_extend {
    ($app:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _app_extend_create() -> *mut $crate::PluginWrapper {
            let constructor: fn() -> $app = $constructor;
            let object = constructor();
            let boxed: Box<dyn $crate::Plugin> = Box::new(object);
            let wrapper = external_contracts::PluginWrapper(boxed);
            Box::into_raw(Box::new(wrapper))
        }
    };
}
