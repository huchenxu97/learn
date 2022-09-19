
pub struct  List{
    header: Link,
}
#[derive(Clone)]
enum Link {
    Empty,
    More(Box<Node>)
}
#[derive(Clone)]
struct Node{
    elem: i32,
    next: Link,
}

impl List {
    fn new()->Self{
        // build a list, header points to Link::Empty, 
        List { header: Link::Empty }
    }

    fn push(&mut self, elem: i32){
        // new a Node struct at heap, node.next points to self.header,
        // put node value into Link::More()
        // change points of self.headr to Link Type entity 
        let new_node = Box::new(Node{
            elem: elem,
            next: self.header.clone(),
        });
        self.header = Link::More(new_node);
    }

    fn pop(&mut self) -> Option<i32>{
        //get header, change the mem, change self.header to the previous element
        match std::mem::replace(&mut self.header, Link::Empty){
            Link::Empty => None,
            Link::More(x) => {
                self.header = x.next;
                Some(x.elem)
            },
        }
    }
    
}

#[cfg(test)]
mod test{
    use super::List;
    #[test]
    fn basic_test(){
        let mut  list = List::new();
        list.push(1);
        list.push(2);
        list.pop();
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(),Some(1));
    }
}
