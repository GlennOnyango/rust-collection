use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();

    let mut numbers = vec![1, 2, 3];

    numbers.push(23);

    numbers.push(24);

    let third = &numbers[2];

    println!("The third number is {}", third);

    let third = &numbers.get(2);

    match third {
        Some(third) => println!("the number option is {third}"),
        None => println!("No number found"),
    }

    let wrong_var = &v.get(100);

    match wrong_var {
        Some(value) => println!("This is an inncorect address {value}"),
        None => println!("Number not found"),
    }

    for i in &numbers {
        println!(" This is number {i}")
    }

    //enum on the vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.23),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(value) => println!("This is an integer {value}"),
            SpreadsheetCell::Text(value) => println!("This is a text {value}"),
            SpreadsheetCell::Float(value) => println!("This is a float {value}"),
        }
    }

    //Creating a string

    //Two type  str slice and String

    let mut s = "Mwana dada".to_string();

    let mut s_supper = String::new();

    let mut s_string = String::from("Mwana dada");

    s.push_str(" Mzuri");

    println!("The new string s is {s}");

    //string concetination

    let new_string = s + &s_string;

    println!("{new_string}");

    let beautifull_name = String::from("Glenn");

    let western_name = String::from("Panyako");

    let combined_names = format!("{beautifull_name} - {western_name}");

    println!("{combined_names}");

    let hello = "Здравствуйте";

    for c in hello.chars() {
        println!("{c}");
    }

    //Hash maps

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let team_name = "Blue";

    let score = scores.get(team_name).copied().unwrap_or(0);

    println!("{team_name}{score}");

    for (key, value) in &scores {
        println!("{key} {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    scores.entry(String::from("Green")).or_insert(50);


    scores.entry(String::from("Blue")).or_insert(100);

    for (key, value) in &scores {
        println!("{key} - {value}");
    }

    let text = "hello world wonderful world";

    let mut map2 = HashMap::new();

    for word in text.split_whitespace() {
        println!("{word}");
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map2);

}
