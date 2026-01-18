use std::collections::{HashMap, LinkedList};
use literal::list;

type NodeId = usize;

struct Node {
    n: usize,
    k: usize,
    a: NodeId,
    b: NodeId,
    c: NodeId,
    d: NodeId
}

const VOID: usize = 0;
const DEAD: usize = 1;
const ALIVE: usize = 2;

impl Node {
    fn new(n: usize, k: usize, a: NodeId, b: NodeId, c: NodeId, d: NodeId) -> Self {
        Node { n, k, a, b, c, d }
    }
}

struct Arena {
    nodes: Vec<Node>,
    root: NodeId,
    evolve_cache: HashMap<NodeId, NodeId>
}

impl Arena {
    fn new() -> Self {
        let mut nodes = vec![];

        let void = Node::new(0, 0, VOID, VOID, VOID, VOID);
        let dead = Node::new(0, 0, VOID, VOID, VOID, VOID);
        let alive = Node::new(1, 0, VOID, VOID, VOID, VOID);

        nodes.push(void);
        nodes.push(dead);
        nodes.push(alive);

        Arena { nodes, root: ALIVE, evolve_cache: HashMap::new() }
    }

    fn new_node(&mut self, node: Node) -> NodeId {
        let id = self.nodes.len();

        // Set new root if needed
        if self.nodes[self.root].k < node.k {
            self.root = id;
        }

        self.nodes.push(node);
        id
    }

    fn join(&mut self, a: NodeId, b: NodeId, c: NodeId, d: NodeId) -> NodeId {
        let n = &self.nodes[a].n + &self.nodes[b].n + &self.nodes[c].n + &self.nodes[d].n;
        let to_add = Node::new(n, &self.nodes[a].k + 1, a, b, c, d);
        self.new_node(to_add)
    }

    fn get_zero(&mut self, k: usize) -> NodeId {
        if k == 0 {
            DEAD
        } else {
            let z= self.get_zero(k-1);
            self.join(z, z, z, z)
        }
    }

    fn centre(&mut self, m: NodeId) -> NodeId {
        let m_node = &(self.nodes[m]);
        let (ma, mb, mc, md) = (m_node.a, m_node.b, m_node.c, m_node.d);
        let z = self.get_zero(m_node.k - 1);
        let ja = self.join(z, z, z, ma);
        let jb = self.join(z, z, mb, z);
        let jc = self.join(z, mc, z, z);
        let jd = self.join(md, z, z, z);
        self.join(ja, jb, jc, jd)
    }

    fn life(&self, a: NodeId, b: NodeId, c: NodeId, d: NodeId, e: NodeId, f: NodeId, g: NodeId, h: NodeId, i: NodeId) -> NodeId {
        let mut outer = 0;

        for id in vec![a, b, c, d, f, g, h, i] {
            outer += self.nodes[id].n;
        }

        if self.nodes[e].n == 1 && outer == 2 || outer == 3 {
            ALIVE
        } else {
            DEAD
        }
    }

    fn life_4x4(&mut self, m: NodeId) -> NodeId {
        let m_node = &self.nodes[m];
        let a = &self.nodes[m_node.a];
        let b = &self.nodes[m_node.b];
        let c = &self.nodes[m_node.c];
        let d = &self.nodes[m_node.d];

        let ad = self.life(a.a, a.b, b.a, a.c, a.d, b.c, c.a, c.b, d.a);
        let bc = self.life(a.b, b.a, b.b, a.d, b.c, b.d, c.b, d.a, d.b);
        let cb = self.life(a.c, a.d, b.c, c.a, c.b, d.a, c.c, c.d, d.c);
        let da = self.life(a.d, b.c, b.d, c.b, d.a, d.b, c.d, d.c, d.d);

        self.join(ad, bc, cb, da)
    }

    fn next_gen(&mut self, m: NodeId) -> NodeId {
        if let Some(next) = self.evolve_cache.get(&m) {
            return *next;
        } 

        let next = if self.nodes[m].n == 0 {
            // empty
            self.nodes[m].a
        } else if self.nodes[m].k == 2 {
            // base case
            self.life_4x4(m)
        } else {
            let m_node = &self.nodes[m];
            let (ma, mb, mc, md) = (m_node.a, m_node.b, m_node.c, m_node.d);
            
            let a = &self.nodes[ma];
            let (aa, ab, ac, ad) = (a.a, a.b, a.c, a.d);
            
            let b = &self.nodes[mb];
            let (ba, bb, bc, bd) = (b.a, b.b, b.c, b.d);
            
            let c = &self.nodes[mc];
            let (ca, cb, cc, cd) = (c.a, c.b, c.c, c.d);
            
            let d = &self.nodes[md];
            let (da, db, dc, dd) = (d.a, d.b, d.c, d.d);
            
            let j1 = self.join(aa, ab, ac, ad);
            let j2 = self.join(ab, ba, ad, bc);
            let j3 = self.join(ba, bb, bc, bd);
            let j4 = self.join(ac, ad, ca, cb);
            let j5 = self.join(ad, bc, cb, da);
            let j6 = self.join(bc, bd, da, db);
            let j7 = self.join(ca, cb, cc, cd);
            let j8 = self.join(cb, da, cd, dc);
            let j9 = self.join(da, db, dc, dd);

            let c1 = self.next_gen(j1);
            let c2 = self.next_gen(j2);
            let c3 = self.next_gen(j3);
            let c4 = self.next_gen(j4);
            let c5 = self.next_gen(j5);
            let c6 = self.next_gen(j6);
            let c7 = self.next_gen(j7);
            let c8 = self.next_gen(j8);
            let c9 = self.next_gen(j9);
            
            let s1 = self.join(self.nodes[c1].d, self.nodes[c2].c, self.nodes[c4].b, self.nodes[c5].a);
            let s2 = self.join(self.nodes[c2].d, self.nodes[c3].c, self.nodes[c5].b, self.nodes[c6].a);
            let s3 = self.join(self.nodes[c4].d, self.nodes[c5].c, self.nodes[c7].b, self.nodes[c8].a);
            let s4 = self.join(self.nodes[c5].d, self.nodes[c6].c, self.nodes[c8].b, self.nodes[c9].a);

            let s = self.join(s1, s2, s3, s4);
            s 
        };

        self.evolve_cache.insert(m, next);
        next
    }

    // Convert QuadTree to (x,y) world coordinate system.
    fn to_world(&self) -> LinkedList<(isize, isize)> {
        list![]
    }

    // Convert world coordinates to QuadTree. Doesn't return something.
    fn from_world(&self, cells: LinkedList<(isize, isize)>) {
    }
}
