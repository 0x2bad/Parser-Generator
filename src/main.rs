#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
struct Item(usize, usize, usize);

struct Parser {
    lr: Vec<Vec<Vec<usize>>>,
    rr: Vec<Vec<Vec<usize>>>,
}

impl Parser
{
    fn first_rec(&self, t: usize) -> Vec<usize>
    {
        if self.rr[t].is_empty()
        {   return vec![t];   }

        let mut r = vec![];
        for i in &self.rr[t]
        {   r.append(&mut self.first_rec(i[0]));   }
        return r;
    }

    fn first(&self, t: usize) -> Vec<usize>
    {
        let mut f = self.first_rec(t);
        f.sort_unstable();
        f.dedup();
        return f;
    }

    fn generate_follow(&self) -> Vec<Vec<usize>>
    {
        let mut f = vec![vec![]; self.lr.len()];
        for i in 257..self.lr.len() - 1
        {
            println!("token {}", token(i));
            for j in &self.lr[i]
            {
                println!("new option");
                for k in 0..j.len() - 2
                {
                    let fi = self.first(j[k+1]);
                    for m in fi
                    {
                        if let Err(idx) = f[j[k]].binary_search(&m)
                        {   f[j[k]].insert(idx, m);   }
                    }
                }
            }
        }

        // I'm not sure if I need the 'while' loop and flag
        // For now I'll use them to be safe
        let mut flag = true;
        while flag == true
        {
            flag = false;
            for i in 257..self.lr.len() - 1
            {
                for j in &self.lr[i]
                {// everything in f[i] goes into f[j[j.len() - 2]]
                    for k in 0..f[i].len()
                    {
                        let e = f[i][k];
                        if let Err(idx) = f[j[j.len() - 2]].binary_search(&e)
                        {
                            f[j[j.len() - 2]].insert(idx, e);
                            flag = true;
                        }
                    }
                }
            }
        }
        return f;
    }

    fn closure(&self, kernel: &Vec<Item>) -> Vec<Item>
    {
        let mut j = kernel.to_vec();
        let mut added = vec![false; self.lr.len()];
        loop
        {
            let n = j.len();
            for i in 0..n
            {
                let b = self.lr[j[i].0][j[i].1][j[i].2];
                if added[b] == false
                {// item can be added since not found
                    added[b] = true;
                    for p in 0..self.lr[b].len() { j.push(Item(b, p, 0)); }
                }
            }
            if n == j.len() { return j; }
        }
    }

    fn goto(&self, k: &Vec<Item>, t: usize) -> Vec<Item>
    {
        let mut j: Vec<Item> = vec![];

        for i in k
        {
            if self.lr[i.0][i.1][i.2] == t { j.push(*i); }
        }

        for i in &mut j { i.2 += 1; }

        return self.closure(&j);
    }

