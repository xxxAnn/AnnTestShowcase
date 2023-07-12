pub fn to_byte(s: impl Into<String>) -> u32 {
    let mut v = s.into().chars().collect::<Vec<char>>();
    v.sort();
    v.dedup();
    v.into_iter().map(|c| (2u32).pow(c as u32 - 97)).sum()
}

fn nlic(a: &Node, b: &Node) -> bool {
    (a.num & b.num) == 0
}


impl Node {
    fn next(&mut self) -> Option<&mut Box<Node>> {
        self.next.as_mut()
    }

    fn set_next(&mut self, u: u32) {
        self.next = Some(Box::new(Node::new(u)));
    }

    fn push(&mut self, u: u32) {
        match self.next() {
            Some(n) => n.push(u),
            None => self.set_next(u)
        }
    }

    fn len(&self) -> usize {
        match &self.next {
            Some(n) => n.len() + 1,
            None => 1
        }
    }
}

impl Head {
    fn next(&mut self) -> Option<&mut Node> {
        self.next.as_mut()
    }

    fn set_next(&mut self, u: u32) {
        self.next = Some(Node::new(u));
    }

    fn push(&mut self, u: u32) {
        self.total += u;
        match self.next() {
            Some(n) => n.push(u),
            None => self.set_next(u)
        }
    }

    fn fits(&self, u: u32) -> bool {
        (self.total & u) == 0
    }

    fn len(&self) -> usize {
        match &self.next {
            Some(n) => n.len(),
            None => 0
        }
    }
}

struct Head {
    total: u32,
    next: Option<Node>
}


struct Node {
    num: u32,
    next: Option<Box<Node>>
}

impl Node {
    fn new(num: u32) -> Self {
        Self { num, next: None }
    }
}

impl Head {
    fn new(num: u32) -> Self {
        let mut s = Self {
            total: 0,
            next: None
        };
        s.push(num);
        s
    }
}

fn main() {
    let mut heads: Vec<Head> = Vec::new();
    let mut i = 0;
    for word in std::fs::read_to_string("words.txt").unwrap().lines() {
        println!("{word}{}", word.chars().count());
        if word.chars().count() == 5 {
            let wrd = to_byte(word);
            for head in heads.iter_mut() {
                if head.fits(wrd) {
                    head.push(wrd);
                }
            }
            heads.push(Head::new(wrd));
            i+=1;
        }
    }

    for head in heads {
        if head.len() == 5 {
            println!("{}", head.total)
        }
    }
}

