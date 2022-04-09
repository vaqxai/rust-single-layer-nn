use crate::vprint;

pub fn count_letters(input: &str) -> Vec<f64> {
    let mut counts = vec![0.0; 26];
    for c in input.chars() {
        if c < 'a' || c > 'z' {
            continue;
        }
        let index = c as usize - 'a' as usize;
        counts[index] += 1.0;
    }
    vprint(format!("counts: {:?}", counts));
    counts
}