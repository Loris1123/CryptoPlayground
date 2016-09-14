/// Implementation of the Euclidean Algorithm (https://en.wikipedia.org/wiki/Euclidean_algorithm)
/// to find the greates common divisor (GCD) of the numbers a and b.
pub fn calculate_gcd(first: i64, second: i64){
    println!("Not yet implemented");

    let mut a = first;
    let mut b = second;

    while b != 0{
        let t = b;
        b = a % b;
        a = t;
    }
    println!("GCD: {:?}", a);
}
