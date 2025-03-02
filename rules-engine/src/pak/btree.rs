use std::{cmp::Ordering, collections::VecDeque, fmt::Debug, path::Display, rc::Rc};

use super::{value::PakValue, PakPointer};


//==============================================================================================
//        PakTree
//==============================================================================================

/// This is a pretty limited b-tree implementation. Since the end product is a read only datastructure, deletion is not supported. This
/// has been optimized for read performance and memory efficiency.
#[derive(Debug)]
pub struct PakTree {
    pages : Vec<PakTreePage>,
    max_size: usize,
}

impl PakTree {
    pub fn new(power_of_two: u32) -> Self {
        PakTree {
            pages: vec![PakTreePage::new()],
            max_size : 2usize.pow(power_of_two),
        }
    }
    
    pub fn access<'t>(&'t mut self) -> PakTreeAccess<'t> {
        PakTreeAccess {
            current: 0,
            table: self,
            trail : VecDeque::new()
        }
    }
}

//==============================================================================================
//        PakTreeAccess
//==============================================================================================

pub struct PakTreeAccess<'t> {
    table: &'t mut PakTree,
    current: usize,
    trail : VecDeque<usize>
}

impl PakTreeAccess<'_> {    
    // fn root(&mut self) -> &mut Self {
    //     self.current = self.table.root;
    //     self
    // }
    
    fn goto(&mut self, page: usize) -> &mut Self {
        self.trail.push_front(self.current);
        self.current = page;
        self
    }
    
    fn back(&mut self) -> &mut Self {
        let front = self.trail.pop_front();
        if let Some(index) = front {
            self.current = index;
        }
        self
    }
    
    fn current(&self) -> &PakTreePage {
        &self.table.pages[self.current]
    }
    
    fn current_mut(&mut self) -> &mut PakTreePage {
        &mut self.table.pages[self.current]
    }
    
    fn push(&mut self, entry : PakTreePageEntry) -> PakTreeStatus{
        self.current_mut().push(entry)
    }
    
    fn insert_entry(&mut self, entry : PakTreePageEntry) -> usize {
        let mut result = self.push(entry);
        while let PakTreeStatus::Next(index, entry) = result {
            self.goto(index);
            result = self.push(entry);
        }
        
        let index = match result {
            PakTreeStatus::Ok(index) => index,
            PakTreeStatus::Next(_, _) => 0,
        };
        
        if self.current().values.len() > self.table.max_size {
            self.split();
        }
        
        index
    }
    
    pub fn insert<K>(&mut self, key: K, value: PakPointer) -> &mut Self where K: Into<PakValue> {
        self.insert_entry(PakTreePageEntry::new(key.into(), value));
        
        println!("{:?} \n", self.table);
        self
    }
    
    fn split(&mut self) {
        let mut leading_entries = VecDeque::new();
        let mut trailing_entries = VecDeque::new();
        
        let half_max = self.table.max_size / 2;
        
        for _ in 0..half_max {
            let current = self.current_mut();
            leading_entries.push_back(current.values.pop_front().unwrap());
            trailing_entries.push_front(current.values.pop_back().unwrap());
        }
        
        let mut middle_entry = self.current_mut().values.pop_front().unwrap();
        self.current_mut().values = trailing_entries;
        let leading_index = self.new_page(leading_entries);
        middle_entry.previous = Some(leading_index);
        
        self.back();
        
        self.insert_entry(middle_entry);
    }
    
    fn new_page(&mut self, values : VecDeque<PakTreePageEntry>) -> usize {
        let index = self.table.pages.len();
        self.table.pages.push(PakTreePage::new_with_entries(values));
        index
    }
}

//==============================================================================================
//        PakTreePage
//==============================================================================================

#[derive(Debug)]
struct PakTreePage {
    values: VecDeque<PakTreePageEntry>,
    next: Option<usize>,
}

impl PakTreePage {
    fn new() -> Self {
        PakTreePage {
            values: VecDeque::new(),
            next: None,
        }
    }
    
    fn new_with_entries(entries: VecDeque<PakTreePageEntry>) -> Self {
        PakTreePage {
            values: entries,
            next: None,
        }
    }
    
    fn push(&mut self, mut e : PakTreePageEntry) -> PakTreeStatus {
        for (index, entry) in self.values.iter_mut().enumerate() {
            match entry.key.cmp(&e.key) {
                Ordering::Less => continue,
                Ordering::Greater => match entry.previous {
                        Some(next) => return PakTreeStatus::Next(next, e),
                        None => {
                            self.values.insert(index, e);
                            return PakTreeStatus::Ok(index);
                        },
                    },
                Ordering::Equal => {
                    entry.values.append(&mut e.values);
                    return PakTreeStatus::Ok(index);
                },
            }
        }
        match self.next {
            Some(index) => PakTreeStatus::Next(index, e),
            None => {
                self.values.push_back(e);
                return PakTreeStatus::Ok(self.values.len() - 1);
            },
        }
    }
    
    fn new_child(&mut self, index : usize, pointer : usize) {
        if index < self.values.len() {
            self.values[index].previous = Some(pointer);
        } else {
            self.next = Some(pointer);
        }
    }
}

enum PakTreeStatus {
    Ok(usize),
    Next(usize, PakTreePageEntry),
}

//==============================================================================================
//        PakTreePageEntry
//==============================================================================================

pub struct PakTreePageEntry {
    key: PakValue,
    values: Vec<PakPointer>,
    previous: Option<usize>,
}

impl Debug for PakTreePageEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.key.fmt(f)?;
        if let Some(previous) = self.previous {
            f.write_str(" -> ")?;
            write!(f, "{}", previous)?;
        }
        Ok(())
    }
}

impl PakTreePageEntry {
    pub fn new(key: PakValue, value: PakPointer) -> Self {
        PakTreePageEntry {
            key,
            values : vec![value],
            previous: None,
        }
    }
    
    pub fn compare(&self, key: &PakValue) -> Ordering {
        self.key.cmp(key)
    }
}

impl PartialEq for PakTreePageEntry {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl Eq for PakTreePageEntry {}

impl PartialOrd for PakTreePageEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl Ord for PakTreePageEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

#[cfg(test)]
mod tests {
    use crate::pak::PakPointer;

    use super::PakTree;

   
    #[test]
    fn insert_into_pak_tree() {
        let mut tree = PakTree::new(2);
        tree.access().insert(10, PakPointer::default())
            .insert(20, PakPointer::default())
            .insert(30, PakPointer::default())
            .insert(40, PakPointer::default())
        
            // should trigger a split
            .insert(25, PakPointer::default())
        
            .insert(5, PakPointer::default())
            .insert(15, PakPointer::default())
            .insert(35, PakPointer::default())
            .insert(45, PakPointer::default())
            .insert(46, PakPointer::default());
        
        println!("{:?}", tree)
        
    }
}
