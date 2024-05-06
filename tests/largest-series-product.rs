fn largest_product(s: &str, n: usize) -> Option<u64> {
    if n == 0 {
        return Some(1);
    }
    if n > s.len() {
        return None;
    }

    let digits: Vec<u64> = s.chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u64))
        .collect();

    let mut max_product = 0;
    for i in 0..=(digits.len() - n) {
        let product = digits[i..i + n].iter().product();
        if product > max_product {
            max_product = product;
        }
    }

    Some(max_product)
}

fn main() {
    let input = "1027839564";
    let n = 3;
    println!("Largest product for series of {} digits: {:?}", n, largest_product(input, n));

    let input = "73167176531330624919225119674426574742355349194934";
    let n = 6;
    println!("Largest product for series of {} digits: {:?}", n, largest_product(input, n));
}
