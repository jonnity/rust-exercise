enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn is_holiday(day: Day) -> bool {
    match day {
        Day::Saturday => true,
        Day::Sunday => true,
        _ => false,
    }
}

fn callendar_color(day: Day) -> String {
    if let Day::Saturday = day {
        "blue".to_string()
    } else if let Day::Sunday = day {
        "red".to_string()
    } else {
        "black".to_string()
    }
}

fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // ↓はエラーになる
    // let sum = x + y;

    let sum = match y {
        None => x,
        Some(value) => x + value,
    };
    println!("sum is {}", sum);

    println!("Sunday is holiday: {}", is_holiday(Day::Monday));
    println!(
        "Saturday is painted with {} in callendar",
        callendar_color(Day::Saturday)
    )
}
