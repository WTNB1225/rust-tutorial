use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("予想値は1から100の間でなければなりませんが、{}でした", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

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
    
        if guess < 1 || guess > 100 {
            println!("数字を1から100の間で入力してください");
            continue;
        }
    
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