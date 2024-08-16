static WORD_LIST: &str = include_str!("words.txt");

/// Returns a list of words from the words.txt file
pub fn get_word_list() -> Vec<&'static str> {
    let list: Vec<&str> = WORD_LIST.split("\n").collect();
    list
}
