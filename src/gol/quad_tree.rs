use memoize::memoize;
use std::collections::LinkedList;
use literal::list;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct QTNode {
    k: usize,
    n: usize,
    a: Box<Option<QTNode>>,
    b: Box<Option<QTNode>>,
    c: Box<Option<QTNode>>,
    d: Box<Option<QTNode>>
}

impl QTNode {
    pub fn new(k: usize, n: usize, a: Option<QTNode>, b: Option<QTNode>, c: Option<QTNode>, d: Option<QTNode>) -> Self {
        Self {
            k, // Level of the node
            n, // Number of cells under this node
            a: Box::new(a),
            b: Box::new(b),
            c: Box::new(c),
            d: Box::new(d)
        }
    }
}

pub fn join(a: &Option<QTNode>, b: &Option<QTNode>, c: &Option<QTNode>, d: &Option<QTNode>) -> QTNode {
    let ar = a.as_ref().unwrap();
    let br = b.as_ref().unwrap();
    let cr = c.as_ref().unwrap();
    let dr = d.as_ref().unwrap();
    
    let an = ar.n;
    let bn = br.n;
    let cn = cr.n;
    let dn = dr.n;
    let n = an + bn + cn + dn;
    // We can later on see whether this performs or not
    QTNode::new(ar.k + 1, n, Some(ar.clone()), Some(br.clone()),Some(cr.clone()), Some(dr.clone()))
}

#[memoize]
pub fn get_zero(k: usize) -> QTNode {
    if k == 0 {
        QTNode::new(0, 0, None, None, None, None)
    } else {
        let z = Some(get_zero(k-1));
        join(&z, &z,&z, &z)
    }
}

// In the worst case, the grid in here is 2x2 so there's no risk of unwrapping a None value
#[memoize]
pub fn centre(m: QTNode) -> QTNode {
    let z = &Some(get_zero(m.k - 1));
    let a = m.a.as_ref();
    let b = m.b.as_ref();
    let c = m.c.as_ref();
    let d = m.d.as_ref();

    let j1 = &Some(join(z, z, z, a));
    let j2 = &Some(join(z, z, b, z));
    let j3 = &Some(join(z, c, z, z));
    let j4 = &Some(join(d, z, z, z));

    join(j1, j2, j3, j4)
}

pub fn life(
    a: Option<&QTNode>,
    b: Option<&QTNode>,
    c: Option<&QTNode>,
    d: Option<&QTNode>,
    e: Option<&QTNode>,
    f: Option<&QTNode>,
    g: Option<&QTNode>,
    h: Option<&QTNode>,
    i: Option<&QTNode>) -> QTNode {
    let neighbors = vec![a, b, c, d, f, g, h, i];
    let mut outer: usize = 0;

    for node in neighbors.iter() {
        outer += node.unwrap().n;
    }

    if (e.unwrap().n == 1 && outer == 2) || outer == 3 {
        QTNode::new(0, 1, None, None, None, None)
    } else {
        QTNode::new(0, 0, None, None, None, None)
    }
}

#[memoize]
pub fn life_4x4(m: QTNode) -> QTNode {
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

    let ad2 = &Some(life(
        aa.as_ref(),
        ab.as_ref(),
        ba.as_ref(),
        ac.as_ref(),
        ad.as_ref(),
        bc.as_ref(),
        ca.as_ref(),
        cb.as_ref(),
        da.as_ref()
    ));
    let bc2 = &Some(life(
        ab.as_ref(),
        ba.as_ref(),
        bb.as_ref(),
        ad.as_ref(),
        bc.as_ref(),
        bd.as_ref(),
        cb.as_ref(),
        da.as_ref(),
        db.as_ref()
    ));
    let cb2 = &Some(life(
        ac.as_ref(),
        ad.as_ref(),
        bc.as_ref(),
        ca.as_ref(),
        cb.as_ref(),
        da.as_ref(),
        cc.as_ref(),
        cd.as_ref(),
        dc.as_ref()
    ));
    let da2 = &Some(life(
        ad.as_ref(),
        bc.as_ref(),
        bd.as_ref(),
        cb.as_ref(),
        da.as_ref(),
        db.as_ref(),
        cd.as_ref(),
        dc.as_ref(),
        dd.as_ref()
    ));
    join(ad2, bc2, cb2, da2)
}

