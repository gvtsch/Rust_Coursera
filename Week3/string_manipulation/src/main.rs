fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog";
    /* Use slicing to get the first three characters of the sentence */
    println!("{}", &sentence[0..4]); /* [0..=4] means inclusive */
    println!("{}", &sentence[0..=4]);

    /* Concatenate using format! */
    let description  = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    let mut vowel_counts = [0; 5]; // Array to store vowel counts

    /* iterate over the characters in the sentence */
    for c in sentence.chars() {
        match c {
            'a' => vowel_counts[0] += 1,
            'e' => vowel_counts[1] += 1,
            'i' => vowel_counts[2] += 1,
            'o' => vowel_counts[3] += 1,
            'u' => vowel_counts[4] += 1,
            _ => continue,
        }
    }

    /* Print the count for each vowel */
    println!("Count of 'a': {}", vowel_counts[0]);
    println!("Count of 'e': {}", vowel_counts[1]);
    println!("Count of 'i': {}", vowel_counts[2]);
    println!("Count of 'o': {}", vowel_counts[3]);
    println!("Count of 'u': {}", vowel_counts[4]);

    /* Split and collect into a vector */
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);

    /* Loop over every word in the sentence */
    let mut longest_word = "";
    for word in words {
        if word.len() > longest_word.len() {
            longest_word = word;
        }
    }
    println!("The longest word is: {}", longest_word)
}
