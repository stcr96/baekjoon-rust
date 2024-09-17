// 2164번, 카드2

use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::collections::VecDeque;

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut cards: VecDeque<usize> = (1..=n).collect();
    while cards.len() != 1 {
        cards.pop_front().unwrap();
        let card = cards.pop_front().unwrap();
        cards.push_back(card);
    }
    writeln!(writer, "{}", cards[0]).unwrap();
}