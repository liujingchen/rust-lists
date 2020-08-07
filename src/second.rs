

pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {head: None}
    }
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take()
        });
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
                self.head = node.next;
                node.elem
            }
        )
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: &'a Link<T>
}
impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter{next: &self.head}
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next {
            self.next = &node.next;
            Some(&node.elem)
        } else {
            None
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

        link.push(4);
        link.push(5);
        link.push(6);
        assert_eq!(link.pop(), Some(6));
        assert_eq!(link.pop(), Some(5));
        assert_eq!(link.pop(), Some(4));
        assert_eq!(link.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| {
            *value = 42;
        });
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.peek_mut(), Some(&mut 42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
