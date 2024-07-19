// 5622번, 다이얼

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    
    
    let time: u32 = input.chars().map(|x| {
        if "ABC".contains(x) { return 3 }
        else if "DEF".contains(x) { return 4 }
        else if "GHI".contains(x) { return 5 }
        else if "JKL".contains(x) { return 6 }
        else if "MNO".contains(x) { return 7 }
        else if "PQRS".contains(x) { return 8 }
        else if "TUV".contains(x) { return 9 }
        else if "WXYZ".contains(x) { return 10 }
        else { return 0 };
    })
    .sum::<u32>();
    
    println!("{}", time);
}