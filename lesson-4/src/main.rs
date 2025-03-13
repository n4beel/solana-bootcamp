fn main() {
    fizz_buzz();
}

fn fizz_buzz(){
    for i in 1..=301{
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizz buzz");
        }
        else if i % 3 == 0 {
            println!("fizz");
        }
        else if i % 5 == 0 {
            println!("buzz");
        }
        else {
            println!("{}", i)
        }
    }
}