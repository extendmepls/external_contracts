use std::any::Any;

pub struct Context {
    pub name: String,
}

pub trait Plugin: Any + Send + Sync {
    fn run(&self, ctx: &Context);
}
