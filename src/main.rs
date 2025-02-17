use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref SPECIAL_NUMS: Mutex<Vec<i32>> = Mutex::new(vec![0]);
    static ref ALPHA: Mutex<Vec<char>> = Mutex::new(vec!['a']);
}
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
    let mut set: bool = false;

    for t in regex {
        match t {
            Regex::Fecho => fecho = true,
            Regex::Agrupar => agrupar = true,
            Regex::Set => set = true,
        }
    }
    for (pos, letter) in original.chars().enumerate() {
        if count <= alter.len() - 1 {
            if set && letter == '#' {}
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
    Set,
}
fn history_values(one: char, two: char) {
    if one == '\\' && two == 'd' {
        let mut special = SPECIAL_NUMS.lock().unwrap();
        special.extend(0..9);
    } else {
        let mut alp = ALPHA.lock().unwrap();
        alp.extend(one..two);
    }
}

fn regex_analizy(regex: &str) -> Option<(Vec<char>, Vec<Regex>)> {
    let mut vec_regex: Vec<char> = regex.chars().collect();
    let mut get_symbols: Vec<Regex> = Vec::new();
    let mut indexes_set: Vec<char> = Vec::new();
    let mut agrupar = false;
    let mut fecho = false;
    let mut set = false;
    let mut loop_count = 0;
    loop {
        for (index, symbol) in vec_regex.clone().iter().enumerate() {
            if *symbol == '*' {
                if loop_count == 2 {
                    vec_regex.remove(index);
                }
                fecho = true;
            }

            if *symbol == '(' || *symbol == ')' {
                if loop_count == 1 {
                    vec_regex.remove(index);
                }
                agrupar = true;
            }
            if *symbol == '[' || *symbol == ']' {
                if loop_count == 0 {
                    if *symbol == '[' {
                        indexes_set.push(vec_regex[index + 1]);
                        if vec_regex[index + 1] == '\\' {
                            vec_regex.insert(index - 1, '%');
                        } else {
                            vec_regex.insert(index - 1, '#')
                        }
                    } else {
                        indexes_set.push(vec_regex[index - 1]);
                        if vec_regex[index - 3] == '[' {
                            vec_regex.drain((index - 3)..index);
                        } else {
                            vec_regex.drain((index - 2)..index);
                        }
                    }
                }
                set = true;
            }
        }
        loop_count += 1;
        if loop_count > 2 {
            break;
        }
    }

    if !indexes_set.is_empty() {
        history_values(indexes_set[0], indexes_set[1]);
    }

    if fecho {
        get_symbols.push(Regex::Fecho);
    }
    if agrupar {
        get_symbols.push(Regex::Agrupar);
    }
    if set {
        get_symbols.push(Regex::Set);
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