#[memoize]
pub fn next_gen(m: QTNode) -> QTNode {
    if m.n == 0 {
        m.a.unwrap()
    } else if m.k == 2 {
        life_4x4(m)
    } else {
        let a = m.a.unwrap();
        let aa = &Some(a.a.unwrap());
        let ab = &Some(a.b.unwrap());
        let ac = &Some(a.c.unwrap());
        let ad = &Some(a.d.unwrap());

        let b = m.b.unwrap();
        let ba = &Some(b.a.unwrap());
        let bb = &Some(b.b.unwrap());
        let bc = &Some(b.c.unwrap());
        let bd = &Some(b.d.unwrap());

        let c = m.c.unwrap();
        let ca = &Some(c.a.unwrap());
        let cb = &Some(c.b.unwrap());
        let cc = &Some(c.c.unwrap());
        let cd = &Some(c.d.unwrap());

        let d = m.d.unwrap();
        let da = &Some(d.a.unwrap());
        let db = &Some(d.b.unwrap());
        let dc = &Some(d.c.unwrap());
        let dd = &Some(d.d.unwrap());

        let c1 = next_gen(join(aa, ab, ac, ad));
        let c2 = next_gen(join(ab, ba, ad, bc));
        let c3 = next_gen(join(ba, bb,bc, bd));
        let c4 = next_gen(join(ac, ad, ca, cb));
        let c5 = next_gen(join(ad, bc, cb, da));
        let c6 = next_gen(join(bc, bd, da, db));
        let c7 = next_gen(join(ca, cb, cc, cd));
        let c8 = next_gen(join(cb, da, cd, dc));
        let c9 = next_gen(join(da, db, dc, dd));

        let j1 = &Some(join(c1.d.as_ref(), c2.c.as_ref(), c4.b.as_ref(), c5.a.as_ref()));
        let j2 = &Some(join(c2.d.as_ref(), c3.c.as_ref(), c5.b.as_ref(), c6.a.as_ref()));
        let j3 = &Some(join(c4.d.as_ref(), c5.c.as_ref(), c7.b.as_ref(), c8.a.as_ref()));
        let j4 = &Some(join(c5.d.as_ref(), c6.c.as_ref(), c8.b.as_ref(), c9.a.as_ref()));

        join(j1, j2, j3, j4)
    }
}

#[memoize]
pub fn successor(m: QTNode) -> QTNode {
    if m.n == 0 {
        m.a.unwrap()
    } else if m.k == 2 {
        life_4x4(m)
    } else {
        let a = m.a.unwrap();
        let aa = &Some(a.a.unwrap());
        let ab = &Some(a.b.unwrap());
        let ac = &Some(a.c.unwrap());
        let ad = &Some(a.d.unwrap());

        let b = m.b.unwrap();
        let ba = &Some(b.a.unwrap());
        let bb = &Some(b.b.unwrap());
        let bc = &Some(b.c.unwrap());
        let bd = &Some(b.d.unwrap());

        let c = m.c.unwrap();
        let ca = &Some(c.a.unwrap());
        let cb = &Some(c.b.unwrap());
        let cc = &Some(c.c.unwrap());
        let cd = &Some(c.d.unwrap());

        let d = m.d.unwrap();
        let da = &Some(d.a.unwrap());
        let db = &Some(d.b.unwrap());
        let dc = &Some(d.c.unwrap());
        let dd = &Some(d.d.unwrap());

        let c1 = successor(join(aa, ab, ac, ad));
        let c2 = successor(join(ab, ba, ad, bc));
        let c3 = successor(join(ba, bb, bc, bd));
        let c4 = successor(join(ac, ad, ca, cb));
        let c5 = successor(join(ad, bc, cb, da));
        let c6 = successor(join(bc, bd, da, db));
        let c7 = successor(join(ca, cb, cc, cd));
        let c8 = successor(join(cb, da, cd, dc));
        let c9 = successor(join(da, db, dc, dd));

        let j1 = &Some(successor(join(c1.d.as_ref(), c2.c.as_ref(), c4.b.as_ref(), c5.a.as_ref())));
        let j2 = &Some(successor(join(c2.d.as_ref(), c3.c.as_ref(), c5.b.as_ref(), c6.a.as_ref())));
        let j3 = &Some(successor(join(c4.d.as_ref(), c5.c.as_ref(), c7.b.as_ref(), c8.a.as_ref())));
        let j4 = &Some(successor(join(c5.d.as_ref(), c6.c.as_ref(), c8.b.as_ref(), c9.a.as_ref())));

        join(j1, j2, j3, j4)
    }
}

#[memoize]
pub fn expand(m: QTNode, x: usize, y: usize) -> LinkedList<(isize, isize)> {
    if m.n == 0 {
        return list![];
    }

    if m.k == 0 {
        return list![((x >> 0) as isize, (y >> 0) as isize)];
    } else {
        let k = m.k as u32;
        let size: usize = (2 as u32).pow(k) as usize;
        let offset = size >> 1;

        let a = m.a.unwrap();
        let b = m.b.unwrap();
        let c = m.c.unwrap();
        let d = m.d.unwrap();

        let mut points: LinkedList<(isize, isize)> = list![];

        points.append(&mut expand(a, x, y));
        points.append(&mut expand(b, x + offset, y));
        points.append(&mut expand(c, x, y + offset));
        points.append(&mut expand(d, x + offset, y + offset));
        points
    }
}
