use virtual_machine::VirtualMachine;
use process::Context;

pub trait GetValue {
    fn get_value(&self, vm: &VirtualMachine, context: &Context) -> i32;
}
