use chrono::{DateTime, Local};

fn main() {
    let local: DateTime<Local> = Local::now();
    let day = local.format("%a");
    let date = local.format("%e");
    let month = local.format("%b");
    let year = local.format("%Y");
    let hour = local.format("%H");
    let minute = local.format("%M");

    println!("{} {} {} {}, {}:{}", day, date, month, year, hour, minute);
}
