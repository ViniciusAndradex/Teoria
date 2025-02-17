fn replace(original: &str, regex: &str, to: &str) -> String {
    let alter = regex_analizy(regex).unwrap().to_vec();
    replacement(original, alter, to)
}

fn replacement(original: &str, alter: Vec<char>, to: &str) -> String {
    let mut result = original.to_string();
    let words_index = identify_word(&result, alter.clone());
    let letter_to: Vec<char> = to.chars().collect();
    let mut origin_vec: Vec<char> = result.chars().collect();
    if words_index.is_empty() {
        return result;
    }
    let mut count = 0;
    let mut index = words_index[words_index.len() - 1] + 1;

    for inner_index in words_index.clone() {
        if count == letter_to.len() {
            break;
        }
        origin_vec[inner_index] = letter_to[count];
        count += 1;
    }
    if words_index.len() < letter_to.len() {
        loop {
            if count == letter_to.len() {
                break;
            }
            origin_vec.insert(index, letter_to[count]);
            count += 1;
            index += 1;
        }
    } else {
        loop {
            if count == words_index.len() {
                break;
            }
            if count == letter_to.len() {
                origin_vec.remove(index);
                index += 1;
                count += 1;
                continue;
            }
            origin_vec.insert(index, letter_to[count]);
            count += 1;
            index += 1;
        }
    }
    result = origin_vec.iter().collect();
    replacement(&result, alter.clone(), to)
}

fn identify_word(original: &str, alter: Vec<char>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let mut count: usize = 0;
    let last_char = alter.len() - 2;
    for (pos, letter) in original.chars().enumerate() {
        if count < alter.len() - 1 {
            if letter == alter[count] {
                result.push(pos);
                count += 1;
            }
        } else if letter == alter[last_char] && count != 0 {
            result.push(pos);
            count += 1;
        } else if count > 0 {
            break;
        }
    }

    result
}

fn regex_analizy(regex: &str) -> Option<Vec<char>> {
    let vec_regex: Vec<char> = regex.chars().collect();

    for symbol in vec_regex.clone() {
        if symbol == '*' {
            return Some(vec_regex);
        }
    }
    return None;
}

fn main() {
    let word = replace("fun function funnnnção", "por*", "teste");
    println!("Word: {}", word);
}
