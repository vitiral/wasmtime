//! Data structures to provide transformation of the source
// addresses of a WebAssembly module into the native code.

use cranelift_codegen::ir;
use cranelift_entity::PrimaryMap;
use cranelift_wasm::DefinedFuncIndex;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

/// Single source location to generated address mapping.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstructionAddressMap {
    /// Original source location.
    pub srcloc: ir::SourceLoc,

    /// Generated instructions offset.
    pub code_offset: usize,

    /// Generated instructions length.
    pub code_len: usize,
}

/// Function and its instructions addresses mappings.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionAddressMap {
    /// Instructions maps.
    /// The array is sorted by the InstructionAddressMap::code_offset field.
    pub instructions: Vec<InstructionAddressMap>,

    /// Function start source location (normally declaration).
    pub start_srcloc: ir::SourceLoc,

    /// Function end source location.
    pub end_srcloc: ir::SourceLoc,

    /// Generated function body offset if applicable, otherwise 0.
    pub body_offset: usize,

    /// Generated function body length.
    pub body_len: usize,
}

/// Module functions addresses mappings.
pub type ModuleAddressMap = PrimaryMap<DefinedFuncIndex, FunctionAddressMap>;

/// Value ranges for functions.
pub type ValueLabelsRanges = PrimaryMap<DefinedFuncIndex, cranelift_codegen::ValueLabelsRanges>;

/// Stack slots for functions.
pub type StackSlots = PrimaryMap<DefinedFuncIndex, ir::StackSlots>;

/// Module `vmctx` related info.
pub struct ModuleVmctxInfo {
    /// The memory definition offset in the VMContext structure.
    pub memory_offset: i64,

    /// The functions stack slots.
    pub stack_slots: StackSlots,
}
