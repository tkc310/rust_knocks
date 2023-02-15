fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn it_add() {
    assert_eq!(add(1, 3), 4);
}    


fn print_tuple_number(&(x, y): &(i32, i32)) {
    println!("tuple_number: ({}, {})", x, y);
}

fn print_if_even_odd(a: i32) {
    if a % 2 == 0 {
        println!("even")
    } else {
        println!("odd")
    };
}

fn print_loop_number() {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 3;
        }
    };

    println!("loop_number: {}", result)
}

fn print_while_number() {
    let mut number = 10;
    while number != 0 {
        number -= 1;
    };
    
    println!("while_number: {}", number);
}

fn print_for_number() {
    let items = [10, 20, 30, 40, 50];
    
    for item in items.iter() {
        println!("for_number: {}", item);
    }
}


struct User {
    name: String,
    mail: String,
}

fn build_user(mail: String, name: String) -> User {
    User {
        mail,
        name,
    }
}

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct Point<T> { x: T, y: T }

impl<T> Point<T> {
    fn xy(self) -> (T, T) {
        (self.x, self.y)
    }
}

enum Coin {
    Btc,
    Eth,
}

fn value_in_price(coin: Coin) -> String {
    match coin {
        Coin::Btc => String::from("BitCoin"),
        Coin::Eth => String::from("Ethereum"),
    }
}

fn main() {
    println!("add: {}", add(1,3));

    let point = (3,1);
    print_tuple_number(&point);

    print_if_even_odd(2);

    print_loop_number();

    print_while_number();

    print_for_number();

    let user = build_user("hoge@example.com".to_string(), "hoge".to_string());
    println!("user name: {}, mail: {}", user.name, user.mail);

    let square = Square::new(10, 2);
    println!("area: {}", square.area());

    let point_generics = Point::<u32>{ x: 1, y: 2 };
    let (x, y) = point_generics.xy();
    println!("point: {}, {}", x, y);

    let coin_1 =  value_in_price(Coin::Btc);
    let coin_2 =  value_in_price(Coin::Eth);
    println!("coin: {}, {}", coin_1, coin_2)
}
