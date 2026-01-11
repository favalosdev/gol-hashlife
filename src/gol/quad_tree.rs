use memoize::memoize;
use std::collections::LinkedList;
use literal::list;

type NodeId = usize;

struct Node {
    n: usize,
    k: usize,
    children: Vec<NodeId>
}

impl Node {
    fn new(n: usize, k: usize, a: NodeId, b: NodeId, c: NodeId, d: NodeId) -> Self {
        let mut children = Vec::new();
        children.push(a);
        children.push(b);
        children.push(c);
        children.push(d);
        Node { n, k, children }
    }
}

struct Arena {
    nodes: Vec<Node>
}

impl Arena {
    fn new() -> Self {
        let mut nodes = Vec::new();

        let dead = Node {
            n: 0,
            k: 0,
            children: vec![] 
        };

        let alive = Node {
            n: 1,
            k: 0,
            children: vec![]
        };

        nodes.push(dead);
        nodes.push(alive);
        
        Arena { nodes }
    }

    fn new_node(&mut self, node: Node) -> NodeId {
        let id = self.nodes.len() + 1;
        self.nodes.push(node);
        id
    }

    fn join(&mut self, a: NodeId, b: NodeId, c: NodeId, d: NodeId) -> NodeId {
        let n = self.nodes[a].n + self.nodes[b].n + self.nodes[c].n + self.nodes[d].n;
        let to_add = Node::new(n, self.nodes[a].k + 1, a, b, c, d);
        self.new_node(to_add)
    }

    fn get_zero(&mut self, k: usize) -> NodeId {
        if k == 0 {
            0
        } else {
            let sub = self.get_zero(k-1);
            self.join(sub, sub, sub, sub)
        }
    }
}
