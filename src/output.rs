pub fn wrap_day_results<T: FnOnce()>(day: u8, day_results: T) {
    println!("----------");
    println!("Day {}", day);
    day_results();
    println!("----------");
}
