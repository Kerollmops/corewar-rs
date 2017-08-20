use machine::Machine;
use process::Context;

pub trait GetValue {
    fn get_value(&self, vm: &Machine, context: &Context) -> i32;
}
