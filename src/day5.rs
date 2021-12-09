use std::cmp::{max, min};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve_part1(input: &str) -> usize {
    let mut map = HashMap::new();
    for x in input.lines() {
        if let (Some(x1), Some(y1), Some(x2), Some(y2)) =
            scan_fmt!(x, "{d},{d} -> {d},{d}", u32, u32, u32, u32)
        {
            if x1 == x2 {
                for y in min(y1, y2)..=max(y1, y2) {
                    *map.entry((x1, y)).or_insert(0) += 1;
                }
            } else if y1 == y2 {
                for x in min(x1, x2)..=max(x1, x2) {
                    *map.entry((x, y1)).or_insert(0) += 1;
                }
            }
        }
    }
    map.values().filter(|&&v| v >= 2).count()
}

pub fn solve(input: &str) -> usize {
    let mut map = HashMap::new();
    for x in input.lines() {
        if let (Some(x1), Some(y1), Some(x2), Some(y2)) =
            scan_fmt!(x, "{d},{d} -> {d},{d}", i32, i32, i32, i32)
        {
            if x1 == x2 {
                for y in min(y1, y2)..=max(y1, y2) {
                    *map.entry((x1, y)).or_insert(0) += 1;
                }
            } else if y1 == y2 {
                for x in min(x1, x2)..=max(x1, x2) {
                    *map.entry((x, y1)).or_insert(0) += 1;
                }
            } else {
                let dx = if x2 < x1 { -1 } else { 1 };
                let dy = if y2 < y1 { -1 } else { 1 };
                for i in 0..=(x2 - x1).abs() {
                    *map.entry((x1 + i * dx, y1 + i * dy)).or_insert(0) += 1;
                }
            }
        }
    }
    map.values().filter(|&&v| v >= 2).count()
}
