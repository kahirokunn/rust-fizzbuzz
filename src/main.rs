fn fizz_buzz(number: u32) -> String {
    match (number % 3 == 0, number % 5 == 0) {
        (true, true) => String::from("Fizz Buzz"),
        (true, false) => String::from("Fizz"),
        (false, true) => String::from("Buzz"),
        (false, false) => String::from(number.to_string())
    }
}

fn start(last_step: u32) -> Vec<String> {
    play_game(1, last_step, vec![])
}

fn play_game(step: u32, last_step: u32, mut log: Vec<String>) -> Vec<String> {
    match last_step - step {
        0 => log,
        _ => {
            log.push(fizz_buzz(step));
            play_game(step + 1, last_step, log)
        },
    }
}

fn main() {
    for item in &start(30) {
        println!("{}", item);
    }
}
