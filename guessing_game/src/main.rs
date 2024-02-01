use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数字を当ててみてください");

    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("秘密の数字は: {}", secret_number);

    loop {
        println!("入力してください");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗しました");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("あなたの予想: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎます"),
            Ordering::Greater => println!("大きすぎます"),
            Ordering::Equal => {
                println!("当たりです");
                break;
            }
        }
    }
}