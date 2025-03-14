fn main() {
    let sum = 5 + 10;
    println!("The sum is: {}", sum);

    let difference = 95.5 - 4.3;
    println!("The difference is: {}", difference);

    let product = 4 * 30;
    println!("The product is: {}", product);

    let quotient = 56.7 / 32.2;
    println!("The quotient is: {}", quotient);
    let truncated = -5 / 3;
    println!("The truncated quotient is: {}", truncated);

    let remainder = 43 % 5;
    println!("The remainder is: {}", remainder);



    //Boolean
    let t = true;
    println!("The value of t is: {}", t);

    let f: bool = false;
    println!("The value of f is: {}", f);

    //Character
    let c = 'z';
    println!("The value of c is: {}", c);
    let z: char = 'ℤ'; // with explicit type annotation
    println!("The value of z is: {}", z);
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);


    //Compound Types
    //Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;


    //Array
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];

    let first = a[0];
    let second = a[1];

}
