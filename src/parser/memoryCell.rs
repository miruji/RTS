/*
    Memory Cell
*/

use crate::tokenizer::token::*;

#[derive(PartialEq)]
#[derive(Clone)]
pub enum MemoryCellMode {
    LockedFinal,      // variableName
    LockedVariable,   // variableName~
    UnlockedVariable, // variableName~~
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
