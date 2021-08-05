// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

// fn string_like(arg: impl ToString + std::fmt::Display) {
//     println!("{}", arg);
// }

// fn main() {
//     string_like("blue");
//     string_like("red".to_string());
//     string_like(String::from("hi"));
//     string_like("rust is fun!".to_owned());
//     // string_like("nice weather".into());
//     string_like(format!("Interpolation {}", "Station"));
//     string_like(&String::from("abc")[0..1]);
//     string_like("  hello there ".trim());
//     string_like("Happy Monday!".to_string().replace("Mon", "Tues"));
//     string_like("mY sHiFt KeY iS sTiCkY".to_lowercase());
// }
