#[derive(Debug, PartialEq, Copy, Clone)]
struct Item(usize, usize, usize);

fn closure(g: &Vec<Vec<Vec<usize>>>, kernel: &Vec<Item>) -> Vec<Item>
{
    let mut j = kernel.to_vec();
    let mut added = vec![false; g.len()];
    loop {
        let n = j.len();
        for i in 0..n
        {
            let b = g[j[i].0][j[i].1][j[i].2];
            if added[b] == false
            {
                added[b] = true;
                for p in 0..g[b].len() { j.push(Item(b, p, 0)); }
            }
        }
        if n == j.len() { return j; }
    }
}

fn goto(g: &Vec<Vec<Vec<usize>>>, k: &Vec<Item>, t: usize) -> Vec<Item>
{
    let mut j: Vec<Item> = vec![];
    let l = closure(g, k);

    for i in l
    {
        if g[i.0][i.1][i.2] == t { j.push(i); }
    }

    for i in &mut j { i.2 += 1; }

    return closure(g, &j);
}

/* Terminals */
// 40. ( -> terminal
// 41. ) -> terminal
// 42. * -> terminal
// 43. + -> terminal
// 105. i -> terminal

/* Non-Terminals */
// 256. null -> 
// 257. $ -> accept
// 258. S -> E $
// 259. E -> E + T | T
// 260. T -> T * F | F
// 261. F -> ( E ) | i
#[test]
fn test_goto()
{
    let mut grammar: Vec<Vec<Vec<usize>>> = vec![vec![]; 262];

    grammar[258] = vec![vec![259, 257, 256]];
    grammar[259] = vec![vec![259, 43, 260, 256], vec![260, 256]];
    grammar[260] = vec![vec![260, 42, 261, 256], vec![261, 256]];
    grammar[261] = vec![vec![40, 259, 41, 256], vec![105, 256]];

    {// I0
        let kernel = vec![Item(258, 0, 0)];

        // goto(I0, E) -> I1
        let c = goto(&grammar, &kernel, 259);
        assert_eq!(c[0], Item(258, 0, 1));
        assert_eq!(c[1], Item(259, 0, 1));
        assert_eq!(2, c.len());

        // goto(I0, T) -> I2
        let c = goto(&grammar, &kernel, 260);
        assert_eq!(c[0], Item(259, 1, 1));
        assert_eq!(c[1], Item(260, 0, 1));
        assert_eq!(2, c.len());

        // goto(I0, F) -> I3
        let c = goto(&grammar, &kernel, 261);
        assert_eq!(c[0], Item(260, 1, 1));
        assert_eq!(1, c.len());

        // goto(I0, '(') -> I4
        let c = goto(&grammar, &kernel, 40);
        assert_eq!(c[0], Item(261, 0, 1));
        assert_eq!(c[1], Item(259, 0, 0));
        assert_eq!(c[2], Item(259, 1, 0));
        assert_eq!(c[3], Item(260, 0, 0));
        assert_eq!(c[4], Item(260, 1, 0));
        assert_eq!(c[5], Item(261, 0, 0));
        assert_eq!(c[6], Item(261, 1, 0));
        assert_eq!(7, c.len());

        // goto(I0, i) -> I5
        let c = goto(&grammar, &kernel, 105);
        assert_eq!(c[0], Item(261, 1, 1));
        assert_eq!(1, c.len());

        // goto(I0, +) -> null
        let c = goto(&grammar, &kernel, 43);
        assert_eq!(0, c.len());

        // goto(I0, *) -> null
        let c = goto(&grammar, &kernel, 42);
        assert_eq!(0, c.len());

        // goto(I0, ')') -> null
        let c = goto(&grammar, &kernel, 41);
        assert_eq!(0, c.len());

        // goto(I0, $) -> null
        let c = goto(&grammar, &kernel, 257);
        assert_eq!(0, c.len());
    }
    
    {// I1
        let kernel = vec![Item(258, 0, 1), Item(259, 0, 1)];

        // goto(I1, +) -> I6
        let c = goto(&grammar, &kernel, 43);
        assert_eq!(c[0], Item(259, 0, 2));
        assert_eq!(c[1], Item(260, 0, 0));
        assert_eq!(c[2], Item(260, 1, 0));
        assert_eq!(c[3], Item(261, 0, 0));
        assert_eq!(c[4], Item(261, 1, 0));
        assert_eq!(5, c.len());
    }
}

