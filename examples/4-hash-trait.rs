const TABLE_SIZE: usize = 10;
use std::hash::Hasher;
use std::{collections::hash_map::DefaultHasher, hash::Hash};

struct HashTable<T>
where
    T: Sized,
{
    table: [Vec<T>; TABLE_SIZE],
}

fn calculate_hash<H: Hash>(t: &H) -> usize {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    (s.finish() % TABLE_SIZE as u64) as usize
}

impl<T> HashTable<T>
where
    T: Hash + PartialEq,
{
    fn new() -> Self {
        HashTable {
            table: [(); TABLE_SIZE].map(|_| Vec::new()),
        }
    }

    fn insert(&mut self, word: T) {
        let index = calculate_hash(&word);
        self.table[index].push(word);
    }

    fn search(&self, word: &T) -> bool {
        let index = calculate_hash(word);

        self.table[index].contains(word)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut table = HashTable::new();
        table.insert("hello".to_string());
        table.insert("olleh".to_string());

        assert!(table.search(&"hello".to_string()));
        assert!(table.search(&"olleh".to_string()));

        assert!(!table.search(&"helol".to_string()));
    }
}
