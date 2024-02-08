#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    //let v1 = vec![1, 2, 3];
//
    //let v1_iter = v1.iter();
//
    //for val in v1_iter {
    //    println!("Got: {}", val);
    //}
//
    //let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    //println!("{:?}", v2);

    let counter = Counter::new().skip(2).take(4);
    for val in counter {
        println!("Got: {}", val);
    }
}