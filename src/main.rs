mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::time::{Duration, Instant};

use itertools::Itertools;

struct Timer {
    start: Instant,
    last: Instant,
}
impl Timer {
    fn new() -> Self {
        let instant = Instant::now();
        Self {
            last: instant,
            start: instant,
        }
    }

    fn click(&mut self) {
        static ONE_SECOND: Duration = Duration::from_secs(1);
        let elapsed = self.last.elapsed();
        if elapsed > ONE_SECOND {
            println!("{}\n", format!("*** {} s ***", elapsed.as_secs()));
        } else {
            println!("{}\n", format!("*** {} ms ***", elapsed.as_millis()));
        }

        self.last = Instant::now();
    }

    fn display_total(self) {
        println!(
            "{}",
            format!("*** TOTAL : {} s ***", self.start.elapsed().as_secs())
        );
    }
}

fn main() {
    let mut timer = Timer::new();
    println!("*** day 1 *** ");
    day1::calibrate_trebuchet();
    timer.click();

    println!("*** day 2 *** ");
    day2::play_with_cubes();
    timer.click();

    println!("*** day 3 *** ");
    day3::calibrate_engine();
    timer.click();

    println!("*** day4 ***");
    day4::play_cards();
    timer.click();

    println!("*** day5 ***");
    day5::process_seed();
    timer.click();

    println!("*** day6 ***");
    day6::race_boat();
    timer.click();

    println!("*** day7 ***");
    day7::play_camel_cards();
    timer.click();

    println!("*** day8 ***");
    day8::cross_desert();
    timer.click();

    println!("*** day9 ***");
    day9::observe_oasis();
    timer.click();

    println!("*** day10 ***");
    day10::follow_pipes();
    timer.click();

    println!("*** day11 ***");
    day11::observe_space();
    timer.click();

    println!("*** day12 ***");
    // takes > 1mn...
    day12::arrange_springs();
    timer.click();

    println!("*** day13 ***");
    day13::check_notes();
    timer.click();

    println!("*** day14 ***");
    day14::tune_parabol();
    timer.click();

    println!("*** day15 ***");
    day15::init_factory();
    timer.click();

    println!("*** day16 ***");
    day16::fix_contraption();
    timer.click();

    println!("*** day17 ***");
    day17::carry_lava();
    timer.click();

    println!("*** day18 ***");
    day18::dig_lagoon();
    timer.click();

    println!("*** day19 ***");
    day19::filter_parts();
    timer.click();

    println!("*** day20 ***");
    day20::warm_factory();
    timer.click();

    println!("*** day21 ***");
    // takes >30s...
    day21::walk_exercise();
    timer.click();

    println!("*** day22 ***");
    day22::dispatch_sand();
    timer.click();

    println!("*** day23 ***");
    day23::hike_garden();
    timer.click();

    println!("*** day24 ***");
    day24::split_snow();
    timer.click();

    println!("*** day25 ***");
    // takes >1mn...
    day25::fix_machine();
    timer.click();

    timer.display_total();
}
