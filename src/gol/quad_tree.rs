use memoize::memoize;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Node {
    k: usize,
    n: usize,
    a: Box<Option<Node>>,
    b: Box<Option<Node>>,
    c: Box<Option<Node>>,
    d: Box<Option<Node>>,
}

impl Node {
    pub fn new(k: usize, n: usize, a: Option<Node>, b: Option<Node>, c: Option<Node>, d: Option<Node>) -> Self {
        Self {
            k,
            n,
            a: Box::new(a),
            b: Box::new(b),
            c: Box::new(c),
            d: Box::new(d)
        }
    }
}

#[memoize]
pub fn join(a: Node, b: Node, c: Node, d: Node) -> Node {
    let n = a.n + b.n + c.n + d.n;
     Node::new(a.k + 1, n, Some(a), Some(b), Some(c), Some(d))
}

#[memoize]
pub fn get_zero(k: usize) -> Node {
    if k == 0 {
        Node::new(0, 0, None, None, None, None)
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

pub fn life(
    a: Option<&Node>,
    b: Option<&Node>,
    c: Option<&Node>,
    d: Option<&Node>,
    e: Option<&Node>,
    f: Option<&Node>,
    g: Option<&Node>,
    h: Option<&Node>,
    i: Option<&Node>) -> Node {
    let neighbors = vec![a, b, c, d, f, g, h, i];
    let mut outer: usize = 0;

    for node in neighbors.iter() {
        outer += node.unwrap().n;
    }

    if (e.unwrap().n == 1 && outer == 2) || outer == 3 {
        Node::new(0, 1, None, None, None, None)
    } else {
        Node::new(0, 0, None, None, None, None)
    }
}

#[memoize]
pub fn life_4x4(m: Node) -> Node {
    let a = m.a.unwrap();
    let aa = a.a.as_ref();
    let ab = a.b.as_ref();
    let ac = a.c.as_ref();
    let ad = a.d.as_ref();

    let b = m.b.unwrap();
    let ba = b.a.as_ref();
    let bb = b.b.as_ref();
    let bc = b.c.as_ref();
    let bd = b.d.as_ref();

    let c = m.c.unwrap();
    let ca = c.a.as_ref();
    let cb = c.b.as_ref();
    let cc = c.c.as_ref();
    let cd = c.d.as_ref();

    let d = m.d.unwrap();
    let da = d.a.as_ref();
    let db = d.b.as_ref();
    let dc = d.c.as_ref();
    let dd = d.d.as_ref();

    let ad2 = life(
        aa.as_ref(),
        ab.as_ref(),
        ba.as_ref(),
        ac.as_ref(),
        ad.as_ref(),
        bc.as_ref(),
        ca.as_ref(),
        cb.as_ref(),
        da.as_ref()
    );
    let bc2 = life(
        ab.as_ref(),
        ba.as_ref(),
        bb.as_ref(),
        ad.as_ref(),
        bc.as_ref(),
        bd.as_ref(),
        cb.as_ref(),
        da.as_ref(),
        db.as_ref()
    );
    let cb2 = life(
        ac.as_ref(),
        ad.as_ref(),
        bc.as_ref(),
        ca.as_ref(),
        cb.as_ref(),
        da.as_ref(),
        cc.as_ref(),
        cd.as_ref(),
        dc.as_ref()
    );
    let da2 = life(
        ad.as_ref(),
        bc.as_ref(),
        bd.as_ref(),
        cb.as_ref(),
        da.as_ref(),
        db.as_ref(),
        cd.as_ref(),
        dc.as_ref(),
        dd.as_ref()
    );
    join(ad2, bc2, cb2, da2)
}