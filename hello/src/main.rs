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
    // // if we want to have a changable variable, use mut
    // let mut num: i8 = 3;
    // num = 5;

    // arrays, borrowing and refercning
    // slice is a subset of an array

    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // a slice
    // & is a way to reference the original array
    //  and borrow it
    // slice from index 1.4
    let slice = &arr[0..4];
    // print!("{:?}", slice);

    // strings
    // let str: &str = "hello word";
    // let string: String = String::from("Hello worlds");

    // flow controls
    // let n = 3;
    // if n > 0 {
    //     println!("n is positive");
    // } else if n < 0 {
    //     println!("n is negative");
    // } else {
    //     println!("n is zero");
    // }

    // for loop
    // for i in 0..100 {
    //     if i % 2 == 0 {
    //         println!("Even number loop {}", i);
    //     }

    //     println!("Number {} ", i);
    // }

    // while loop
    // we add in mut to make it mutable
    // let mut i: u8 = 4;
    // while i < 10 {
    //     println!("i is {}", i);
    //     // increment
    //     i += 1;
    //     if i == 9 {
    //         break;
    //     }
    // }

    //  match - switch
    // let n: i32 = 3;
    // match n {
    //     0 => println!("n is zero"),
    //     1 | 2 => println!("n is one or two"),
    //     3..=4 => println!("n is three or four"),
    //     _ => println!("n is something else"),
    // }

    // structs
    let make = String::from("Toyota");
    let model = String::from("Corolla");
    let car: Car = Car {
        make,
        model: model,
        year: 2020,
    };

    car.return_car_info();
}
// struct information -> like a class
struct Car {
    make: String,
    model: String,
    year: u16,
}

// adding method to the struct class
impl Car {
    fn return_car_info(&self) {
        println!(
            "Car make: {}, model: {}, year: {}",
            self.make, self.model, self.year
        );
    }
}

// function
pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    // return statement does not need ;
    digit == 0
}
