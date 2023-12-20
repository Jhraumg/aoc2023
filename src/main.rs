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
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    println!("*** day 1 *** ");
    day1::calibrate_trebuchet();
    println!("*** day 2 *** ");
    day2::play_with_cubes();
    println!("*** day 3 *** ");
    day3::calibrate_engine();

    println!("*** day4 ***");
    day4::play_cards();

    println!("*** day5 ***");
    day5::process_seed();

    println!("*** day6 ***");
    day6::race_boat();

    println!("*** day7 ***");
    day7::play_camel_cards();

    println!("*** day8 ***");
    day8::cross_desert();

    println!("*** day9 ***");
    day9::observe_oasis();

    println!("*** day10 ***");
    day10::follow_pipes();

    println!("*** day11 ***");
    day11::observe_space();

    println!("*** day12 ***");
    // takes > 1mn...
    // day12::arrange_springs();

    println!("*** day13 ***");
    day13::check_notes();

    println!("*** day14 ***");
    day14::tune_parabol();

    println!("*** day15 ***");
    day15::init_factory();

    println!("*** day16 ***");
    day16::fix_contraption();

    println!("*** day17 ***");
    day17::carry_lava();

    println!("*** day18 ***");
    day18::dig_lagoon();

    println!("*** day19 ***");
    day19::filter_parts();

    println!("*** day20 ***");
    day20::warm_factory();
}
