/*
    Memory Cell
*/

use crate::tokenizer::token::*;

// MemoryCellMode
#[derive(PartialEq)]
#[derive(Clone)]
pub enum MemoryCellMode {
    LockedFinal,      // memoryCellName
//  LockedConst ?
    LockedVariable,   // memoryCellName~
    UnlockedVariable, // memoryCellName~~
}
impl ToString for MemoryCellMode {
    fn to_string(&self) -> String {
        match self {
            // basic
            MemoryCellMode::LockedFinal      => String::from("Locked Final"),
            MemoryCellMode::LockedVariable   => String::from("Locked Variable"),
            MemoryCellMode::UnlockedVariable => String::from("Unlocked Variable")
        }
    }
}
// MemoryCell
#[derive(Clone)]
pub struct MemoryCell {
    pub name:       String,         // unique name
    pub mode:       MemoryCellMode, // mode
    pub valueType:  TokenType,      // type
    pub value:      Token           // value
}
impl MemoryCell {
    pub fn new(
        name:       String,
        mode:       MemoryCellMode,
        valueType:  TokenType,
        value:      Token
    ) -> Self {
        MemoryCell {
            name,
            mode,
            valueType,
            value
        }
    }
}
