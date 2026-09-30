fn main() {
    let a:i32 = 18;
    let b:i32 = 26;

    println!("Value of a: {} ",a);
    println!("Value of b: {} ",b);

    let mut res = a>b ;
    println!("a is greater than b: {} ",res);

    res = a<b ;
    println!("a is lesser than b: {} ",res) ;

    res = a>=b ;
    println!("a is greater than or equal to b: {} ",res);

    res = a<=b;
    println!("a is lesser than or equal to b: {} ",res) ;

    res = a==b ;
    println!("a is equal to b: {} ",res) ;

    res = a!=b ;
    println!("a is not equal to b: {} ",res);
}
