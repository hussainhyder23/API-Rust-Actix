// pub struct InputUser {
//     pub id : i32,
//     pub first_name: String,
//     pub last_name: String,
// }
// struct Vec2 {
//     x: f64, // 64-bit floating point, aka "double precision"
//     y: f64,
// }

// fn main(){
//     let u = InputUser{
//         id:52,
//         first_name:"Rust".to_string(),
//         last_name:"Rusty".to_string(),
//     };
//     let v1 = Vec2 { x: 1.0, y: 3.0 };
//     let v2 = Vec2 { y: 2.0, x: 4.0 };
//     let v3 = Vec2 {
//     x: 14.0,
//     ..v2
// };
//     println!("{:?}", v3.y);
// }
struct Number {
    odd: bool,
    value: i32,
}

fn main() {
    let one = Number { odd: true, value: 3 };
    let two = Number { odd: false, value: 4 };
    print_number(one);
    print_number(two);
}
fn print_number(n: Number) {
    match n.value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("{}", n.value),
    }
}
// fn print_number(n: Number) {
//     if let Number { odd:true, value } = n {
//         println!("Odd number: {}", value);
//     } else if let Number { odd:false, value } = n {
//         println!("Even number: {}", value);
//     }
// }