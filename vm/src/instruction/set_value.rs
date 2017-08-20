use machine::Machine;
use process::Context;

pub trait SetValue {
    fn set_value(&self, value: i32, vm: &Machine, context: &Context);
    fn set_value_mod(&self, value: i32, vm: &Machine, context: &Context, modulo: usize);
}
