use std::env::{args,Args};
fn main() {
    // println!("Hello, world!");
    let mut args: Args = args();
    let first_operand = args.nth(1).unwrap().parse::<f32>().unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second_operand = args.nth(0).unwrap().parse::<f32>().unwrap();
  
    // println!(" {} {} {} ",first_operand,operator,second_operand)
    let result =match operator {
        '+'=> first_operand + second_operand,
        '-' => first_operand - second_operand,
        'x' | '*' => first_operand * second_operand,
        '/'=> first_operand / second_operand,
        _=> 0.0};
    println!("{:?}",result)
}
