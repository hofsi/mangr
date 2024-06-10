use cal::calc_days_since_0ad;

mod cal;
fn main() {
    let mut calender = cal::Calender::new();
    calender.new_day(calc_days_since_0ad(2024, 5, 11));
    calender.new_day(calc_days_since_0ad(2024, 5, 12));
    calender.new_day(calc_days_since_0ad(2024, 5, 13));
    calender.days.get_mut(&calc_days_since_0ad(2024, 5, 11))
        .unwrap() 
        .new_task("Test Task".to_string(), 0.0,0.0,0, cal::Topic::Home);
   calender.days.get_mut(&calc_days_since_0ad(2024, 5, 12))
        .unwrap() 
        .new_task("Test Taskeritis".to_string(), 0.0,0.0,0, cal::Topic::Home);
   calender.days.get_mut(&calc_days_since_0ad(2024, 5, 13))
        .unwrap() 
        .new_task("Test Task".to_string(), 0.0,0.0,0, cal::Topic::Home);
    cal::print_week_from_day(&calender,(2024, 5, 12));
    println!("{}",calc_days_since_0ad(2024, 5, 13)); 
}

