use std::{fs::File, io::BufReader};

use rust_challenges::other::space_missions::longest_mars_mission;

fn main() {
    let f = File::open("data/space_missions.log").expect("Could not open log file");
    let reader = BufReader::new(f);
    let (duration, code) = longest_mars_mission(reader);
    println!("Mission Duration (days): {duration}");
    println!("Security Code: {code}");
}
