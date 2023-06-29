fn main() {
    for i in 1u32..=100 {

        let divisible_by_three = i % 3 == 0;
        let divisible_by_five = i % 5 == 0;

        if divisible_by_three && divisible_by_five {
            println!("FizzBuzz");
        } else if divisible_by_three {
            println!("Fizz");
        } else if divisible_by_five {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}