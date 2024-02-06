fn main() {
  //let mut s1 = String::from("foo");
  //let s2 = "bar";
  //s1.push_str(s2);
  //println!("s2 is {}", s2);
  //let s1 = String::from("hello, ");
  //let s2 = String::from("world!");
  //let s3 = s1 + &s2;
  //println!("s3 is {}", s3);
  let hello = "Здравствуйте";
  for c in hello.bytes() {
      println!("{}", c);
  }
}