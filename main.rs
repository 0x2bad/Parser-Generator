#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
struct Item(usize, usize, usize);

fn closure(g: &Vec<Vec<Vec<usize>>>, kernel: &Vec<Item>) -> Vec<Item>
{
    let mut j = kernel.to_vec();
    let mut added = vec![false; g.len()];
    loop
    {
        let n = j.len();
        for i in 0..n
        {
            let b = g[j[i].0][j[i].1][j[i].2];
            if added[b] == false
            {// item can be added since not found
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

    for i in k
    {
        if g[i.0][i.1][i.2] == t { j.push(*i); }
    }

    for i in &mut j { i.2 += 1; }

    return closure(g, &j);
}


fn items(g: &Vec<Vec<Vec<usize>>>) -> Vec<Vec<Item>>
{
    let mut c = vec![closure(g, &vec![Item(257, 0, 0)])];
    loop
    {
        println!("in loop");
        let n = c.len();
        for i in 0..n
        {
            for j in 0..g.len()
            {
                let mut r = goto(g, &c[i], j);
                if ! r.is_empty()
                {
                    r.sort_unstable();
                    if let Err(idx) = c.binary_search(&r)
                    {   c.insert(idx, r);   }
                }
            }
        }
        if n == c.len()
        {   return c   };
    }
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

    grammar[S] = vec![vec![E, ACCEPT, END]];
    grammar[E] = vec![vec![E, ADD, T, END], vec![T, END]];
    grammar[T] = vec![vec![T, MULT, F, END], vec![F, END]];
    grammar[F] = vec![vec![LP, E, RP, END], vec![ID, END]];

    {// I0
        let kernel = vec![Item(S, 0, 0)];

        // goto(I0, E) -> I1
        let c = goto(&grammar, &closure(&grammar, &kernel), E);
        assert_eq!(c[0], Item(S, 0, 1));
        assert_eq!(c[1], Item(E, 0, 1));
        assert_eq!(2, c.len());

        // goto(I0, T) -> I2
        let c = goto(&grammar, &closure(&grammar, &kernel), T);
        assert_eq!(c[0], Item(E, 1, 1));
        assert_eq!(c[1], Item(T, 0, 1));
        assert_eq!(2, c.len());

        // goto(I0, F) -> I3
        let c = goto(&grammar, &closure(&grammar, &kernel), F);
        assert_eq!(c[0], Item(T, 1, 1));
        assert_eq!(1, c.len());

        // goto(I0, '(') -> I4
        let c = goto(&grammar, &closure(&grammar, &kernel), LP);
        assert_eq!(c[0], Item(F, 0, 1));
        assert_eq!(c[1], Item(E, 0, 0));
        assert_eq!(c[2], Item(E, 1, 0));
        assert_eq!(c[3], Item(T, 0, 0));
        assert_eq!(c[4], Item(T, 1, 0));
        assert_eq!(c[5], Item(F, 0, 0));
        assert_eq!(c[6], Item(F, 1, 0));
        assert_eq!(7, c.len());

        // goto(I0, i) -> I5
        let c = goto(&grammar, &closure(&grammar, &kernel), ID);
        assert_eq!(c[0], Item(F, 1, 1));
        assert_eq!(1, c.len());

        // goto(I0, +) -> null
        let c = goto(&grammar, &closure(&grammar, &kernel), ADD);
        assert_eq!(0, c.len());

        // goto(I0, *) -> null
        let c = goto(&grammar, &closure(&grammar, &kernel), MULT);
        assert_eq!(0, c.len());

        // goto(I0, ')') -> null
        let c = goto(&grammar, &closure(&grammar, &kernel), RP);
        assert_eq!(0, c.len());

        // goto(I0, $) -> null
        let c = goto(&grammar, &closure(&grammar, &kernel), ACCEPT);
        assert_eq!(0, c.len());
    }
    
    {// I1
        let kernel = vec![Item(S, 0, 1), Item(E, 0, 1)];

        // goto(I1, +) -> I6
        let c = goto(&grammar, &closure(&grammar, &kernel), ADD);
        assert_eq!(c[0], Item(E, 0, 2));
        assert_eq!(c[1], Item(T, 0, 0));
        assert_eq!(c[2], Item(T, 1, 0));
        assert_eq!(c[3], Item(F, 0, 0));
        assert_eq!(c[4], Item(F, 1, 0));
        assert_eq!(5, c.len());


    }
}

static END: usize = 256;
static ACCEPT: usize = 257;
static S: usize = 258;
static E: usize = 259;
static T: usize = 260;
static F: usize = 261;
static LP: usize = 40;
static RP: usize = 41;
static MULT: usize = 42;
static ADD: usize = 43;
static ID: usize = 105;

#[test]
fn test_closure()
{
    let mut grammar: Vec<Vec<Vec<usize>>> = vec![vec![]; 262];

    grammar[S] = vec![vec![E, ACCEPT, END]];
    grammar[E] = vec![vec![E, ADD, T, END], vec![T, END]];
    grammar[T] = vec![vec![T, MULT, F, END], vec![F, END]];
    grammar[F] = vec![vec![LP, E, RP, END], vec![ID, END]];

    // I0
    let kernel = vec![Item(S, 0, 0)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(S, 0, 0));
    assert_eq!(c[1], Item(E, 0, 0));
    assert_eq!(c[2], Item(E, 1, 0));
    assert_eq!(c[3], Item(T, 0, 0));
    assert_eq!(c[4], Item(T, 1, 0));
    assert_eq!(c[5], Item(F, 0, 0));
    assert_eq!(c[6], Item(F, 1, 0));

    assert_eq!(7, c.len());

    // I1
    let kernel = vec![Item(S, 0, 1), Item(E, 0, 1)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(S, 0, 1));
    assert_eq!(c[1], Item(E, 0, 1));
    assert_eq!(2, c.len());

    // I2
    let kernel = vec![Item(E, 1, 1), Item(T, 0, 1)];
    let c = closure(&grammar, &kernel);

    assert_eq!(c[0], Item(E, 1, 1));
    assert_eq!(c[1], Item(T, 0, 1));
    assert_eq!(2, c.len());

    // I3
    let kernel = vec![Item(T, 1, 1)];
    let c = closure(&grammar, &kernel);

    assert_eq!(c[0], Item(T, 1, 1));
    assert_eq!(1, c.len());

    // I4
    let kernel = vec![Item(F, 0, 1)];
    let c = closure(&grammar, &kernel);

    assert_eq!(c[0], Item(F, 0, 1));
    assert_eq!(c[1], Item(E, 0, 0));
    assert_eq!(c[2], Item(E, 1, 0));
    assert_eq!(c[3], Item(T, 0, 0));
    assert_eq!(c[4], Item(T, 1, 0));
    assert_eq!(c[5], Item(F, 0, 0));
    assert_eq!(c[6], Item(F, 1, 0));
    assert_eq!(7, c.len());
    
    // I5
    let kernel = vec![Item(F, 1, 1)];
    let c = closure(&grammar, &kernel);

    assert_eq!(c[0], Item(F, 1, 1));
    assert_eq!(1, c.len());

    // I6
    let kernel = vec![Item(E, 0, 2)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(E, 0, 2));
    assert_eq!(c[1], Item(T, 0, 0));
    assert_eq!(c[2], Item(T, 1, 0));
    assert_eq!(c[3], Item(F, 0, 0));
    assert_eq!(c[4], Item(F, 1, 0));
    assert_eq!(5, c.len());

    // I7
    let kernel = vec![Item(T, 0, 2)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(T, 0, 2));
    assert_eq!(c[1], Item(F, 0, 0));
    assert_eq!(c[2], Item(F, 1, 0));
    assert_eq!(3, c.len());

    // I8
    let kernel = vec![Item(E, 0, 1), Item(F, 0, 2)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(E, 0, 1));
    assert_eq!(c[1], Item(F, 0, 2));
    assert_eq!(2, c.len());

    // I9
    let kernel = vec![Item(E, 0, 3), Item(T, 0, 1)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(E, 0, 3));
    assert_eq!(c[1], Item(T, 0, 1));
    assert_eq!(2, c.len());

    // I10
    let kernel = vec![Item(T, 0, 3)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(T, 0, 3));
    assert_eq!(1, c.len());

    // I11
    let kernel = vec![Item(F, 0, 3)];
    let c = closure(&grammar, &kernel);
    assert_eq!(c[0], Item(F, 0, 3));
    assert_eq!(1, c.len());
}

fn main()
{
    let r = vec![5, 3, 67, 33, 65];

    println!("{:?}", r.iter().find(|&&x| x == 33));
    println!("{:?}", r);
}
