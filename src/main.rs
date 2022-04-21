mod math;
mod perceptron;
use once_cell::sync::OnceCell;
use perceptron::Perceptron;
mod fileread;
mod lettercounter;
use lettercounter::count_letters;

use clap::Parser;

fn train(p_polish: &mut Perceptron, p_english: &mut Perceptron, p_german: &mut Perceptron) {

    // TODO: Mod perceptrons to use a different classification function

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
    let input_str = &fileread::file_to_str_ascii_only(filepath);
    let test_data = count_letters(input_str);
    let outcome_polish = p_polish.predict_raw(&test_data);
    let outcome_english = p_english.predict_raw(&test_data);
    let outcome_german = p_german.predict_raw(&test_data);

    interpret_results(outcome_polish, outcome_english, outcome_german, input_str);
}

fn interpret_results(outcome_polish: f64, outcome_english: f64, outcome_german: f64, text: &String) {

    println!("Perceptrons' reactions: PL: {}, EN: {}, GE {}", outcome_polish, outcome_english, outcome_german);

    // choose the strongest reaction

    let mut strongest_reaction = "";
    let mut strongest_reaction_value = 0.0;

    if outcome_polish > strongest_reaction_value {
        strongest_reaction = "polish";
        strongest_reaction_value = outcome_polish;
    }

    if outcome_english > strongest_reaction_value {
        strongest_reaction = "english";
        strongest_reaction_value = outcome_english;
    }

    if outcome_german > strongest_reaction_value {
        strongest_reaction = "german";
    }

    // only show first 64 chars of input

    let input_truncated = match text.len() {
        64.. => {
            let out = &text[0..64];
            out.to_owned().push('…');
            out
        },
        _   => text,
    };

    match strongest_reaction {
        "polish" => println!("Network thinks this text is polish: {}...", input_truncated),
        "english" => println!("Network thinks this text is english: {}...", input_truncated),
        "german" => println!("Network thinks this text is german: {}...", input_truncated),
 _ => println!("Network could not classify this text: {}", input_truncated),
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(short, long, help = "A string to classify with the net", conflicts_with = "testfile")]
    text: Option::<String>,

    #[clap(short, long, default_value_t = 0.1)]
    learning_rate: f64,

    #[clap(short = 'f', long, default_value = "test.txt", help = "Path to a file containing text to be classified")]
    testfile: String,

    #[clap(short, long, default_value_t = 1, help = "The number of times to train the network")]
    iterations: u64,

    #[clap(short, long, help = "Print additional info")]
    verbose: bool,
}

static ARGS: OnceCell::<Args> = OnceCell::new();

fn vprint(message: String) {
    if ARGS.get().unwrap_or(&Args::parse()).verbose {
        println!("{}", message);
    }
}

fn main() {

    match ARGS.set(Args::parse()) {
        Ok(_) => {},
        Err(_) => {
            println!("Could not set args global variable, because it was already set");
            return;
        },
    };

    let args = match ARGS.get() {
        Some(args) => args,
        None => {
            println!("Could not get args global variable.");
            return;
        },
    };


    let mut p_polish: Perceptron = Perceptron::new(26, args.learning_rate);
    let mut p_english: Perceptron = Perceptron::new(26, args.learning_rate);
    let mut p_german: Perceptron = Perceptron::new(26, args.learning_rate);

    for i in 0..args.iterations {
        train(&mut p_polish, &mut p_english, &mut p_german);
        if (i+1) % 5 == 0 || i+1 == args.iterations {
            println!("Perceptrons trained {} times", i+1);
        }
        vprint(format!("Polish perceptron: {}\nEnglish perceptron: {}\nGerman perceptron: {}", p_polish, p_english, p_german));
    }

    println!("Training complete!");

    if let Some(text) = &args.text {
        let input_letters = count_letters(&text.to_ascii_lowercase());
        let outcome_polish = p_polish.predict(&input_letters);
        let outcome_english = p_english.predict(&input_letters);
        let outcome_german = p_german.predict(&input_letters);

        interpret_results(outcome_polish, outcome_english, outcome_german, text)

    } else {
        test(&mut p_polish, &mut p_english, &mut p_german, &args.testfile);
    }
}
