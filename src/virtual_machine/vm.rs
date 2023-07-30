
use crate::virtual_machine::types::Value;
use std::array::from_fn;


pub struct VirtualMachine {
    registers: [Value; 32],
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            registers: from_fn(|_| Value::None),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vm = VirtualMachine::new();
        assert_eq!(vm.registers.len(), 32);
        for register in vm.registers.iter() {
            assert_eq!(*register, Value::None);
        }
    }
}
