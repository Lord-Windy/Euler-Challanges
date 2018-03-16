



/*

Using suggested algorithm now. Mine worked but was very slow

*/

pub fn challenge3() {
    println!("Largest prime factor");
    println!("What is the largest prime factor of the number 600851475143 ?");

    let mut n: i64 = 600851475143;
    let mut max_factor: i64;
    let mut factor: i64;
    let mut last_factor: i64 = 1;

    if n % 2 == 0 {
        last_factor = 2;
        n = n/2;
        while n % 2 == 0 {
            n = n/2;
        }
    }

    factor = 3;

    max_factor = (n as f64).sqrt() as i64;

    while n > 1 && factor <= max_factor {
        if n % factor == 0 {
            n = n/factor;

            last_factor = factor;

            while n % factor == 0 {
                n = n/factor;
            }
            max_factor = (n as f64).sqrt() as i64;
        }
        factor = factor+2;
    }

    let answer;

    if n == 1 {
        answer = last_factor;
    } else {
        answer = n;
    }

    println!("Real answer is: 6857, my answer {}", answer);

}

//Original poor performing. Recoded to a more optimal solution suggested by the site.

/* fn find_next_prime(prime_list: &Vec<i64>, to_check: i64) -> bool{

    for i in prime_list.iter() {
        if to_check % i == 0 {
            return false
        }
    }

    true
}

let mut number: i64 = 600851475143;
    let mut max_factor: i64 = ((number as f64).sqrt() as i64);
    println!("Max Factor: {}", max_factor);
    let mut prime_list: Vec<i64> = Vec::new();
    let mut prime_factors: Vec<i64> = Vec::new();


    //First set of primes
    prime_list.push(2);
    prime_factors.push(2);

    //next prime number to queue it up
    let mut to_check: i64 = 7;

    loop {
        // first check to see if to check is now greater than half of the number we are searching for halved
        // we don't want to waste cycles
        if to_check > max_factor {
            break;
        }

        //now check if the to_check is a prime number
        if find_next_prime(&prime_list, to_check) {
            prime_list.push(to_check);
            //println!("Found Prime: {}", to_check);
            //great it is, now check to see if this is a prime factor
            if number % to_check == 0 {

                prime_factors.push(to_check);
            }
        }

        //lastly increment to_check
        to_check += 2;
    }
    */
