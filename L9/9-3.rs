// 9506번, 약수들의 합

use std::io;

fn main() {
    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).unwrap();
        let n: i32 = n.trim().parse().unwrap();
        
        if n == -1 { break }
        
        let mut divisor_except_me: Vec<i32> = Vec::new();
        // 본인을 제외한다면 반복을 절반으로 줄일 수 있다.
        for i in 1..=n/2 {
            if n % i == 0 { divisor_except_me.push(i); }
        }
        
        if is_perfect(n, &mut divisor_except_me) {
            print!("{n} = 1");
            for x in divisor_except_me {
                if x == 1 { continue }
                print!(" + {x}");
            }
            print!("\n");
        } else {
            println!("{n} is NOT perfect.")
        }
    }
}

fn is_perfect(num: i32, divisor: &mut Vec<i32>) -> bool {
    if num == divisor.iter().sum() {
        true
    } else {
        false
    }
}
