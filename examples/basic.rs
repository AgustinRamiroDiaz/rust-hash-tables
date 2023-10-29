const TABLE_SIZE: usize = 10;

fn hash(key: &str) -> usize {
    let mut s = 0;
    for c in key.chars() {
        s += c as usize;
    }
    s % TABLE_SIZE
}

fn insert(table: &mut [String], word: String) {
    let index = hash(&word);
    table[index] = word;
}

fn search(table: &[String], word: &str) -> bool {
    let index = hash(word);

    table[index] == word
}

fn main() {
    let mut table = [(); 100].map(|_| String::new());

    insert(&mut table, "hello".to_string());

    println!("{:#?}", table);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut table = [(); 100].map(|_| String::new());

        insert(&mut table, "hello".to_string());
        assert!(search(&table, "hello"));

        // This will override it
        insert(&mut table, "olleh".to_string());
        assert!(search(&table, "olleh"));
        assert!(!search(&table, "hello"));
    }
}
