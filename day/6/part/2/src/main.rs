use std::io;

fn main() -> io::Result<()> {
    let race = Race {
        time: 49877895,
        best_distance: 356137815021882,
    };
    println!("{}", compute_winning_race_count(&race));

    Ok(())
}

struct Race {
    time: u64,
    best_distance: u64,
}

fn compute_winning_race_count(race: &Race) -> u64 {
    (0..=race.time)
        .map(|time_holding_button| compute_distance(race.time, time_holding_button))
        .filter(|distance| distance > &race.best_distance)
        .count() as u64
}

fn compute_distance(race_time: u64, time_holding_button: u64) -> u64 {
    let remaining_time = race_time - time_holding_button;
    let speed = time_holding_button;
    remaining_time * speed
}
