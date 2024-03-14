use colored::*;
use rand::Rng;
use std::io;

fn main() {

    // Insertion, deletion, and modification probabilities
    let ins_prob = 0.2;
    let del_prob = 0.2;
    let mod_prob = 0.3;

    let mut rng = rand::thread_rng();
    let nucleotides = ['A', 'C', 'G', 'T'];

    // Takes input through terminal and stores to 'sequence'
    println!("Enter a string of nucleotide bases:");
    let mut sequence = String::new();
    io::stdin().read_line(&mut sequence).unwrap();
    sequence = sequence.trim().to_string();

    // Creates a string for each of the three outputs.
    let mut original_sequence = sequence.clone();
    let mut output = Vec::new();
    let mut changelog = Vec::new();

    // Insertion before the sequence
    let mut ins_count = 0;
    while rng.gen::<f64>() < ins_prob {
        let new_base = nucleotides[rng.gen_range(0..4)];
        sequence.insert(ins_count, new_base);
        output.push(new_base.to_string());
        changelog.push(format!("{}", new_base.to_string().truecolor(255,231,76)));
        original_sequence.insert(ins_count, ' ');
        ins_count += 1;
    }

    // In-sequence changes
    let mut i = ins_count;
    while i < sequence.len() {
        let current_base = sequence.chars().nth(i).unwrap();

        // Deletion
        if rng.gen::<f64>() < del_prob {
            changelog.push("-".truecolor(255,89,100).to_string());
        }

        // Modification
        else{ 
            if rng.gen::<f64>() < mod_prob {
                let mut new_base = nucleotides[rng.gen_range(0..4)];
                while new_base == current_base {
                    new_base = nucleotides[rng.gen_range(0..4)];
                }
                sequence.remove(i);
                sequence.insert(i, new_base);
                output.push(new_base.to_string());
                changelog.push(format!("{}", new_base.to_string().truecolor(56,97,140)));
                } 
            else {
                output.push(current_base.to_string());
                changelog.push(current_base.to_string());
                }
        }

        // Insertion after the current base (can insert multiple)
        while rng.gen::<f64>() < ins_prob {
            let new_base = nucleotides[rng.gen_range(0..4)];
            sequence.insert(i + 1, new_base);
            output.push(new_base.to_string());
            changelog.push(format!("{}", new_base.to_string().truecolor(255,231,76)));
            original_sequence.insert(i + 1, ' ');
            i += 1;
        }

        i += 1;

    }


    // Terminal output
    println!("--- Modified sequence ---");
    println!("{}", output.join(""));
    println!("--- Changelog ---");
    println!("{}", original_sequence);
    println!("{}", changelog.join(""));
}