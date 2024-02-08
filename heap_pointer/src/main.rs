use std::rc::{Rc, Weak};
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        // leafのstrong_count = {}, weak_count = {}
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            // branchのstrong_count = {}, weak_count = {}
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

//fn main() {
//    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
//    println!("a initial rc count = {}", Rc::strong_count(&a));
//    println!("a next item = {:?}", a.tail());
//
//    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
//    println!("a rc count after b creation = {}", Rc::strong_count(&a));
//    println!("b initial rc count = {}", Rc::strong_count(&b));
//    println!("b next item = {:?}", b.tail());
//
//    if let Some(link) = a.tail() {
//        *link.borrow_mut() = Rc::clone(&b);
//    }
//    // aを変更後のbの参照カウント = {}
//    println!("b rc count after changing a = {}", Rc::strong_count(&b));
//    // aを変更後のaの参照カウント = {}
//    println!("a rc count after changing a = {}", Rc::strong_count(&a));
//}

//#[derive(Debug)]
//enum List {
//    Cons(Rc<RefCell<i32>>, Rc<List>),
//    Nil,
//}
//
//use List::{Cons, Nil};
//use std::rc::Rc;
//use std::cell::RefCell;
//
//fn main() {
//    let value = Rc::new(RefCell::new(5));
//
//    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
//
//    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
//    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
//
//    *value.borrow_mut() += 10;
//
//    println!("a after = {:?}", a);
//    println!("b after = {:?}", b);
//    println!("c after = {:?}", c);
//}

//enum List {
//    Cons(i32, Rc<List>),
//    Nil,
//}
//
//use List::{Cons, Nil};
//use std::rc::Rc;

//fn main() {
//    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//    println!("参照カウント後 = {}", Rc::strong_count(&a));
//    let b = Cons(3, Rc::clone(&a));
//    println!("参照カウント後 = {}", Rc::strong_count(&a));
//    {
//        let c = Cons(4, Rc::clone(&a));
//        println!("参照カウント後 = {}", Rc::strong_count(&a));
//    }
//    println!("参照カウント後 = {}", Rc::strong_count(&a));
//}

//use List::{Cons, Nil};
//use std::ops::Deref;
//
//enum List {
//    Cons(i32, Box<List>),
//    Nil,
//}
//
//struct MyBox<T>(T);
//
//impl <T> MyBox<T> {
//    fn new(x: T) -> MyBox<T> {
//        MyBox(x)
//    }
//}
//
//impl<T> Deref for MyBox<T> {
//    type Target = T;
//
//    fn deref(&self) -> &T {
//        &self.0
//    }
//}
//
//struct CustomSmartPointer {
//    data: String,
//}
//
//impl Drop for CustomSmartPointer {
//    fn drop(&mut self) {
//        println!("CustomSmartPointerが破棄されました: {}", self.data);
//    }
//}
//
//fn main() {
//    let c = CustomSmartPointer { data: String::from("some data") };
//    println!("CustomSmartPointerを作成しました。");
//    drop(c);
//    println!("cを手動で破棄しました。");
//    let d = CustomSmartPointer { data: String::from("other data") };
//    println!("CustomSmartPointerを作成しました。");
//}




//fn main() {
//    let x = 5;
//    let y = MyBox::new(x);
//
//    assert_eq!(5, x);
//    assert_eq!(5, *y); // *(y.deref())と同じ
//
//    let m = MyBox::new(String::from("Rust"));
//    hello(&m);
//}
//
//fn hello(name: &str) {
//    println!("Hello, {}!", name);
//}

//fn main() {
//    let list = Cons(1,
//        Box::new(Cons(2,
//            Box::new(Cons(3,
//                Box::new(Nil))))));
//
//}