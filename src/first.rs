

pub struct List {
    head: Link
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {head: Link::Empty}
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: std::mem::replace(&mut self.head, Link::Empty)
        });
        self.head = Link::More(new_node)
    }
    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                None
            },
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut link = List::new();
        assert_eq!(link.pop(), None);
        link.push(1);
        link.push(2);
        link.push(3);
        assert_eq!(link.pop(), Some(3));
        assert_eq!(link.pop(), Some(2));
        assert_eq!(link.pop(), Some(1));
        assert_eq!(link.pop(), None);
    }
}
