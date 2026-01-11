use memoize::memoize;
use std::collections::{LinkedList, HashMap};
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
    evolve_cache: HashMap<NodeId, NodeId>
}

impl Arena {
    fn new() -> Self {
        let mut nodes = Vec::new();

        let void = Node::new(0, 0, VOID, VOID, VOID, VOID);
        let dead = Node::new(0, 1, VOID, VOID, VOID, VOID);
        let alive = Node::new(1, 1, VOID, VOID, VOID, VOID);

        nodes.push(void);
        nodes.push(dead);
        nodes.push(alive);

        Arena { nodes, evolve_cache: HashMap::new() }
    }

    fn new_node(&mut self, node: Node) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }

    fn join(&mut self, a: NodeId, b: NodeId, c: NodeId, d: NodeId) -> NodeId {
        let n = self.nodes[a].n + self.nodes[b].n + self.nodes[c].n + self.nodes[d].n;
        let to_add = Node::new(n, self.nodes[a].k + 1, a, b, c, d);
        self.new_node(to_add)
    }

    fn get_zero(&mut self, k: usize) -> NodeId {
        if k == 1 {
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
        let result = if self.nodes[m].n == 0 {
            // empty
            self.nodes[m].a
        } else if self.nodes[m].k == 2 {
            // base case
            self.life_4x4(m)
        } else {
            // Recursive case
            // We need to get all the node references first to avoid borrow checker issues
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
            
            let c1 = self.next_gen(self.join(aa, ab, ac, ad));
            let c2 = self.next_gen(self.join(ab, ba, ad, bc));
            let c3 = self.next_gen(self.join(ba, bb, bc, bd));
            let c4 = self.next_gen(self.join(ac, ad, ca, cb));
            let c5 = self.next_gen(self.join(ad, bc, cb, da));
            let c6 = self.next_gen(self.join(bc, bd, da, db));
            let c7 = self.next_gen(self.join(ca, cb, cc, cd));
            let c8 = self.next_gen(self.join(cb, da, cd, dc));
            let c9 = self.next_gen(self.join(da, db, dc, dd));
            
            // Get the d, c, b, a fields from each result
            let c1_node = &self.nodes[c1];
            let c2_node = &self.nodes[c2];
            let c3_node = &self.nodes[c3];
            let c4_node = &self.nodes[c4];
            let c5_node = &self.nodes[c5];
            let c6_node = &self.nodes[c6];
            let c7_node = &self.nodes[c7];
            let c8_node = &self.nodes[c8];
            let c9_node = &self.nodes[c9];
            
            let s = self.join(
                self.join(c1_node.d, c2_node.c, c4_node.b, c5_node.a),
                self.join(c2_node.d, c3_node.c, c5_node.b, c6_node.a),
                self.join(c4_node.d, c5_node.c, c7_node.b, c8_node.a),
                self.join(c5_node.d, c6_node.c, c8_node.b, c9_node.a),
            );
            
            s
        };
        result
    } 

    /*
    @lru_cache(maxsize=2**20)
    def next_gen(m):
        """Return the 2**k-1 x 2**k-1 successor, 1 generation forward"""    
        if m.n==0: # empty
            return m.a    
        elif m.k == 2:  # base case               
            s = life_4x4(m)    
        else:
            c1 = next_gen(join(m.a.a, m.a.b, m.a.c, m.a.d))
            c2 = next_gen(join(m.a.b, m.b.a, m.a.d, m.b.c))
            c3 = next_gen(join(m.b.a, m.b.b, m.b.c, m.b.d))
            c4 = next_gen(join(m.a.c, m.a.d, m.c.a, m.c.b))        
            c5 = next_gen(join(m.a.d, m.b.c, m.c.b, m.d.a))
            c6 = next_gen(join(m.b.c, m.b.d, m.d.a, m.d.b))
            c7 = next_gen(join(m.c.a, m.c.b, m.c.c, m.c.d))
            c8 = next_gen(join(m.c.b, m.d.a, m.c.d, m.d.c))
            c9 = next_gen(join(m.d.a, m.d.b, m.d.c, m.d.d))
            
            s = join(
                (join(c1.d, c2.c, c4.b, c5.a)),
                (join(c2.d, c3.c, c5.b, c6.a)),
                (join(c4.d, c5.c, c7.b, c8.a)),
                (join(c5.d, c6.c, c8.b, c9.a)),
            )                    
        return s
    */
}
