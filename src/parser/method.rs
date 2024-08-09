/*
    Method
*/

use crate::tokenizer::line::*;
use crate::tokenizer::token::*;

use crate::parser::MemoryCellList;
use crate::parser::getMemoryCellByName;

pub fn test1(methodLink: Arc<RwLock<Method>>, name: &String) -> Option<Arc<RwLock<MemoryCellList>>> {
    let method = methodLink.read().unwrap();
    let parentMethod = &method.parent;
    if let Some(parentBuffer) = parentMethod {
        let parent = parentBuffer.read().unwrap();
        let mcl = parent.mcl.read().unwrap();
        println!("  > parent mcl len: {}", mcl.value.len());
        // todo: search memory cell in mcl
        if let Some(mc) = getMemoryCellByName(parent.mcl.clone(), name) {
            return Some(parent.mcl.clone());
        }
        println!("  * parent: {}", parent.name);
        return test1(parentBuffer.clone(), name);
    }
    None
}

use std::sync::{Arc, RwLock};

pub struct Method {
    pub name:       String,     // unique name
    pub lines:      Vec<Line>,  // nesting lines
    pub parameters: Vec<Token>, // parameters
    pub resultType: String,     // result type
    pub mcl:        Arc<RwLock<MemoryCellList>>,

    pub methods:    Vec<Arc<RwLock<Method>>>,
    pub parent:     Option<Arc<RwLock<Method>>>,
        // if result type = None, => procedure
        // else => function
}
impl Method {
    pub fn new(
        name:   String,
        lines:  Vec<Line>,
        parent: Option<Arc<RwLock<Method>>>,
    ) -> Self {
        Method {
            name,
            lines,
            parameters: Vec::new(),
            resultType: String::from("None"),
            mcl:        Arc::new(RwLock::new(MemoryCellList::new())),
            methods:    Vec::new(),
            parent
        }
    }
}
/*
    pub fn newWithResult(
        name:       String,
        lines:      Vec<Line>,
        resultType: String,
    ) -> Self {
        Method {
            name,
            lines,
            parameters: Vec::new(),
            resultType,
            mcl:        MemoryCellList::new(),
            methods:    Vec::new(),
            //parent:     None,
        }
    }
    pub fn newWithParameters(
        name:       String,
        lines:      Vec<Line>,
        parameters: Vec<Token>,
    ) -> Self {
        Method {
            name,
            lines,
            parameters,
            resultType: String::from("None"),
            mcl:        MemoryCellList::new(),
            methods:    Vec::new(),
            //parent:     None,
        }
    }
    pub fn newFull(
        name:       String,
        lines:      Vec<Line>,
        parameters: Vec<Token>,
        resultType: String,
    ) -> Self {
        Method {
            name,
            lines,
            parameters,
            resultType,
            mcl:        MemoryCellList::new(),
            methods:    Vec::new(),
            //parent:     None,
        }
    }
}
*/