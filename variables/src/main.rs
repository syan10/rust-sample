
use std::io;

const THREE_HOURS_IN_SECONDS:u32 = 60 * 60 * 3;

fn main() {

    println!("the value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    let x = 5;

    println!("the value of x is: {}", x);

    let x = x+1;
    {
        let x = x*2;
        println!("the value of x-1 is: {}", x);
    }

    println!("the value of x-3 is: {}", x);

    let space = "    ";
    println!("space is {}", space);


    let space = space.len();
    println!("space is {}", space);

    let guess_num:u32 = "32".parse().expect("not a number!");
    println!("guess_num is {}", guess_num);

    let x1:u8 = 255;
    println!("x1 is {}", x1);

    //bool
    let t = true;
    println!("t is {}", t);

    let f:bool = false;
    println!("f is {}", f);

    //char
    let mut c = 'z';
    println!("c is {}", c);
    c = 'Z';
    println!("c is {}", c);
    c = '好';
    println!("c is {}", c);


    //tup
    let tup:(i32, f64, u8) = (500, 6.4, 1);
    println!("tup is {}, {}, {}", tup.0, tup.1, tup.2);

    //array
    let a = [1, 2, 3, 4, 5];
    println!("a[2] is {}", a[2]);

    let b: [i32; 5]= [1, 2, 3, 4, 5];
    println!("b[2] is {}", b[2]);

    let c = [4; 5];
    println!("c[2] is {}", c[2]);



    //test_array1();
    test_array2();
}

fn test_array1() {
    println!("run the test of test_array1");

    let a: [u8; 3] = [1, 2, 3];

    println!("Please input the index of array a[]");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("not a number");

    let element = a[index];
    println!("the value of element at index {} is: {}", index, element);
}

fn test_array2() {
    println!("run the test of test_array2");

    let mut arr: [u8; 1000] = [0; 1000];

    for a in &arr {
        let b = a+1;
        print!("{} ", b);
    }
    println!();

    for i in 0..arr.len() {
        arr[i] = arr[i] + i as u8;
        println!("i as u8 is {}, data is {} ", i as u8, arr[i]);
    }
    println!();
}