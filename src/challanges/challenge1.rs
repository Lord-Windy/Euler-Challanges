pub fn challenge1() {
    println!("Challenge 1 - Multiples of 3 and 5");
    println!("If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23. Find the sum of all the multiples of 3 or 5 below 1000.");

    let mut answer: i64 = 0;

    for i in 1..1000 {
        if i % 3 == 0 {
            answer += i as i64;
            continue;
        }
        if i % 5 == 0 {
            answer += i as i64;
        }
    }

    println!("Real answer is: 233168, my answer {}", answer);
}