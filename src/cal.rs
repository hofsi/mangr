use std::collections::HashMap;
use chrono::{DateTime,FixedOffset};
use colored::Colorize;
/*
TODO:
 - Add Calender function to add or subtract tasks
 - Add Basic Calender Prining function
 - Add Calender Serialize, Save and Load Funcion
 - Add Basic CLI functionality
 - Add Ratatui


*/

enum WeekDay{
    Monday,
    Tuesday,
    Wendsday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Clone, Copy, Debug)]
pub enum Topic{
    Home,
    Work,
    Friends,
}

struct Task{
    name: String,
    done: bool,
    time: Option<f32>,
    length: Option<f32>,
    priority: i8,
    topic: Option<Topic>,
}
impl Task{
    pub fn new(name: String, time: f32, length: f32, priority: i8, topic: Topic,) -> Self{
        Self { time: Some(time), length: Some(length), priority, topic: Some(topic), name, done: false } 
    }
    pub fn new_short(name: String) -> Self{
        Self { time: None, length: None, priority: 0 , topic: None, name, done: false } 
    }

    pub fn modify(&mut self, name: Option<String>, time: Option <f32>, length: Option <f32>, priority: Option <i8>, topic: Option<Topic>) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(time) = time {
            self.time = Some(time);
        }
        if let Some(length) = length {
            self.length = Some(length);
        }
        if let Some(priority) = priority {
            self.priority = priority;
        }
        if let Some(topic) = topic {
            self.topic = Some(topic);
        } 
    }
}


pub struct Day{
    tasks: Vec<Task>,
}

impl Day{
    pub fn new() -> Self {
        Self {         
            tasks: Vec::new() 
        }
    }
    
    pub fn check_task(&self, name: &String) -> Option<usize>{
        for (index,task) in self.tasks.iter().enumerate(){
            if &task.name == name{
                return Some(index);
            }
        }
        None
    }
    
    pub fn new_task(&mut self, name: String, time: f32, length: f32, priority: i8, topic: Topic,) -> Result<(), &'static str>{
        if self.check_task(&name).is_none() {
            self.tasks.push(Task::new(name, time, length, priority, topic));
            return Ok(());
        }
        Err("Task Name already exists") 
    }
    
    pub fn mod_task(&mut self, name: String, new_name: Option<String>, time: Option<f32>, length: Option<f32>, priority: Option<i8>, topic: Option<Topic>,) -> Result<(),&str>{
        if new_name.is_some(){
            if self.check_task(&new_name.as_ref().unwrap()).is_some() {
                return Err("New Name Already Exists");
            }
        }
        let id: usize = match self.check_task(&name){
            Some(val) => {val}
            None => {return Err("Task Does not Exist");}
        };
        self.tasks[id].modify(new_name,time,length,priority,topic);
        Ok(())
    }
}

pub struct Calender{
    pub days: HashMap<u128, Day>,
}

impl Calender {
    pub fn new() -> Self{
        Calender {days: HashMap::new()}
    }
    pub fn new_day(&mut self, day: u128) -> Result<(),&str>{
        if !self.days.contains_key(&day) { 
            self.days.insert(day,Day::new());
            return Ok(());
        }
        Err("Day already exists")
    }
     
}

pub fn print_week_from_day(calender: &Calender, date: (u64, u8, u16)) {
    let days = calc_days_since_0ad(date.0, date.1, date.2); 
    let week_start = days - get_weekday(days) as u128;
    let week_days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    for i in 0..7{
        println!("{}:",week_days[i]);
        if calender.days.contains_key(&(week_start + i as u128)){
            print_day(calender.days.get(&(week_start + i as u128)).unwrap())   
        }
    }
}

pub fn print_day(day: &Day){
    for elm in day.tasks.iter(){
        println!("");
        print_task(elm)
    }
    println!("");
}

pub fn print_task(task: &Task){
    println!("{} {}",task.name.underline(), task.done.to_string().red());
    if task.time.is_some() && task.length.is_some(){
        println!("{} {}",task.time.unwrap(), task.length.unwrap());
    }
    if task.topic.is_some(){
        println!("{} {:?}",task.priority, task.topic.unwrap())
    }
}


//This looses and regains 10 days between 1582 and 1927 depending when the switch to the gregorian
//calender happend in the respective country.
//This assumes that there is a year 0, which there is not.
pub fn calc_days_since_0ad(year: u64, month: u8, days: u16) -> u128{
    let leap_years = &year/4;
    let mut leap_centuary:i128 = 0;
    if year > 1582{
        let year_i128 = year as i128;
        leap_centuary = year_i128 /400 + 2 - year_i128 /100;
    }

    let prev_month = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let mut monthly_days: u128 = prev_month[month as usize]; 
    if leap_year(&year) && month > 2 {
        monthly_days += 1;
    }
    year as u128 * 365 + (leap_years as i128 + leap_centuary) as u128 + monthly_days as u128 + days as u128
}

fn leap_year(year: &u64) -> bool{
    if year%4==0 && (!(year%100==0 && year > &1582) || year%400==0){
      return true  
    }
    false
}

pub fn get_weekday(days: u128) -> u8{
    ((days+2)%7) as u8 
} 
