
fn fibonacci(old: i64, new: i64) -> i64 {
    old+new
}

pub fn challenge2() {
    println!("Challenge 2 - Even Fibonacci numbers");
    println!("By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.");
    let mut answer: i64 = 2;

    /*
    Initial values for a fibinachi of 1 and 2 making 3, so we can immediately go 2 and 3
    */
    let mut old_value: i64 = 2;
    let mut new_value: i64 = fibonacci(1, old_value);
    let mut temp: i64;

    while new_value < 4000001 {
        //shift values down
        temp = old_value;
        old_value = new_value;
        new_value = fibonacci(temp, old_value);

        if new_value % 2 == 0 {
            answer += new_value;
        }

    }

    println!("Real answer is: 4613732, my answer {}", answer);
}