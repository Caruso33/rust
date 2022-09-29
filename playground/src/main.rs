fn main() {}

fn reverse_words(words: &str) -> String {
    let mut single_words: Vec<&str> = words.split(' ').collect();

    single_words.reverse();

    single_words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::reverse_words;
    #[test]
    fn returns_expected() {
        assert_eq!(reverse_words("hello world!"), "world! hello");
    }
}
