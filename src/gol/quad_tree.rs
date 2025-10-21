use memoize::memoize;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Node {
    k: usize,
    a: Box<Option<Node>>,
    b: Box<Option<Node>>,
    c: Box<Option<Node>>,
    d: Box<Option<Node>>,
    n: usize
}

impl Node {
    pub fn new(k: usize, a: Option<Node>, b: Option<Node>, c: Option<Node>, d: Option<Node>, n: usize) -> Self {
        Self {
            k,
            a: Box::new(a),
            b: Box::new(b),
            c: Box::new(c),
            d: Box::new(d),
            n
        }
    }
}

#[memoize]
pub fn join(a: Node, b: Node, c: Node, d: Node) -> Node {
    let n = a.n + b.n + c.n + d.n;
    return Node::new(a.k + 1, Some(a), Some(b), Some(c), Some(d), n);
}

#[memoize]
pub fn get_zero(k: usize) -> Node {
    if k == 0 {
        // Ugly as fuck but what else could be done?
        Node::new(0, None, None, None, None, 0)
    } else {
        join(get_zero(k-1), get_zero(k-1), get_zero(k-1), get_zero(k-1))
    }
}

// In the worst case, the grid in here is 2x2 so there's no risk of unwrapping a None value
#[memoize]
pub fn centre(m: Node) -> Node {
    let z = get_zero(m.k - 1);
    join(
        join(z.clone(), z.clone(), z.clone(), m.a.unwrap()),
        join(z.clone(), z.clone(), m.b.unwrap(), z.clone()),
        join(z.clone(), m.c.unwrap(), z.clone(), z.clone()),
        join(m.d.unwrap(), z.clone(), z.clone(), z.clone())
    )
}