    fn items(&self) -> Vec<Vec<Item>>
    {
        let mut c = vec![self.closure(&vec![Item(257, 0, 0)])];
        loop
        {
            let n = c.len();
            for i in 0..n
            {
                for j in 0..self.lr.len() - 1
                {
                    let mut r = self.goto(&c[i], j);
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
}

/* Terminals */
// 40. ( -> terminal
// 41. ) -> terminal
// 42. * -> terminal
// 43. + -> terminal
// 105. i -> terminal

/* Non-Terminals */
// 256. $ -> accept
// 257. S -> E $
// 258. E -> E + T | T
// 259. T -> T * F | F
// 260. F -> ( E ) | i
// 261. null -> 

#[test]
fn test_first_and_follow()
{
    let mut grammar: Vec<Vec<Vec<usize>>> = vec![vec![]; 262];

    grammar[S] = vec![vec![E, ACCEPT, END]];
    grammar[E] = vec![vec![E, ADD, T, END], vec![T, END]];
    grammar[T] = vec![vec![T, MULT, F, END], vec![F, END]];
    grammar[F] = vec![vec![LP, E, RP, END], vec![ID, END]];

    let mut grammar2: Vec<Vec<Vec<usize>>> = vec![vec![]; 264];

    grammar2[S] = vec![vec![E, ACCEPT, END]];
    grammar2[E] = vec![vec![T, E2, END]];
    grammar2[T] = vec![vec![F, T2, END]];
    grammar2[F] = vec![vec![LP, E, RP, END], vec![ID, END]];
    grammar2[E2] = vec![vec![END], vec![ADD, T, E2, END]];
    grammar2[T2] = vec![vec![END], vec![MULT, F, T2, END]];

    let p = Parser {
        lr: grammar,
        rr: grammar2,
    };

    let f = p.first(E);
    assert_eq!(f, vec![LP, ID]);

    let f = p.first(T);
    assert_eq!(f, vec![LP, ID]);

    let f = p.first(E2);
    assert_eq!(f, vec![ADD, END]);

    let f = p.first(T2);
    assert_eq!(f, vec![MULT, END]);

    let f = p.first(ADD);
    assert_eq!(f, vec![ADD]);

    let f = p.generate_follow();

    assert_eq!(f[S], vec![]);
    assert_eq!(f[E], vec![RP, ADD, ACCEPT]);
    assert_eq!(f[T], vec![RP, MULT, ADD, ACCEPT]);
    assert_eq!(f[F], vec![RP, MULT, ADD, ACCEPT]);
}

#[test]
fn test_items()
{
    let mut grammar: Vec<Vec<Vec<usize>>> = vec![vec![]; 262];

    grammar[S] = vec![vec![E, END]];
    grammar[E] = vec![vec![E, ADD, T, END], vec![T, END]];
    grammar[T] = vec![vec![T, MULT, F, END], vec![F, END]];
    grammar[F] = vec![vec![LP, E, RP, END], vec![ID, END]];

    let mut grammar2: Vec<Vec<Vec<usize>>> = vec![vec![]; 264];

    grammar2[S] = vec![vec![E, ACCEPT, END]];
    grammar2[E] = vec![vec![T, E2, END]];
    grammar2[T] = vec![vec![F, T2, END]];
    grammar2[F] = vec![vec![LP, E, RP, END], vec![ID, END]];
    grammar2[E2] = vec![vec![END], vec![ADD, T, E2, END]];
    grammar2[T2] = vec![vec![END], vec![MULT, F, T2, END]];

    let p = Parser {
        lr: grammar,
        rr: grammar2,
    };

    let c = p.items();

    // I still need to work on this
    for i in c
    {
        println!("{:?}", i);
    }
}

#[test]
fn test_goto()
{

    let mut grammar: Vec<Vec<Vec<usize>>> = vec![vec![]; 262];
  
    grammar[S] = vec![vec![E, ACCEPT, END]];
    grammar[E] = vec![vec![E, ADD, T, END], vec![T, END]];
    grammar[T] = vec![vec![T, MULT, F, END], vec![F, END]];
    grammar[F] = vec![vec![LP, E, RP, END], vec![ID, END]];

    let mut grammar2: Vec<Vec<Vec<usize>>> = vec![vec![]; 264];

    grammar2[S] = vec![vec![E, ACCEPT, END]];
    grammar2[E] = vec![vec![T, E2, END]];
    grammar2[T] = vec![vec![F, T2, END]];
    grammar2[F] = vec![vec![LP, E, RP, END], vec![ID, END]];
    grammar2[E2] = vec![vec![END], vec![ADD, T, E2, END]];
    grammar2[T2] = vec![vec![END], vec![MULT, F, T2, END]];

    let p = Parser {
        lr: grammar,
        rr: grammar2,
    };

    {// I0
        let kernel = vec![Item(S, 0, 0)];

        // goto(I0, E) -> I1
        let c = p.goto(&p.closure(&kernel), E);
        assert_eq!(c[0], Item(S, 0, 1));
        assert_eq!(c[1], Item(E, 0, 1));
        assert_eq!(2, c.len());

        // goto(I0, T) -> I2
        let c = p.goto(&p.closure(&kernel), T);
        assert_eq!(c[0], Item(E, 1, 1));
        assert_eq!(c[1], Item(T, 0, 1));
        assert_eq!(2, c.len());

        // goto(I0, F) -> I3
        let c = p.goto(&p.closure(&kernel), F);
        assert_eq!(c[0], Item(T, 1, 1));
        assert_eq!(1, c.len());

        // goto(I0, '(') -> I4
        let c = p.goto(&p.closure(&kernel), LP);
        assert_eq!(c[0], Item(F, 0, 1));
        assert_eq!(c[1], Item(E, 0, 0));
        assert_eq!(c[2], Item(E, 1, 0));
        assert_eq!(c[3], Item(T, 0, 0));
        assert_eq!(c[4], Item(T, 1, 0));
        assert_eq!(c[5], Item(F, 0, 0));
        assert_eq!(c[6], Item(F, 1, 0));
        assert_eq!(7, c.len());

        // goto(I0, i) -> I5
        let c = p.goto(&p.closure(&kernel), ID);
        assert_eq!(c[0], Item(F, 1, 1));
        assert_eq!(1, c.len());

        // goto(I0, +) -> null
        let c = p.goto(&p.closure(&kernel), ADD);
        assert_eq!(0, c.len());

        // goto(I0, *) -> null
        let c = p.goto(&p.closure(&kernel), MULT);
        assert_eq!(0, c.len());

        // goto(I0, ')') -> null
        let c = p.goto(&p.closure(&kernel), RP);
        assert_eq!(0, c.len());

        // goto(I0, $) -> null
        let c = p.goto(&p.closure(&kernel), ACCEPT);
        assert_eq!(0, c.len());
    }
    
    {// I1
        let kernel = vec![Item(S, 0, 1), Item(E, 0, 1)];

        // goto(I1, +) -> I6
        let c = p.goto(&p.closure(&kernel), ADD);
        assert_eq!(c[0], Item(E, 0, 2));
        assert_eq!(c[1], Item(T, 0, 0));
        assert_eq!(c[2], Item(T, 1, 0));
        assert_eq!(c[3], Item(F, 0, 0));
        assert_eq!(c[4], Item(F, 1, 0));
        assert_eq!(5, c.len());

    }
}

static ACCEPT: usize = 256;
static S: usize = 257;
static E: usize = 258;
static T: usize = 259;
static F: usize = 260;
static END: usize = 261;
static E2: usize = 262;
static T2: usize = 263;
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

    let mut grammar2: Vec<Vec<Vec<usize>>> = vec![vec![]; 264];

    grammar2[S] = vec![vec![E, ACCEPT, END]];
    grammar2[E] = vec![vec![T, E2, END]];
    grammar2[T] = vec![vec![F, T2, END]];
    grammar2[F] = vec![vec![LP, E, RP, END], vec![ID, END]];
    grammar2[E2] = vec![vec![END], vec![ADD, T, E2, END]];
    grammar2[T2] = vec![vec![END], vec![MULT, F, T2, END]];

    let p = Parser {
        lr: grammar,
        rr: grammar2,
    };

    // I0
    let kernel = vec![Item(S, 0, 0)];
    let c = p.closure(&kernel);
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
    let c = p.closure(&kernel);
    assert_eq!(c[0], Item(S, 0, 1));
    assert_eq!(c[1], Item(E, 0, 1));
    assert_eq!(2, c.len());

    // I2
    let kernel = vec![Item(E, 1, 1), Item(T, 0, 1)];
    let c = p.closure(&kernel);

    assert_eq!(c[0], Item(E, 1, 1));
    assert_eq!(c[1], Item(T, 0, 1));
    assert_eq!(2, c.len());

    // I3
    let kernel = vec![Item(T, 1, 1)];
    let c = p.closure(&kernel);

    assert_eq!(c[0], Item(T, 1, 1));
    assert_eq!(1, c.len());

    // I4
    let kernel = vec![Item(F, 0, 1)];
    let c = p.closure(&kernel);

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
    let c = p.closure(&kernel);

    assert_eq!(c[0], Item(F, 1, 1));
    assert_eq!(1, c.len());

    // I6
    let kernel = vec![Item(E, 0, 2)];
    let c = p.closure(&kernel);
    assert_eq!(c[0], Item(E, 0, 2));
    assert_eq!(c[1], Item(T, 0, 0));
    assert_eq!(c[2], Item(T, 1, 0));
    assert_eq!(c[3], Item(F, 0, 0));
    assert_eq!(c[4], Item(F, 1, 0));
    assert_eq!(5, c.len());

    // I7
    let kernel = vec![Item(T, 0, 2)];
    let c = p.closure(&kernel);
    assert_eq!(c[0], Item(T, 0, 2));
    assert_eq!(c[1], Item(F, 0, 0));
    assert_eq!(c[2], Item(F, 1, 0));
    assert_eq!(3, c.len());

    // I8
    let kernel = vec![Item(E, 0, 1), Item(F, 0, 2)];
    let c = p.closure(&kernel);
    assert_eq!(c[0], Item(E, 0, 1));
    assert_eq!(c[1], Item(F, 0, 2));
    assert_eq!(2, c.len());

    // I9
    let kernel = vec![Item(E, 0, 3), Item(T, 0, 1)];
    let c = p.closure(&kernel);
    assert_eq!(c[0], Item(E, 0, 3));
    assert_eq!(c[1], Item(T, 0, 1));
    assert_eq!(2, c.len());

    // I10
    let kernel = vec![Item(T, 0, 3)];
    let c = p.closure(&kernel);
    assert_eq!(c[0], Item(T, 0, 3));
    assert_eq!(1, c.len());

    // I11
    let kernel = vec![Item(F, 0, 3)];
    let c = p.closure(&kernel);
    assert_eq!(c[0], Item(F, 0, 3));
    assert_eq!(1, c.len());
}

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, between {})", self.x, self.y)
    }
}

