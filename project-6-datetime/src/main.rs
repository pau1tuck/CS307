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

/*
use chrono::{DateTime, Utc};

fn main() {
    let local: DateTime<Utc> = Utc::local();

    println!("UTC local is: {}", local);
    println!("UTC local in RFC 2822 is: {}", local.to_rfc2822());
    println!("UTC local in RFC 3339 is: {}", local.to_rfc3339());
    println!("UTC local in a custom format is: {}", local.format("%a %b %e %T %Y"));
}
 */
