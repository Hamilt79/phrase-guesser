use std::sync::OnceLock;

static WORD_LIST: &str = include_str!("words.txt");
static WORDS: OnceLock<Vec<&str>> = OnceLock::new();

/// Returns a list of words from the words.txt file
pub fn get_word_list() -> &'static Vec<&'static str> {
    WORDS.get_or_init(|| {
        let list: Vec<&str> = WORD_LIST.split("\n").collect();
        list
    })
}