#[test]
fn test_closure()
{
    let mut grammar: Vec<Vec<Vec<usize>>> = vec![vec![]; 262];

    grammar[258] = vec![vec![259, 257, 256]];
    grammar[259] = vec![vec![259, 43, 260, 256], vec![260, 256]];
    grammar[260] = vec![vec![260, 42, 261, 256], vec![261, 256]];
    grammar[261] = vec![vec![40, 259, 41, 256], vec![105, 256]];

    // I0
    let kernel = vec![Item(258, 0, 0)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(258, 0, 0));
    assert_eq!(c[1], Item(259, 0, 0));
    assert_eq!(c[2], Item(259, 1, 0));
    assert_eq!(c[3], Item(260, 0, 0));
    assert_eq!(c[4], Item(260, 1, 0));
    assert_eq!(c[5], Item(261, 0, 0));
    assert_eq!(c[6], Item(261, 1, 0));

    assert_eq!(7, c.len());

    // I1
    let kernel = vec![Item(258, 0, 1), Item(259, 0, 1)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(258, 0, 1));
    assert_eq!(c[1], Item(259, 0, 1));
    assert_eq!(2, c.len());

    // I2
    let kernel = vec![Item(259, 1, 1), Item(260, 0, 1)];
    let c = closure(&grammar, &kernel);

    assert_eq!(c[0], Item(259, 1, 1));
    assert_eq!(c[1], Item(260, 0, 1));
    assert_eq!(2, c.len());

    // I3
    let kernel = vec![Item(260, 1, 1)];
    let c = closure(&grammar, &kernel);

    assert_eq!(c[0], Item(260, 1, 1));
    assert_eq!(1, c.len());

    // I4
    let kernel = vec![Item(261, 0, 1)];
    let c = closure(&grammar, &kernel);

    assert_eq!(c[0], Item(261, 0, 1));
    assert_eq!(c[1], Item(259, 0, 0));
    assert_eq!(c[2], Item(259, 1, 0));
    assert_eq!(c[3], Item(260, 0, 0));
    assert_eq!(c[4], Item(260, 1, 0));
    assert_eq!(c[5], Item(261, 0, 0));
    assert_eq!(c[6], Item(261, 1, 0));
    assert_eq!(7, c.len());
    
    // I5
    let kernel = vec![Item(261, 1, 1)];
    let c = closure(&grammar, &kernel);

    assert_eq!(c[0], Item(261, 1, 1));
    assert_eq!(1, c.len());

    // I6
    let kernel = vec![Item(259, 0, 2)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(259, 0, 2));
    assert_eq!(c[1], Item(260, 0, 0));
    assert_eq!(c[2], Item(260, 1, 0));
    assert_eq!(c[3], Item(261, 0, 0));
    assert_eq!(c[4], Item(261, 1, 0));
    assert_eq!(5, c.len());

    // I7
    let kernel = vec![Item(260, 0, 2)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(260, 0, 2));
    assert_eq!(c[1], Item(261, 0, 0));
    assert_eq!(c[2], Item(261, 1, 0));
    assert_eq!(3, c.len());

    // I8
    let kernel = vec![Item(259, 0, 1), Item(261, 0, 2)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(259, 0, 1));
    assert_eq!(c[1], Item(261, 0, 2));
    assert_eq!(2, c.len());

    // I9
    let kernel = vec![Item(259, 0, 3), Item(260, 0, 1)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(259, 0, 3));
    assert_eq!(c[1], Item(260, 0, 1));
    assert_eq!(2, c.len());

    // I10
    let kernel = vec![Item(260, 0, 3)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(260, 0, 3));
    assert_eq!(1, c.len());

    // I11
    let kernel = vec![Item(261, 0, 3)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(261, 0, 3));
    assert_eq!(1, c.len());
}

fn main()
{
    let v1 = vec![Item(4, 5, 3)];
    let v2 = vec![Item(4, 5, 3)];
    if v1 != v2 {
        println!("not equal");
    }
}
