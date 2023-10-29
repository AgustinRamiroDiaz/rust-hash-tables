use std::hash::Hasher;
use std::{collections::hash_map::DefaultHasher, hash::Hash};

struct HashTable<T>
where
    T: Sized,
{
    table: Vec<Vec<T>>,
}

impl<T> HashTable<T>
where
    T: Hash + PartialEq,
{
    fn new(table_size: usize) -> Self {
        let table = (0..table_size).map(|_| Vec::new()).collect();

        HashTable { table }
    }
    fn calculate_hash<H: Hash>(&self, t: &H) -> usize {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        let len = self.table.len();
        (s.finish() % len as u64) as usize
    }

    fn insert(&mut self, word: T) {
        let index = self.calculate_hash(&word);
        self.table[index].push(word);
    }

    fn search(&self, word: T) -> bool {
        let index = self.calculate_hash(&word);

        self.table[index].contains(&word)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut table = HashTable::new(10);
        table.insert("hello".to_string());
        table.insert("olleh".to_string());

        assert!(table.search("hello".to_string()));
        assert!(table.search("olleh".to_string()));

        assert!(!table.search("helol".to_string()));
    }
}
