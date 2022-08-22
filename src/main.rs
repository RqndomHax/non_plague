mod entities;
mod traits;

use entities::human::*;

fn main() {
    let mut humans: Vec<Human> = Vec::new();

    Human::spawn(Human::Female, &mut humans);
    println!("humans size = {}", humans.len());
}
