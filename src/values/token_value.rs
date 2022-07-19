use llvm_sys::core::{LLVMAddIncoming, LLVMCountIncoming, LLVMGetIncomingBlock, LLVMGetIncomingValue};
use llvm_sys::prelude::{LLVMBasicBlockRef, LLVMValueRef};

use std::ffi::CStr;

use crate::basic_block::BasicBlock;
use crate::support::LLVMString;
use crate::values::traits::AsValueRef;
use crate::values::{BasicValue, BasicValueEnum, InstructionValue, Value};

/// A `TokenValue` is used when a value is associated with an instruction but all
/// uses of the value must not attempt to introspect or obscure it
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TokenValue<'ctx> {
    token_value: Value<'ctx>,
}

impl<'ctx> TokenValue<'ctx> {
    pub(crate) unsafe fn new(value: LLVMValueRef) -> Self {
        assert!(!value.is_null());

        TokenValue {
            token_value: Value::new(value),
        }
    }

    /// Gets the name of a `TokenValue`. If the value is a constant, this will
    /// return an empty string.
    pub fn get_name(&self) -> &CStr {
        self.token_value.get_name()
    }

    pub fn set_name(self, name: &str) {
        self.token_value.set_name(name);
    }

    pub fn is_null(self) -> bool {
        self.token_value.is_null()
    }

    pub fn is_undef(self) -> bool {
        self.token_value.is_undef()
    }

    pub fn print_to_string(self) -> LLVMString {
        self.token_value.print_to_string()
    }

    pub fn print_to_stderr(self) {
        self.token_value.print_to_stderr()
    }

    pub fn replace_all_uses_with(self, other: &TokenValue<'ctx>) {
        self.token_value.replace_all_uses_with(other.as_value_ref())
    }
}

unsafe impl AsValueRef for TokenValue<'_> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.token_value.value
    }
}
