/* use std::collections::HashMap;

fn main() {

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    /* let field_name_2 = String::from("Favorite color 2");
    let field_value_2 = String::from("Red"); */

    let mut map = HashMap::new();
    map.insert(&field_name, field_value);
    /* map.insert(field_name_2, field_value_2); */

    println!("{}", field_name);
} */

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

}