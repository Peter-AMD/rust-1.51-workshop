fn main() {
    let mut city_names: Vec<&str> = vec!["Pythonia", "Javasburg", "C by the Sea", "Rustville"];

    let last_city: &str = match city_names.pop() {
        Some(value) => {
            value
        }
        None => {
            ""
        }
    };

    if last_city.starts_with("R") {
        println!("“{}” starts with an R!", last_city);
    } else {
        println!("“{}” doesn't start with R", last_city);
    }

    city_names.push(last_city);
    // Can we add a loop here inside the print?
    println!("Here is the full list of cities:");

    for city in city_names.iter() {
        println!("{}", city);
    }
}
