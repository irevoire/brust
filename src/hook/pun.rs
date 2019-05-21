use serenity::model::id::ChannelId;
// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::{Scheduler, TimeUnits};
// Import week days and WeekDay
use std::thread;
use std::time::Duration;

pub fn init() {
    let channel = ChannelId(560486862817591296); // tribunal des blagues

    let mut scheduler = Scheduler::new();

    scheduler.every(8.hours()).run(move || {
        println!("sending a new pun");
        let mut pun = reqwest::get("http:/pun.irevoire.ovh").unwrap();
        if !pun.status().is_success() {
            println!("server error when getting a new pun");
            return;
        }
        let pun = pun.text().unwrap();

        channel.say(pun).unwrap();
    });

    thread::spawn(move || loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(500));
    });
}
