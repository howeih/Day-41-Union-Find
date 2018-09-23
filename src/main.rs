use std::iter::FromIterator;
use std::iter::Iterator;

fn find(data: &mut Vec<usize>, i: usize) -> usize{
    if i != data[i]{
        let next_i = data[i];
        data[i] = find(data, next_i);
    }
    data[i]
}

fn union(data: &mut Vec<usize>, i:usize, j:usize){
    let pi = find(data, i);
    let pj = find(data, j);
    if pi != pj {
        data[pi] = pj;
    }
}

fn main() {
    let n = 10usize;
    let mut data = Vec::<usize>::from_iter(0..n);
    let connections:Vec<(usize, usize)> = vec![(0, 1), (1, 2), (0, 9), (5, 6), (6, 4), (5, 9)];
    for c in connections{
        union(&mut data, c.0, c.1);
    }
    for i in 0..n{
        println!("item {} -> component {}", i, find(&mut data, i));
    }
}
