struct point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let integer = point { x: 5, y: 10.0 };
    //let result = largest(&number_list);
}

//fn largest<T>(list:&[T]) -> T {
//    let mut largest = list[0];
//
//    println!("iter: {:?}", list.iter());
//    for &item in list.iter() {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}
//fn largest_i32(list: &[i32]) -> i32 {
//    let mut largest = list[0];
//
//    for &item in list.iter() {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}
//
//fn largest_char(list: &[char]) -> char {
//    let mut largest = list[0];
//
//    for &item in list.iter() {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}