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

fn generate_fibonacci_series (how_many: u32) -> Vec<u32> {
    let mut fibonacci_series: Vec<u32> = vec![0, 1];
    if how_many < 2 {
        return fibonacci_series;
    }
    
    let deducted_how_many = how_many - 2;
    
    
    
    for _number in 1..=deducted_how_many {
        let (a, b ) = match &fibonacci_series[..] {
          [.., a, b] => (a, b),
          _ => panic!("vector need at least two numbers to start the series"),
        };
        fibonacci_series.push( a + b );
    }
    fibonacci_series
}
