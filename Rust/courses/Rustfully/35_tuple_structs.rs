struct Color(u16, u16, u16);
struct Date(u16, u16, u16);

fn display_date(date: &Date) {
    println!("The date is: {}/{}/{}", date.0, date.1, date.2)
}

fn main() {
    let blue = Color(0, 0, 255);
    let date = Date(21, 12, 2025);

    display_date(&date);
}