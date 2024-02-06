//fn main() {
//    let mut v: Vec<i32> = Vec::new();
//    let v2 = vec![1, 2, 3];
//    v.push(5);
//    v.push(6);
//    v.push(7);
//    v.push(8);
//    println!("{:?}", v);
//}

//fn main() {
//    let v = vec![1, 2, 3, 4, 5];
//
//    let third: &i32 = &v[2];
//    println!("The third element is {}", third);
//
//    match v.get(2) {
//        //                      "3つ目の要素は{}です"
//        Some(third) => println!("The third element is {}", third),
//        //               "3つ目の要素はありません。"
//        None => println!("There is no third element."),
//    }
//}

//fn main() {
//    let mut v = vec![100, 32, 57];
//    v.push(6);
//    let first = &v[0];
//    println!("The first element is: {}", first);
//}
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
    ];

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{:?}", row);
    }
}