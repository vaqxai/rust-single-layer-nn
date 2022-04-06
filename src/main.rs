mod math;
mod perceptron;
use perceptron::Perceptron;
mod fileread;
use fileread::read_file;
mod lettercounter;
use lettercounter::count_letters;

fn train() {
    let mut pPoland = Perceptron::new(26, 0.1);
    let mut pEnglish = Perceptron::new(26, 0.1);
    let mut pGerman = Perceptron::new(26, 0.1);
    
    let mut input = read_file("polish.txt").unwrap();
    // TODO: lines iter to vector
    let mut input_letters = count_letters(input);
    pPoland.train(input, 1.0);

    input = read_file("english.txt");
    input_letters = count_letters(input);
    pEnglish.train(input, 1.0);

    input = read_file("german.txt");
    input_letters = count_letters(input);
    pGerman.train(input, 1.0);

    let test_data = read_file("test.txt");
    let outcome_polish = pPoland.predict(&test_data);
    let outcome_english = pEnglish.predict(&test_data);
    let outcome_german = pGerman.predict(&test_data);

    println!("Perceptron thinks this text is polish: {} german {} english {} ", outcome_polish, outcome_german, outcome_english);

}

fn main() {
    println!("Hello, world!");
    let mut percPoland = Perceptron::new(26 , 0.1);
    percPoland.train(vec!(0.1, 0.2), 1.0);

}
