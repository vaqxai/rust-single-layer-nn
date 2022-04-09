mod math;
mod perceptron;
use perceptron::Perceptron;
mod fileread;
mod lettercounter;
use lettercounter::count_letters;

use clap::Parser;


fn train(p_polish: &mut Perceptron, p_english: &mut Perceptron, p_german: &mut Perceptron) {

    // TODO: Prepare the data
    // TODO: Add multiple iteration training with different data
    // TODO: Mod perceptrons to use a different classification function
    // TODO: Add a parser for user-input

    let mut input_letters = count_letters(&fileread::file_to_str_ascii_only("train/polish.txt"));
    p_polish.train(&input_letters, 1.0);
    p_english.train(&input_letters, 0.0);
    p_german.train(&input_letters, 0.0);

    input_letters = count_letters(&fileread::file_to_str_ascii_only("train/english.txt"));
    p_english.train(&input_letters, 1.0);
    p_polish.train(&input_letters, 0.0);
    p_german.train(&input_letters, 0.0);

    input_letters = count_letters(&fileread::file_to_str_ascii_only("train/german.txt"));
    p_german.train(&input_letters, 1.0);
    p_polish.train(&input_letters, 0.0);
    p_english.train(&input_letters, 0.0);

}

fn test(p_polish: &mut Perceptron, p_english: &mut Perceptron, p_german: &mut Perceptron, filepath: &str){
    let test_data = count_letters(&fileread::file_to_str_ascii_only(filepath));
    let outcome_polish = p_polish.predict(&test_data);
    let outcome_english = p_english.predict(&test_data);
    let outcome_german = p_german.predict(&test_data);

    println!("Perceptron thinks this text is polish: {} german {} english {} ", outcome_polish, outcome_german, outcome_english);
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, help = "A string to classify with the net", conflicts_with = "testfile")]
    text: Option::<String>,

    #[clap(short, long, default_value_t = 0.1)]
    learning_rate: f64,

    #[clap(short = 'f', long, default_value = "test.txt", help = "Path to a file containing text to be classified")]
    testfile: String,

    #[clap(short, long, default_value_t = 1, help = "The number of times to train the network")]
    iterations: u8,
}

fn main() {

    let args = Args::parse();

    let mut p_polish: Perceptron = Perceptron::new(26, args.learning_rate);
    let mut p_english: Perceptron = Perceptron::new(26, args.learning_rate);
    let mut p_german: Perceptron = Perceptron::new(26, args.learning_rate);

    for i in 0..args.iterations {
        train(&mut p_polish, &mut p_english, &mut p_german);
        println!("Perceptrons trained {} times", i+1);
        println!("Polish perceptron: {}\nEnglish perceptron: {}\nGerman perceptron: {}", p_polish, p_english, p_german);
    }

    if let Some(text) = args.text {
        let input_letters = count_letters(&text);
        let outcome_polish = p_polish.predict(&input_letters);
        let outcome_english = p_english.predict(&input_letters);
        let outcome_german = p_german.predict(&input_letters);

        println!("Perceptron thinks this text is polish: {} german {} english {} ", outcome_polish, outcome_german, outcome_english);
    } else {
        test(&mut p_polish, &mut p_english, &mut p_german, &args.testfile);
    }
}
