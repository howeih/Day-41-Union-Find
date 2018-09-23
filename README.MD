Day 41: Union-Find
<br>
Union-Find is a data structure that is capable of tracking and merging of disjoint sets. As a structure it is very important inside other algorithms like Prolog unification or percolation problem.
<br>
There are two signification improvements that can be made to speed the algorithm up, weighting and path compression. I’ve implemented path compression, only, the code is short and performance is pretty good.
<br>
run:
```
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
```
result:
```
item 0 -> component 9
item 1 -> component 9
item 2 -> component 9
item 3 -> component 3
item 4 -> component 9
item 5 -> component 9
item 6 -> component 9
item 7 -> component 7
item 8 -> component 8
item 9 -> component 9
```