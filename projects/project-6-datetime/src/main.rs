use chrono::prelude::*;

fn main() {
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let local: DateTime<Local> = Local::now();
    let month: &str = months[local.month()];
    println!("{}-{}", local.month(), local.year());
}

// months[local.month() - 1]
