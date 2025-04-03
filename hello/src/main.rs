fn main() {
    // //  unsigned
    // let age: u8 = 10;

    // // Signed i2,i4,i8,i16,i32,i64
    // let signed: i64 = -59;

    // // float
    // let decimal_number: f32 = 1.0;

    // // char
    // let _char_code = "c";

    // // unicode

    // // boolean
    // let is_today_blue: bool = true;

    // println!(
    //     "Printing variables-  signed: {}, unsigned: {}, float:{}",
    //     signed, age, decimal_number
    // );

    // println!("his age is {}", age);

    // println!("Hello, world!, is today blue? : {}", is_today_blue);

    // // arrays
    // // create an array of type u8 with a size of 5
    // let arr: [u8; 5] = [1, 2, 3, 5, 6];

    // // printing the structure of an array - like looping through an array
    // println!("{:?}", arr);

    // // tuple
    // // hold elemnets of different values together
    // let example_tuple: (i8, bool, f32) = (10, false, 3.0);
    // // or we could create a tuple using
    // let _tup_2 = (1, 4, true);

    // // printing out a tuple
    // println!("first tup {}", example_tuple.1);
    // // printing structure
    // println!("{:?}", example_tuple);

    // // destructuring a tuple
    // let (a, b, c) = example_tuple;
    // println!("a: {}, b: {}, c: {}", a, b, c);

    // functions
    // print!("{}", is_even(3));

    // mutability
    // if we want to have a changable variable, use mut
    let mut num: i8 = 3;
    num = 5;
}

pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    // return statement does not need ;
    digit == 0
}
