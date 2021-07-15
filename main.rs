// 0. S -> E
// 1. E -> E + T | T
// 2. T -> T * F | F
// 3. F -> ( E ) | i
// 4. + -> terminal
// 5. * -> terminal
// 6. ( -> terminal
// 7. ) -> terminal
// 8. i -> terminal

#[derive(Debug, PartialEq)]
struct Item(usize, usize, usize);

fn closure(G: &Vec<Vec<Vec<usize>>>, J: &mut Vec<Item>)
{
    let mut added: [bool; 10] = [false; 10];
    loop {
        let n = J.len();
        for i in 0..n
        {
            let B = G[J[i].0][J[i].1][J[i].2];
            if added[B] == false
            {
                added[B] = true;
                for p in 0..G[B].len()
                {
                    J.push(Item(B, p, 0));
                }
            }
        }
        if n == J.len() { return; }
    }
}

fn goto(G: &Vec<Vec<Vec<usize>>>, J: &mut Vec<Item>, t: usize)
{
    println!("nothing");
}

#[test]
fn test_closure()
{
let grammar: Vec<Vec<Vec<usize>>> = vec![
    vec![vec![1, 9]],                   // S   
    vec![vec![1, 4, 2, 9], vec![2, 9]], // E   
    vec![vec![2, 5, 3, 9], vec![3, 9]], // T   
    vec![vec![6, 1, 7, 9], vec![8, 9]], // F   
    vec![], // +
    vec![], // *
    vec![], // (
    vec![], // )
    vec![], // i
    vec![]  // end
];

    // I0
    let mut kernel = vec![Item(0, 0, 0)];
    closure(&grammar, &mut kernel);
    assert_eq!(kernel[0], Item(0, 0, 0));
    assert_eq!(kernel[1], Item(1, 0, 0));
    assert_eq!(kernel[2], Item(1, 1, 0));
    assert_eq!(kernel[3], Item(2, 0, 0));
    assert_eq!(kernel[4], Item(2, 1, 0));
    assert_eq!(kernel[5], Item(3, 0, 0));
    assert_eq!(kernel[6], Item(3, 1, 0));
    assert_eq!(7, kernel.len());

    // I1
    let mut kernel = vec![Item(0, 0, 1), Item(1, 0, 1)];
    closure(&grammar, &mut kernel);
    assert_eq!(kernel[0], Item(0, 0, 1));
    assert_eq!(kernel[1], Item(1, 0, 1));
    assert_eq!(2, kernel.len());

    // I2
    let mut kernel = vec![Item(1, 1, 1), Item(2, 0, 1)];
    closure(&grammar, &mut kernel);

    assert_eq!(kernel[0], Item(1, 1, 1));
    assert_eq!(kernel[1], Item(2, 0, 1));
    assert_eq!(2, kernel.len());

    // I3
    let mut kernel = vec![Item(2, 1, 1)];
    closure(&grammar, &mut kernel);

    assert_eq!(kernel[0], Item(2, 1, 1));
    assert_eq!(1, kernel.len());

    // I4
    let mut kernel = vec![Item(3, 0, 1)];
    closure(&grammar, &mut kernel);

    assert_eq!(kernel[0], Item(3, 0, 1));
    assert_eq!(kernel[1], Item(1, 0, 0));
    assert_eq!(kernel[2], Item(1, 1, 0));
    assert_eq!(kernel[3], Item(2, 0, 0));
    assert_eq!(kernel[4], Item(2, 1, 0));
    assert_eq!(kernel[5], Item(3, 0, 0));
    assert_eq!(kernel[6], Item(3, 1, 0));
    assert_eq!(7, kernel.len());
    
    // I5
    let mut kernel = vec![Item(3, 1, 1)];
    closure(&grammar, &mut kernel);

    assert_eq!(kernel[0], Item(3, 1, 1));
    assert_eq!(1, kernel.len());

    // I6
    let mut kernel = vec![Item(1, 0, 2)];
    closure(&grammar, &mut kernel);
    assert_eq!(kernel[0], Item(1, 0, 2));
    assert_eq!(kernel[1], Item(2, 0, 0));
    assert_eq!(kernel[2], Item(2, 1, 0));
    assert_eq!(kernel[3], Item(3, 0, 0));
    assert_eq!(kernel[4], Item(3, 1, 0));
    assert_eq!(5, kernel.len());

    // I7
    let mut kernel = vec![Item(2, 0, 2)];
    closure(&grammar, &mut kernel);
    assert_eq!(kernel[0], Item(2, 0, 2));
    assert_eq!(kernel[1], Item(3, 0, 0));
    assert_eq!(kernel[2], Item(3, 1, 0));
    assert_eq!(3, kernel.len());

    // I8
    let mut kernel = vec![Item(1, 0, 1), Item(3, 0, 2)];
    closure(&grammar, &mut kernel);
    assert_eq!(kernel[0], Item(1, 0, 1));
    assert_eq!(kernel[1], Item(3, 0, 2));
    assert_eq!(2, kernel.len());

    // I9
    let mut kernel = vec![Item(1, 0, 3), Item(2, 0, 1)];
    closure(&grammar, &mut kernel);
    assert_eq!(kernel[0], Item(1, 0, 3));
    assert_eq!(kernel[1], Item(2, 0, 1));
    assert_eq!(2, kernel.len());

    // I10
    let mut kernel = vec![Item(2, 0, 3)];
    closure(&grammar, &mut kernel);
    assert_eq!(kernel[0], Item(2, 0, 3));
    assert_eq!(1, kernel.len());

    // I11
    let mut kernel = vec![Item(3, 0, 3)];
    closure(&grammar, &mut kernel);
    assert_eq!(kernel[0], Item(3, 0, 3));
    assert_eq!(1, kernel.len());
}

fn main()
{
    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 6, 8];
    v1.extend(v2);

}
