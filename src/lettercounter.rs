pub fn count_letters(input: &str) -> Vec<usize> {
    let mut counts = vec![0; 26];
    for c in input.chars() {
        let index = c as usize - 'a' as usize;
        counts[index] += 1;
    }
    counts
}