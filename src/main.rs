use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let mut language = String::new();

    println!("Select your language:");
    println!("1 - Português");
    println!("2 - English");

    io::stdin()
        .read_line(&mut language)
        .expect("Failed to read line");

    if language.trim() == "1" {
        portuguese(); 
    } else if language.trim() == "2" {
        english();
    } else {
        println!("Invalid input. Try again!");
        main();
    }

}

fn english() {
    println!("╔════════════════════════════════════════╗");
    println!("║    WELCOME TO THE NUMBER GUESSING GAME!║");
    println!("╚════════════════════════════════════════╝");
    println!("Try to guess the number between 1 and 100");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let max_attempts = 7;
    let mut guesses = 0;

    let mut max_number = 100;
    let mut min_number = 0;

    while guesses < max_attempts {
        guesses += 1;

        println!("\nGuess the number: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Try again!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                if guesses == max_attempts - 1 {
                    println!("╔═════════════════════════════════╗");
                    println!("║           Last attempt!         ║");
                    println!("╚═════════════════════════════════╝");
                } else {
                    println!("Attempt - {} of 7", guesses);
                };
                if guess > min_number {
                    min_number = guess;
                }
                println!("Between {} and {}", min_number, max_number);
            },
            Ordering::Greater => {
                if guesses == max_attempts - 1 {
                    println!("╔═════════════════════════════════╗");
                    println!("║           Last attempt!         ║");
                    println!("╚═════════════════════════════════╝");
                } else {
                    println!("Attempt - {} of 7", guesses);
                };
                if guess < max_number { 
                    max_number = guess;
                }
                println!("Between {} and {}", min_number, max_number);
            },
            Ordering::Equal => {
                println!("╔══════════════════════════════════════╗");
                println!("║      Congratulations, you won!       ║");
                println!("╚══════════════════════════════════════╝");
                break;
            }
        }
        if guesses == max_attempts {
            println!("The number was: {}", secret_number);
        }
    }

    println!("Do you want to play again? (s/n)");
    let mut play_again = String::new();
    io::stdin()
        .read_line(&mut play_again)
        .expect("Failed to read line");

    if play_again.trim().to_lowercase() == "s" {
        main();
    } else {
        println!("Thank you for playing!");
    }
}


fn portuguese() {

    println!("╔════════════════════════════════════════╗");
    println!("║ BEM VINDO, ao jogo de adivinhar o número!║");
    println!("╚════════════════════════════════════════╝");
    println!("Tenta adivinhar o número de 1 a 100");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let max_attempts = 7;
    let mut guesses = 0;

    let mut max_number = 100;
    let mut min_number = 0;

    while guesses < max_attempts {
        guesses += 1;

        println!("\nAdivinha o número: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Tente novamente!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                if guesses == max_attempts - 1 {
                    println!("╔═════════════════════════════════╗");
                    println!("║       Última tentativa!         ║");
                    println!("╚═════════════════════════════════╝");
                } else {
                    println!("Tentativa - {} de 7", guesses);
                };
                if guess > min_number {
                    min_number = guess;
                }
                println!("Entre {} e {}", min_number, max_number);
            },
            Ordering::Greater => {
                if guesses == max_attempts - 1 {
                    println!("╔═════════════════════════════════╗");
                    println!("║       Última tentativa!         ║");
                    println!("╚═════════════════════════════════╝");
                } else {
                    println!("Tentativa - {} de 7", guesses);
                };
                if guess < max_number { 
                    max_number = guess;
                }
                println!("Entre {} e {}", min_number, max_number);
            },
            Ordering::Equal => {
                println!("╔══════════════════════════════════════╗");
                println!("║         Parabéns, você venceu!       ║");
                println!("╚══════════════════════════════════════╝");
                break;
            }
        }
        if guesses == max_attempts {
            println!("O número era: {}", secret_number);
        }
    }

    println!("Deseja jogar novamente? (s/n)");
    let mut play_again = String::new();
    io::stdin()
        .read_line(&mut play_again)
        .expect("Falha ao ler a linha");

    if play_again.trim().to_lowercase() == "s" {
        main();
    } else {
        println!("Obrigado por jogar!");
    }
}