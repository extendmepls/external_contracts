use std::any::Any;

pub struct Context {
    pub data_example: String,
}

pub trait Plugin: Any + Send + Sync {
    fn name(&self) -> &str;
    fn run(&self, ctx: &Context);
}
