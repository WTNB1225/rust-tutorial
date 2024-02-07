pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    // "（{}さんの文章をもっと読む）"
    format!("(Read more from {}...)", self.summarize_author())
}
}
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {

  fn summarize_author(&self) -> String {
    format!("{}", self.author)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {

  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
}
}

pub fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}

//fn some_function<T, U>(t: &T, u: &U) -> i32 
//  where T: Display + Clone,
//        U: Clone + Debug
//  {
//    0
//  }

fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  }
}

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}