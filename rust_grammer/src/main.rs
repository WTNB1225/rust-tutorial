//use std::io;

fn main() {
    //let x = 5;
    //println!("The value of x is: {}", x);
    //let x = x + 1;
    //println!("The value of x is: {}", x);
    //{
    //    let x = x * 2;
    //    println!("The value of x is: {}", x);
    //}
//
    //println!("The value of x is: {}", x);
//
    //let _guess : u32 = "42".parse().expect("数字を入力してください");
    //
//
    //let sum = 5 + 10;
//
    //let difference = 95.5 - 4.3;
//
    //let product = 4 * 30;
//
    //let quotient = 56.7 / 32.2;
    //let floored = 2 / 3;
//
    //let remainder = 43 % 5;
//
    //println!("sum: {}, difference: {}, product: {}, quotient:{}, floored: {}, remainder: {}", 
    //        sum, difference, product, quotient, floored, remainder);
//
    //let c = 'z';
    //let z = 'ℤ';
    //let heart_eyed_cat = '😻';   
    //println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);
//
    //let tup: (i32, f64, u8) = (500, 6.4, 1);
    //println!("tup: {:?}", tup);
    //println!("tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);
//
    //let a = [1, 2, 3, 4, 5];
    //println!("a: {:?}", a);

    //array();

    another_function(5);
    print_labeled_measurement(5, 'm');

    let y = {
        let x = 3;
        x + 1
    };

    let z = five();

    let retrun_value = plus_one(10);

    println!("yの値は: {}", y);

    println!("zの値は: {}", z);

    println!("return_valueの値は: {}", retrun_value);

    let num = 3;

    if num < 5 {
        println!("条件は真です");
    } else {
        println!("条件は偽です");
    }
}

//fn array() {
//    let a  = [1, 2, 3, 4, 5];
//    println!("添え字を入力してください");
//
//    let mut index = String::new();
//
//    io::stdin()
//        .read_line(&mut index)
//        .expect("読み込みに失敗しました");
//
//    let index: usize = index.trim().parse().expect("数字を入力してください");
//
//    println!("{}番目の添え字: {}",index, a[index]);
//}

fn another_function(x: i32) {
    println!("xの値は: {}", x);
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("値は: {}{}", x, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}