fn replace(original: &str, alter: &str, to: &str) -> String {
    let mut original_vec: Vec<char> = original.chars().collect();
    let altered: Vec<char> = to.chars().collect();
    let words_index = identify_word(original, alter);

    // Ajuste necessário para evitar modificações enquanto iteramos
    let mut offset = 0;

    for vec in words_index {
        let start_index = vec[0] + offset;
        let end_index = vec.last().unwrap() + offset;

        // Remover a palavra original
        original_vec.drain(start_index..=end_index);

        // Inserir a palavra alterada
        for (i, &ch) in altered.iter().enumerate() {
            original_vec.insert(start_index + i, ch);
        }

        // Atualizar o deslocamento causado pela diferença de tamanho
        offset += altered.len() - vec.len();
    }

    original_vec.iter().collect()
}

fn identify_word(original: &str, alter: &str) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();
    let altered: Vec<char> = alter.chars().collect();
    let mut count: usize = 0;
    let mut word: Vec<usize> = Vec::new();

    for (index, letter) in original.chars().enumerate() {
        if letter == altered[count] {
            word.push(index);
            count += 1;

            if count == altered.len() {
                result.push(word.clone());
                word.clear();
                count = 0;
            }
        } else {
            word.clear();
            count = 0;
        }
    }

    result
}

fn main() {
    let word = replace("Verificando a função replace função", "função", "foda-se");
    println!("Word: {}", word);
}
