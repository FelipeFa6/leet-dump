fn fizz_buzz(x: u8) -> String{

    let mut resp = String::new();
    if x%3 == 0 && x%5 == 0 {
        resp = resp + "FizzBuzz";
    }
    else if x%3 == 0 {
        resp = resp + "Fizz";
    }
    else if x%5 == 0 {
        resp = resp +  "Buzz";
    }

    return resp;
}


fn main() {
    // FizzBuzz implementation
    for x in 1..=101 {
        println!("{} {}", x, fizz_buzz(x));
    }
}
