use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use rand::Rng;

fn main() {
    File::open("hello.txt").expect("Failed to open hello.txt");
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {e:?}"),
        },
        _ => {
            panic!("Problem opening the file: {error:?}");
        }
    });
    println!("Hello, world!");
}
#[warn(dead_code)]
fn use_option() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    println!("{}", some_number.unwrap() + 9);
    println!("{}", some_char.unwrap());
    println!("{}", plus_one(some_number).unwrap() + 9);
}
#[warn(dead_code)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
#[warn(dead_code)]
fn en() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    fn route(ip_kind: IpAddrKind) {
        println!("{:?}", ip_kind)
    }
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[warn(dead_code)]
fn meth() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let mut rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    // 不可变借用
    println!("Can rect1 hold rect2? {}", rect1.can_hold0(&rect2));
    // 可变借用
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&mut rect3));
    let rect4 = Rectangle::square(60);
    println!("Can rect1 hold rect4? {}", rect1.can_hold0(&rect4));
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold0(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn can_hold(&self, other: &mut Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[warn(dead_code)]
fn call_first_word() {
    let s = String::from("132");
    let idx = first_word(&s);
    println!("First word: {}", idx);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

#[warn(dead_code)]
fn demo() {
    let penguin_data = "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data
   ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // 声明一个 fields 变量，类型是 Vec
        // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
        // <_>表示 Vec 中的元素类型由编译器自行推断
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            // 输出到标准错误输出
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
        //
        // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
        //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
        //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
        //
        // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }
}

#[warn(dead_code)]
fn guess_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
