fn replace(original: &str, alter: &str, to: &str) -> String{
    let mut original_vec: Vec<char> = original.chars().collect();
    let altered: Vec<char> = to.chars().collect();
    let words_index = identify_word(original, alter);

    for vec in words_index {
        for i in 0..vec.len() {
            if i <= alter.len() {
                original_vec[vec[i]] = altered[i];
            } else {
                original_vec.insert(i, altered[i]);
            }
        }
    }

    original_vec.iter().collect::<String>()
}

fn identify_word(original: &str, alter: &str) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();
    let altered: Vec<char> = alter.chars().collect();
    let mut count: usize = 0;
    let mut word: usize = 0;

    for letter in original.chars() {
        if result.len() <= word {
            result.resize_with(word + 1, Vec::new);
        }

        if letter == altered[count] {
            result[word].push(count);
            count += 1;

            if count == altered.len() {
                count = 0;
                word += 1;
            }
        } else if result.len() != 0 {
            if result[word].is_empty() {
                count = 0;
                continue;
            }
            result[word].clear();
        }
    }

    result
}

fn main() {

    let word = replace("Verificando a função replace", "função", "foda-se");
    println!("Word: {}", word);
}
