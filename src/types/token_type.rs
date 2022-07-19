use llvm_sys::prelude::LLVMTypeRef;

use crate::context::ContextRef;
use crate::support::LLVMString;
use crate::types::traits::AsTypeRef;
use crate::types::{BasicMetadataTypeEnum, FunctionType, Type};

/// A `TokenType` is used when a value is associated with an instruction but all
/// uses of the value must not attempt to introspect or obscure it
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TokenType<'ctx> {
    token_type: Type<'ctx>,
}

impl<'ctx> TokenType<'ctx> {
    pub(crate) unsafe fn new(token_type: LLVMTypeRef) -> Self {
        assert!(!token_type.is_null());

        TokenType {
            token_type: Type::new(token_type),
        }
    }

    /// Gets a reference to the `Context` this `TokenType` was created in.
    pub fn get_context(self) -> ContextRef<'ctx> {
        self.token_type.get_context()
    }

    /// Creates a `FunctionType` with this `TokenType` for its return type.
    pub fn fn_type(self, param_types: &[BasicMetadataTypeEnum<'ctx>], is_var_args: bool) -> FunctionType<'ctx> {
        self.token_type.fn_type(param_types, is_var_args)
    }

    /// Prints the definition of a `TokenType` to a `LLVMString`.
    pub fn print_to_string(self) -> LLVMString {
        self.token_type.print_to_string()
    }
}

unsafe impl AsTypeRef for TokenType<'_> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        self.token_type.ty
    }
}
