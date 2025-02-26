use serde::{Deserialize, Serialize};
use crate::error::VitruvianRulesEngineResult;
use super::{index::PakIndex, PakPointer};

//==============================================================================================
//        PakVault
//==============================================================================================

pub struct PakVault {
    chunks : Vec<PakVaultReference>,
    size_in_bytes : u64,
    pak : Vec<u8>,
}

impl PakVault {
    pub fn new() -> Self {
        Self {
            pak : Vec::new(),
            chunks : Vec::new(),
            size_in_bytes : 0,
        }
    }
    
    pub fn pak<T : PakItemDef + PakItemSearchable>(&mut self, item : T) -> VitruvianRulesEngineResult<PakVaultReference> {
        let indices = item.indices();
        let bytes = item.into_bytes()?;
        let pointer = PakPointer::new(self.size_in_bytes, bytes.len() as u64);
        self.size_in_bytes += bytes.len() as u64;
        self.pak.extend(bytes);
        self.chunks.push(PakVaultReference { pointer, indices: indices.clone() });
        Ok(PakVaultReference { pointer, indices })
    }
    
    pub fn unpak<'d, T>(&'d self, pointer : &PakPointer) -> VitruvianRulesEngineResult<T> where T : PakItemRef<'d> {
        let res = T::from_pak(&self.pak, *pointer)?;
        Ok(res)
    }
    
    pub fn size(&self) -> u64 {
        self.size_in_bytes
    }
    
    pub fn len(&self) -> usize {
        self.chunks.len()
    }
}

//==============================================================================================
//        PakVaultReference
//==============================================================================================

pub struct PakVaultReference {
    pub pointer : PakPointer,
    pub indices : Vec<PakIndex>
}

//==============================================================================================
//        PakItem Trait
//==============================================================================================

pub trait PakItemSearchable {
    fn indices(&self) -> Vec<PakIndex>;
}

pub trait PakItemDef {
    fn into_bytes(&self) -> VitruvianRulesEngineResult<Vec<u8>>;
}

pub trait PakItemRef<'de> : Sized {
    fn from_bytes(bytes: &'de [u8]) -> VitruvianRulesEngineResult<Self>;
    
    fn from_pak(bytes: &'de [u8], pointer : PakPointer) -> VitruvianRulesEngineResult<Self> {
        let data = &bytes[pointer.offset as usize..pointer.offset as usize + pointer.size as usize];
        let res = Self::from_bytes(data)?;
        Ok(res)
    }
}

impl <'de, T> PakItemRef<'de> for T where T : Deserialize<'de> {
    fn from_bytes(bytes: &'de [u8]) -> VitruvianRulesEngineResult<Self> {
        let obj : Self = bincode::deserialize::<Self>(bytes).unwrap();
        Ok(obj)
    }
}

impl <T> PakItemDef for T where T : Serialize {
    fn into_bytes(&self) -> VitruvianRulesEngineResult<Vec<u8>> {
        bincode::serialize(self).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod test {

    use serde::{Deserialize, Serialize};
    use super::*;
    
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    struct Person {
        first_name: String,
        last_name: String,
    }
    
    impl PakItemSearchable for Person {
        fn indices(&self) -> Vec<PakIndex> {
            vec![
                PakIndex::new("first_name".to_string(), self.first_name.clone()),
                PakIndex::new("last_name".to_string(), self.last_name.clone())
            ]
        }
    }
    
    #[test]
    fn pak_vault() {
        let person1 = Person {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        
        let person2 = Person {
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
        };
        
        let mut vault = PakVault::new();
        vault.pak(person1.clone()).unwrap();
        vault.pak(person2.clone()).unwrap();
        
        assert_eq!(
            vault.pak,
            vec![
                bincode::serialize(&person1).unwrap(),
                bincode::serialize(&person2).unwrap()
            ].iter().cloned().flatten().collect::<Vec<u8>>()
        );
    }
    
    #[test]
    fn unpak_vault() {
        let person1 = Person {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        
        let person2 = Person {
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
        };
        
        let mut vault = PakVault::new();
        let reference1 = vault.pak(person1.clone()).unwrap();
        let reference2 = vault.pak(person2.clone()).unwrap();
        
        let person1_unpak : Person = vault.unpak(&reference1.pointer).unwrap();
        let person2_unpak : Person = vault.unpak(&reference2.pointer).unwrap();
        
        assert_eq!(person1, person1_unpak);
        assert_eq!(person2, person2_unpak);
    }
}
