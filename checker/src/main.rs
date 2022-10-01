use std::collections::HashSet;
use std::env::args;
use std::io::{Read, Write};
use std::process::{exit, Command, Stdio};

fn main() {
    let solution: [u8; 8] = [62, 123, 78, 79, 0, 1, 255, 169];
    let args: Vec<_> = args().collect();
    assert!(args.len() == 2);
    let mut program = Command::new(&args[1])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    for _ in 1..10000000 {
        let program_out = program.stdout.as_mut().unwrap();
        let mut guess: [u8; 8] = Default::default();
        program_out.read_exact(&mut guess).unwrap();
        let mut correct_correct: u8 = 0;
        let (guess, mut remaining): (Vec<_>, HashSet<_>) = guess
            .into_iter()
            .zip(solution)
            .filter(|(peg, sol)| {
                if *peg == *sol {
                    correct_correct += 1;
                    false
                } else {
                    true
                }
            })
            .unzip();
        let mut correct_incorrect: u8 = 0;
        for guess_peg in guess {
            if remaining.remove(&guess_peg) {
                correct_incorrect += 1;
            }
        }
        let program_in = program.stdin.as_mut().unwrap();
        program_in
            .write_all(&[correct_correct, correct_incorrect])
            .unwrap();
        program_in.flush().unwrap();
        if correct_correct == 8 {
            eprintln!("Winner, Winner");
            exit(0);
        }
    }
    eprintln!("Ran out of guesses!");
    program.kill().unwrap();
    exit(1);
}
