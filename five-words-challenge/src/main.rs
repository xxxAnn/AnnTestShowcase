pub fn to_byte(s: impl Into<String>) -> u32 {
    let mut v = s.into().chars().collect::<Vec<char>>();
    v.sort();
    v.dedup();
    v.into_iter().map(|c| (2u32).pow(c as u32 - 97)).sum()
}

fn nlic(a: &Node, b: &Node) -> bool {
    (a.num & b.num) == 0
}


struct Node {
    num: u32,
    links: Vec<u32>
}

impl Node {
    pub fn new(num: u32) -> Self {
        Self { num, links: Vec::new() }
    }
}

fn main() {
    let mut nodes = Vec::new();
    for word in std::fs::read_to_string("words.txt").unwrap().lines() {
        let mut nnode = Node::new(to_byte(word));
        for onode in nodes.iter_mut() {
            if nlic(onode, &nnode) {
                onode.links.push(nnode.num);
                nnode.links.push(onode.num);
            }
        }
        nodes.push(nnode);
    }
    for node in nodes {
        println!("{}", node.links.len());
        if node.links.len() == 3 {
            println!("{}; {:?}", node.num, &node.links);
        }
    }
}

