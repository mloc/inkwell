use llvm_sys::core::{LLVMAddClause, LLVMIsCleanup, LLVMSetCleanup};
use llvm_sys::prelude::LLVMValueRef;

use std::ffi::CStr;

use crate::values::traits::AsValueRef;
use crate::values::{BasicValue, BasicValueEnum, InstructionValue, Value};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LandingPadValue<'ctx> {
    landing_pad_value: Value<'ctx>,
}

impl<'ctx> LandingPadValue<'ctx> {
    pub(crate) fn new(value: LLVMValueRef) -> Self {
        assert!(!value.is_null());

        LandingPadValue {
            landing_pad_value: Value::new(value),
        }
    }

    pub fn add_clause<BV: BasicValue<'ctx>>(self, clause: BV) {
        unsafe {
            LLVMAddClause(self.as_value_ref(), clause.as_value_ref());
        }
    }

    pub fn is_cleanup(self) -> bool {
        unsafe { LLVMIsCleanup(self.as_value_ref()) == 1 }
    }

    pub fn set_cleanup(self, val: bool) {
        unsafe { LLVMSetCleanup(self.as_value_ref(), val.into()) }
    }

    pub fn get_name(&self) -> &CStr {
        self.landing_pad_value.get_name()
    }

    pub fn set_name(self, name: &str) {
        self.landing_pad_value.set_name(name);
    }

    pub fn as_instruction(self) -> InstructionValue<'ctx> {
        self.landing_pad_value
            .as_instruction()
            .expect("LandingPadValue should always be a LandingPad InstructionValue")
    }

    pub fn as_basic_value(self) -> BasicValueEnum<'ctx> {
        BasicValueEnum::new(self.as_value_ref())
    }
}

impl AsValueRef for LandingPadValue<'_> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.landing_pad_value.value
    }
}
