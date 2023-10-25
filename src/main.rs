use std::io;
use rand::Rng;

#[derive(Debug, Clone)]
struct WorldLine {
    divergence: f64,
    events: Vec<String>,
}

impl WorldLine {
    fn new(divergence: f64) -> Self {
        WorldLine {
            divergence,
            events: Vec::new(),
        }
    }

    fn add_event(&mut self, event: &str) {
        self.events.push(event.to_string());
    }

    fn apply_butterfly_effect(&mut self, event: &str, new_divergence: f64) {
        self.divergence = new_divergence;
        self.add_event(event);
    }
}

struct DivergenceMeter {
    world_line: WorldLine,
    time_leap_count: usize,
}

impl DivergenceMeter {
    fn new(world_line: WorldLine) -> Self {
        DivergenceMeter {
            world_line,
            time_leap_count: 0,
        }
    }

    fn display(&self) {
        println!("\nDivergence: {:.6}", self.world_line.divergence);
        println!("Time Leaps: {}", self.time_leap_count);
        println!();
    }

    fn send_dmail(&mut self, event: &str) {
        let new_divergence = rand::thread_rng().gen_range(0.0..0.999999);
        println!("\nSending D-Mail...");
        self.world_line.apply_butterfly_effect(event, new_divergence);
        self.display();
    }

    fn time_leap(&mut self, event: &str) {
        self.time_leap_count += 1;
        println!("\nExecuting Time Leap...");

        if rand::thread_rng().gen_bool(0.01) {
            let new_divergence = 1.0;
            self.world_line.apply_butterfly_effect(event, new_divergence);
        }

        self.display();
    }

    fn user_choice(&mut self) {
        loop {
            println!("Choose an action:");
            println!("1: Send D-Mail");
            println!("2: Execute Time Leap");
            println!("3: Display current state");
            println!("4: Quit");
            print!("> ");

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read input");

            match choice.trim() {
                "1" => {
                    let mut event = String::new();
                    println!("\nDescribe the event:");
                    io::stdin().read_line(&mut event).expect("Failed to read input");
                    self.send_dmail(&event);
                }
                "2" => {
                    let mut event = String::new();
                    println!("\nDescribe the event:");
                    io::stdin().read_line(&mut event).expect("Failed to read input");
                    self.time_leap(&event);
                }
                "3" => {
                    self.display();
                }
                "4" => {
                    println!("\nTutturu～");
                    break;
                }
                _ => {
                    println!("\nInvalid choice. Please try again.\n");
                }
            }
        }
    }
}

fn main() {
    let initial_world_line = WorldLine::new(1.0);
    let mut meter = DivergenceMeter::new(initial_world_line);
    meter.user_choice();
}

