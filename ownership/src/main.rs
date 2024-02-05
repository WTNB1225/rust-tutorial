// Title: 所有権
//fn main() {
//    let s = String::from("hello");
//
//    takes_ownership(s);
//
//    let x = 5;
//
//    makes_copy(x);
//
//}
//
//fn takes_ownership(some_string: String) {
//    println!("{}", some_string);
//}
//
//fn makes_copy(some_integer: i32) {
//    println!("{}", some_integer);
//}

// 戻り値とスコープ
//fn main() {
//    let s1 = gives_ownership();
//
//    let s2 = String::from("hello");
//
//    let s3 = takes_and_gives_back(s2);
//
//}
//
//fn gives_ownership() -> String {
//
//    let some_string = String::from("hello");
//
//    some_string
//}
//
//
//fn takes_and_gives_back(a_string: String) -> String {
//    
//    a_string
//}

//fn main() {
//    let s1 = String::from("hello");
//
//    let (s2, len) = caluculate_length(s1);
//
//    println!("{}の長さは: {}", s2, len);
//
//}
//
//fn caluculate_length(s: String) -> (String, usize) {
//    let length = s.len();
//
//    (s, length)
//}

//fn main() {
//    let s1 = String::from("hello");
//    let len = caluculate_length(&s1);
//
//    println!("{}の長さは: {}", s1, len);
//}
//
//fn caluculate_length(s: &String) -> usize {
//    s.len()
//}
//
//fn main() {
//    let mut s = String::from("hello");
//    change(&mut s);
//}
//
//fn change(some_string: &mut String) {
//    some_string.push_str(", world");
//}

fn main() {
  let s = String::from("hello world");
  let _word = first_word(&s);
  println!("{}", _word);
}

fn first_word(s: &String) -> &str{
  let bytes = s.as_bytes();
  
  for (i, &item) in  bytes.iter().enumerate() {
    if item == b' '{
      return &s[0..i];
    }
  }
  &s[..]
}