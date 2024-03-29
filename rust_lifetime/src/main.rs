//fn main() {
//    let string1 = String::from("abcd");
//    let string2 =  "xyz";
//
//    let result = longest(string1.as_str(), string2);
//    println!("The longest string is {}", result);
//}
//
//fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
//    if x.len() > y.len() {
//        x
//    } else {
//        y
//    }
//}

use std::fmt::Display;

fn longest_with_an_announcement<'a,T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("重要なアナウンス: {}", announcement);
        self.part
    }
}

fn main() {
    let result = longest_with_an_announcement("abcd", "xyz", "Today is someone's birthday");
    println!("The longest string is {}", result);
    let s: &'static str = "I have a static lifetime.";
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