fn main()
{
    let mut grammar: Vec<Vec<Vec<usize>>> = vec![vec![]; 262];

    grammar[S] = vec![vec![E, ACCEPT, END]];
    grammar[E] = vec![vec![E, ADD, T, END], vec![T, END]];
    grammar[T] = vec![vec![T, MULT, F, END], vec![F, END]];
    grammar[F] = vec![vec![LP, E, RP, END], vec![ID, END]];

    let mut grammar2: Vec<Vec<Vec<usize>>> = vec![vec![]; 264];

    grammar2[S] = vec![vec![E, ACCEPT, END]];
    grammar2[E] = vec![vec![T, E2, END]];
    grammar2[T] = vec![vec![F, T2, END]];
    grammar2[F] = vec![vec![LP, E, RP, END], vec![ID, END]];
    grammar2[E2] = vec![vec![END], vec![ADD, T, E2, END]];
    grammar2[T2] = vec![vec![END], vec![MULT, F, T2, END]];

    let p = Parser {
        lr: grammar,
        rr: grammar2,
    };

    let c = p.items();

    for i in 0..c.len()
    {
        println!("I{}:", i);
        for j in &c[i]
        {
            println!("\t{}", j);
        }
    }

    let f = p.generate_follow();

    for i in S..f.len() - 1
    {
        println!("{:?}", f[i]);
    }
}

fn token(n: usize) -> String
{
    match n {
        40 => "'('".to_string(),
        41 => "')'".to_string(),
        42 => "*".to_string(),
        43 => "+".to_string(),
        105 => "i".to_string(),
        256 => "$".to_string(),
        257 => "S".to_string(),
        258 => "E".to_string(),
        259 => "T".to_string(),
        260 => "F".to_string(),
        261 => "NULL".to_string(),
        _ => n.to_string(),
    }
}

impl fmt::Display for Item {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "Item({}, {}, {})", token(self.0), token(self.1), token(self.2))
    }
}

