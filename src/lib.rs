pub fn fizz_buzz (num: i32){
    if num%3==0 && num%5==0 {
        println!("FizzBuzz");
    }else if num%3==0 {
        println!("Fizz");
    }else if num%5==0 {
        println!("Buzz");
    }else {
        println!("{}", num)
    }
}
