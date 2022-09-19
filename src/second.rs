pub struct IntoIter<T>(List<T>);

pub struct List<T>{
    header: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T>{
    item: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new()->Self{
        List { header: None }
    }

    fn push(&mut self, item: T){
        self.header = Some(Box::new(Node{
            item: item,
            next: self.header.take(),
        }))
    }

    fn pop(&mut self)-> Option<T>{
        self.header.take().map(|node| {
            self.header = node.next;
            node.item
        })
    }

    fn peek(&self)->Option<&T>{
        self.header.as_ref().map(|node| &node.item)
    }

    fn peek_mut(&mut self)->Option<&mut T>{
        self.header.as_mut().map(|node| &mut node.item)
    }

    fn into_iter(self)->IntoIter<T>{
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
#[cfg(test)]
mod test{
    use super::List;

    #[test]
    fn test(){
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        list.push(1);
        list.push(2);
        list.pop();
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek_mut(), Some(&mut 1));
        list.peek_mut().map(|v| *v = 30);
        assert_eq!(list.peek(), Some(&30));
        list.push(3);
        list.push(4);
        let mut  iter = list.into_iter();
        assert_eq!(iter.next(), Some(4));
    }
}