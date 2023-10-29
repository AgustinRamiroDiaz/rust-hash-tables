const TABLE_SIZE: usize = 10;

fn hash(key: &str) -> usize {
    let mut s = 0;
    for c in key.chars() {
        s += c as usize;
    }
    s % TABLE_SIZE
}

struct HashTable {
    table: [Vec<String>; TABLE_SIZE],
    hash: fn(&str) -> usize,
}

impl HashTable {
    fn new() -> Self {
        HashTable {
            table: [(); TABLE_SIZE].map(|_| Vec::new()),
            hash,
        }
    }

    fn insert(&mut self, word: String) {
        let index = (self.hash)(&word);
        self.table[index].push(word);
    }

    fn search(&self, word: &str) -> bool {
        let index = (self.hash)(word);

        self.table[index].contains(&word.to_string())
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

        assert!(table.search("hello"));
        assert!(table.search("olleh"));

        assert!(!table.search("helol"));
    }
}
