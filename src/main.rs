use colored::*;
use std::{io, thread, time::Duration, time::Instant};
mod fibonaci;
use fibonaci::calculation::fibonacci;

fn main() {
    #[cfg(debug_assertions)]
    println!("DEBUG build");
    #[cfg(not(debug_assertions))]
    println!("RELEASE build");

    #[cfg(windows)]
    enable_ansi_colors();
    let mut continue_program = true;
    println!(
        "MADE IN RUST\n!! The size of a number is not limited. Be careful with your computing resources !!"
    );
    while continue_program {
        looping();
        end_of_program_dialog(&mut continue_program);
    }
    println!("{}", "Thanks for using this app! :)".blue().bold());
    thread::sleep(Duration::from_secs(3)); //delay 3 sekundy po ukonceni aplikace
}

fn end_of_program_dialog(continue_program: &mut bool) {
    let mut users_answer = String::new();
    loop {
        println!("{}", "\nDo you wish to exit? Y/n: ".magenta());
        io::stdin()
            .read_line(&mut users_answer)
            .expect("Chyba vstupniho zarizeni");
        let user_answ_to_compare = users_answer.trim();
        if user_answ_to_compare == "Y" || user_answ_to_compare == "y" {
            *continue_program = false;
            return;
        } else if user_answ_to_compare == "N" || user_answ_to_compare == "n" {
            return;
        } else {
            println!("\nZadali jste neexistujici moznost {user_answ_to_compare}");
        }
        users_answer.clear();
    }
}



fn looping() {
    let n: u128 = loop {
        let mut retezec = String::new();
        println!("Kolikate cislo z fibonacciho posloupnosti chces?: ");
        io::stdin().read_line(&mut retezec).expect("chyba vstupu");
        //let retezec2: u128 = retezec.trim();
        match retezec.trim().parse() {
            Ok(n) => break n,
            Err(e) => {
                println!("{} {}", "Chyba u tveho cisla. Chyba typu:".red().bold(), e);
            }
        };
    };
    let start = Instant::now();
    let result = fibonacci(n);
    let end = start.elapsed();
    println!(
        "{}. cislo fibonnaciho posloupnosi je: {}",
        n.clone(),
        result
    );
    println!(
        "Cas behu vypoctu byl: {} s / {} ms / {} µs / {} ns",
        end.as_secs(),
        end.as_millis(),
        end.as_micros(),
        end.as_nanos()
    );
}

#[cfg(windows)]
fn enable_ansi_colors() {
    use winapi::um::consoleapi::GetConsoleMode;
    use winapi::um::consoleapi::SetConsoleMode;
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle != INVALID_HANDLE_VALUE {
            let mut mode = 0;
            if GetConsoleMode(handle, &mut mode) != 0 {
                SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
            }
        }
    }
}
