use machine::Machine;
use process::Context;

pub trait GetValue {
    fn get_value(&self, vm: &Machine, context: &Context) -> i32;
    fn get_value_mod(&self, vm: &Machine, context: &Context, modulo: usize) -> i32 {
        self.get_value(vm, context)
    }
}
