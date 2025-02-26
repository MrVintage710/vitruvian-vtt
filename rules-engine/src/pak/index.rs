use std::{cmp::Ordering, collections::BTreeMap, mem};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::PakPointer;

const MAX_PAK_PAGE_SIZE : usize = 32;

//==============================================================================================
//        PakIndexTable
//==============================================================================================


pub struct PakIndexTable<K, V> where K : Serialize + DeserializeOwned, V : Serialize + DeserializeOwned {
    table : BTreeMap<K, V>,
}

impl<K, V> PakIndexTable<K, V> where K : Serialize + DeserializeOwned, V : Serialize + DeserializeOwned {
    pub fn new() -> Self {
        Self {
            table : BTreeMap::new(),
        }
    }
    
    pub fn test(&self) {
        
    }
}

//==============================================================================================
//        PakIndexDef
//==============================================================================================

pub struct PakIndexMap<'m, T> where T : Ord + Serialize {
    data : Vec<PakPage<'m, T>>,
}

//==============================================================================================
//        PakPageDef
//==============================================================================================

pub enum PakPage<'m, T> where T : Ord + Serialize {
    Root {
        values : Vec<PakPageEntry<'m, T>>,
        final_edge: Option<&'m PakPage<'m, T>>,
    },
    Branch {
        values : Vec<PakPageEntry<'m, T>>,
        final_edge : Option<&'m PakPage<'m, T>>,
    },
    Leaf {
        values : Vec<PakPageEntry<'m, T>>
    },
}

impl <'m, T> PakPage<'m, T> where T : Ord + Serialize {
    pub fn new() -> Self {
        Self::Root {
            values : Vec::new(),
            final_edge: None,
        }
    }
    
    pub fn branch() -> Self {
        Self::Branch {
            values : Vec::new(),
            final_edge : None,
        }
    }
    
    pub fn leaf() -> Self {
        Self::Leaf {
            values : Vec::new(),
        }
    }
    
    fn query(&self, key : T) {
        match self {
            PakPage::Root { values, final_edge } => todo!(),
            PakPage::Branch { values, final_edge } => todo!(),
            PakPage::Leaf { values } => todo!(),
        }
    }
    
    fn set_final_edge(&mut self, edge : Option<&'m PakPage<'m, T>>) {
        match self {
            PakPage::Root { values, final_edge } => *final_edge = edge,
            PakPage::Branch { values, final_edge } => *final_edge = edge,
            PakPage::Leaf { values } => {}
        }
    }
    
    fn set_entries(&mut self, entries : Vec<PakPageEntry<'m, T>>) {
        match self {
            PakPage::Root { values, final_edge } => *values = entries,
            PakPage::Branch { values, final_edge } => *values = entries,
            PakPage::Leaf { values } => *values = entries,
        }
    }
    
    fn into_branch(&mut self) {
        let prev = mem::replace(self, Self::branch());
        match prev {
            PakPage::Root { values, final_edge } => {self.set_entries(values); self.set_final_edge(final_edge);},
            PakPage::Branch { values, final_edge } => {self.set_entries(values); self.set_final_edge(final_edge);},
            PakPage::Leaf { values } => todo!(),
        }
    }
    
    pub fn is_branch(&self) -> bool {
        match self {
            PakPage::Root { values, final_edge : _ } => false,
            PakPage::Branch { values, final_edge : _ } => true,
            PakPage::Leaf { values } => false,
        }
    }
    
    pub fn is_leaf(&self) -> bool {
        match self {
            PakPage::Root { values, final_edge : _ } => false,
            PakPage::Branch { values, final_edge : _ } => false,
            PakPage::Leaf { values } => true,
        }
    }
    
    pub fn insert(&mut self, key : T, pointer : PakPointer, max_size : usize) -> Option<Vec<PakPage<'m, T>>> {
        match self {
            PakPage::Root { values, final_edge } => {
                values.push(PakPageEntry::new(key, pointer));
                values.sort();
                if values.len() > max_size {
                    Some(self.split(max_size))
                } else {
                    None
                }
            },
            PakPage::Branch { values, final_edge } => todo!(),
            PakPage::Leaf { values } => todo!(),
        }
    }
    
    pub fn split(&mut self, max_size : usize) -> Vec<PakPage<'m, T>> {
        if self.is_leaf() {
            let prev = mem::replace(self, Self::branch());
        }
        
        todo!()
    }
}

//==============================================================================================
//        PakPageEntry
//==============================================================================================

pub struct PakPageEntry<'m, T> where T : Ord + Serialize{
    key : T,
    pointer : PakPointer,
    next : Option<&'m PakPage<'m, T>>
}

impl<'m, T> PakPageEntry<'m, T> where T : Ord + Serialize {
    pub fn new(key : T, pointer : PakPointer) -> Self {
        Self {
            key,
            pointer,
            next : None
        }
    }
    
    pub fn add_next(&mut self, next : &'m PakPage<'m, T>) {
        self.next = Some(next);
    }
}

impl <'m, T> PartialEq for PakPageEntry<'m, T> where T : Ord + Serialize {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl <'m, T> Eq for PakPageEntry<'m, T> where T : Ord + Serialize {}

impl <'m, T> PartialOrd for PakPageEntry<'m, T> where T : Ord + Serialize {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl <'m, T> Ord for PakPageEntry<'m, T> where T : Ord + Serialize {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

//==============================================================================================
//        PakIndex
//==============================================================================================

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
pub struct PakIndex {
    key : String,
    value : String,
}

impl PakIndex {
    pub fn new(key : String, value : String) -> Self {
        Self {
            key,
            value,
        }
    }
}

#[cfg(test)]
mod test {
    
    use super::*;
    
    #[test]
    fn sort_indices() {
        let mut indices = vec![
            PakIndex::new("b".to_string(), "2".to_string()),
            PakIndex::new("a".to_string(), "1".to_string()),
            PakIndex::new("c".to_string(), "3".to_string()),
        ];
        
        indices.sort();
        
        assert_eq!(indices, vec![
            PakIndex::new("a".to_string(), "1".to_string()),
            PakIndex::new("b".to_string(), "2".to_string()),
            PakIndex::new("c".to_string(), "3".to_string()),
        ]);
    }
    
    #[test]
    fn into_branch() {
        let mut page : PakPage<u8> = PakPage::new();
        page.insert(0, PakPointer::default(), 4);
        page.insert(2, PakPointer::default(), 4);
        page.insert(4, PakPointer::default(), 4);
        
        page.into_branch();
        
        assert!(page.is_branch());
    }
}
