use machine::Machine;
use process::Context;

pub trait GetValue {
    fn get_value(&self, vm: &Machine, context: &Context) -> i32;
    fn get_value_long(&self, vm: &Machine, context: &Context) -> i32 {
        self.get_value(vm, context)
    }
}
