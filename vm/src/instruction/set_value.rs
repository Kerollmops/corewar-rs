use machine::Machine;
use process::Context;

pub trait SetValue {
    fn set_value(&self, value: i32, vm: &mut Machine, context: &Context);
    fn set_value_long(&self, value: i32, vm: &mut Machine, context: &Context);
}
