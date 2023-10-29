const TABLE_SIZE: usize = 10;

fn hash(key: &str) -> usize {
    let mut s = 0;
    for c in key.chars() {
        s += c as usize;
    }
    s % TABLE_SIZE
}

fn insert(table: &mut [Vec<String>], word: String) {
    let index = hash(&word);
    table[index].push(word);
}

fn search(table: &[Vec<String>], word: &str) -> bool {
    let index = hash(word);

    table[index].contains(&word.to_string())
}

fn main() {
    let mut table = [(); 100].map(|_| Vec::new());

    insert(&mut table, "hello".to_string());

    println!("{:#?}", table);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut table = [(); 100].map(|_| Vec::new());

        insert(&mut table, "hello".to_string());
        insert(&mut table, "olleh".to_string());

        assert_eq!(search(&table, "helol"), false);
        assert_eq!(search(&table, "hello"), true);
        assert_eq!(search(&table, "olleh"), true);
    }
}
