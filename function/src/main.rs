fn main() {
    foo(5);

    let mut y = 6;

    let x = {
        y = 7;
        y + 1
    };

    println!("main: the value of x is {}", x);

    let z = plus_one(x);
    println!("main: the value of z is {}", z);

    plus_two(&y);
    println!("main: the value of y is {}", y);
}

fn foo(x: i32) {
    println!("foo: the value of x is {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_two(x: &i32) -> i32 {
    *x + 2
}
