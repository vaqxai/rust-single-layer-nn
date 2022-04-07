mod math;
mod perceptron;
use perceptron::Perceptron;
mod fileread;
mod lettercounter;
use lettercounter::count_letters;


fn train() {
    let mut pPoland = Perceptron::new(26, 0.1);
    let mut pEnglish = Perceptron::new(26, 0.1);
    let mut pGerman = Perceptron::new(26, 0.1);

    // TODO: Prepare the data
    // TODO: Add multiple iteration training with different data
    // TODO: Mod perceptrons to use a different classification function
    // TODO: Add a parser for user-input

    let mut input_letters = count_letters(&fileread::file_to_str("polish.txt"));
    pPoland.train(&input_letters, 1.0);
    pEnglish.train(&input_letters, 0.0);
    pGerman.train(&input_letters, 0.0);

    input_letters = count_letters(&fileread::file_to_str("english.txt"));
    pEnglish.train(&input_letters, 1.0);
    pPoland.train(&input_letters, 0.0);
    pGerman.train(&input_letters, 0.0);

    input_letters = count_letters(&fileread::file_to_str("german.txt"));
    pGerman.train(&input_letters, 1.0);
    pPoland.train(&input_letters, 0.0);
    pEnglish.train(&input_letters, 0.0);

    let test_data = count_letters(&fileread::file_to_str("test.txt"));
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
