fn replace(original: &str, regex: &str, to: &str) -> String {
    let alter = regex_analizy(regex).unwrap();
    println!("{}", alter.0.clone().into_iter().collect::<String>());
    replacement(original, alter.0, to, alter.1)
}

fn replacement(original: &str, alter: Vec<char>, to: &str, regex: Vec<Regex>) -> String {
    let mut result = original.to_string();
    let words_index = identify_word(&result, alter.clone(), &regex);
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
    replacement(&result, alter.clone(), to, regex)
}

fn identify_word(original: &str, alter: Vec<char>, regex: &Vec<Regex>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let mut count: usize = 0;
    let last_char = alter.len() - 1;
    let mut agrupar: bool = false;
    let mut fecho: bool = false;

    for t in regex {
        match t {
            Regex::Fecho => fecho = true,
            Regex::Agrupar => agrupar = true,
        }
    }
    for (pos, letter) in original.chars().enumerate() {
        if count <= alter.len() - 1 {
            if letter == alter[count] {
                result.push(pos);
                count += 1;
            }
            if count == alter.len() - 1 {
                count += 1;
            }
        } else if letter == alter[last_char] && count != 0 && fecho && !agrupar {
            result.push(pos);
        } else if count > 0 {
            break;
        }
    }

    if result.len() <= alter.len() - 2 {
        result.clear();
    }

    result
}

enum Regex {
    Fecho,
    Agrupar,
}

fn regex_analizy(regex: &str) -> Option<(Vec<char>, Vec<Regex>)> {
    let mut vec_regex: Vec<char> = regex.chars().collect();
    let mut get_symbols: Vec<Regex> = Vec::new();
    let mut indexes: Vec<usize> = Vec::new();
    for (index, symbol) in vec_regex.clone().iter().enumerate() {
        if *symbol == '*' {
            indexes.push(index);
            get_symbols.push(Regex::Fecho);
        }

        if *symbol == '(' || *symbol == ')' {
            if *symbol == '(' {
                indexes.push(index);
            } else {
                indexes.push(index);
                get_symbols.push(Regex::Agrupar)
            }
        }
    }

    if !indexes.is_empty() {
        for (ind, value) in indexes.iter().enumerate() {
            let index = value - ind;
            vec_regex.remove(index);
        }
    }
    if get_symbols.is_empty() {
        return None;
    }
    Some((vec_regex, get_symbols))
}

fn main() {
    let word = replace("fun function funnnnção furia fu f", "fun*", "teste");
    println!("Word: {}", word);
}
