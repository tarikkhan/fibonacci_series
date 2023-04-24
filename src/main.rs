fn main() {
    let ninth_number = generate_fibonacci(9);

    println!("{ninth_number}");
}


fn generate_fibonacci(position: u32) -> u32 {
    let mut initial = 0;
    let mut current = 1;

    if position == 1 {
        return initial;
    } else {
        for _number in 1..position {
            let temp = current;
            current = current + initial;
            initial = temp;
        }
    }

    initial
}
