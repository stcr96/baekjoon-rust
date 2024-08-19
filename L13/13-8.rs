// 11651번, 좌표 정렬하기 2

use std::io::{self, BufRead, BufReader, BufWriter, Write};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut points: Vec<Point> = Vec::new();
    let mut point = String::new();
    for _ in 0..n {
        point.clear();
        reader.read_line(&mut point).unwrap();
        let mut point_iter = point
            .trim()
            .split_whitespace()
            .map(|p| p.parse::<i32>().unwrap());
        let x = point_iter.next().unwrap();
        let y = point_iter.next().unwrap();
        let point = Point { x, y };
        points.push(point);
    }
    points.sort_by(|a, b| {
        if a.y != b.y {
            a.y.cmp(&b.y)
        } else {
            a.x.cmp(&b.x)
        }
    });
    
    let mut writer = BufWriter::new(io::stdout().lock());
    for point in points {
        writeln!(writer, "{} {}", point.x, point.y).unwrap();
    }
}