fn main() {
    let condition = true;
    let x = if condition { 5 } else { 6 };

    println!("{}", x);

    let condition = false;
    let x = if condition { 5 } else { 6 };

    println!("{}", x);

    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };

    println!("{result}");
}
