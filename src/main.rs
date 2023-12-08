mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

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
}
