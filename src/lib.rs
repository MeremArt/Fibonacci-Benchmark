//recursive method time 21.65 ns
// pub fn fibonacci(n: u64) -> u64 {
//     if n == 0 {
//         return 0;
//     } else if n == 1 {
//         return 1;
//     } else {
//         return fibonacci(n - 1) + fibonacci(n - 2);
//     }
// }
//iterative method time 6.1730 ns
pub fn fibonacci(n: u64) -> u64 {
    let mut a = 0; // The first Fibonacci number, F(0)
    let mut b = 1; // The second Fibonacci number, F(1)
    
    for _ in 0..n { // Loop n times to calculate the nth Fibonacci number
        let temp = a; // Store the current value of a (which is F(n-2))
        a = b; // Move b into a, so a becomes F(n-1)
        b = temp + b; // Calculate the next Fibonacci number (F(n-1) + F(n-2)) and store in b
    }
    
    a // After n iterations, a holds the nth Fibonacci number
}
