//use std::collections::HashMap;
//
//fn main() {
//  //let mut scores = HashMap::new();
//  //scores.insert(String::from("Blue"), 10);
//  //scores.insert(String::from("Yellow"), 50);
//  //println!("{:?}", scores);
//  let teams = vec![String::from("Blue"), String::from("Yellow"), String::from("Red")];
//  let initial_scores = vec![10, 50, 70];
//  let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
//  println!("{:?}", scores);
//  let team_name = String::from("Blue");
//  let score = scores.get(&team_name);
//  println!("{:?}", score);
//  for (key, value) in &scores {
//    println!("{}: {}", key, value);
//  }
//}

use std::collections::HashMap;

fn main() {
  let text = "hello world wonderful world";

  let mut map = HashMap::new();
  
  for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      println!("{:?}", count);
      *count += 1;
  }
  
  println!("{:?}", map);
}