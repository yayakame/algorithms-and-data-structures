#[derive(Debug, Clone)]
pub struct SinglyLinkedNode<T>
{
    pub data: T,
    pub next: Option<Box<SinglyLinkedNode<T>>>,
}

impl<'a, T> SinglyLinkedNode<T>
{
    pub fn new(data: T) -> SinglyLinkedNode<T>
    {
        SinglyLinkedNode { data, next: None }
    }

    pub fn new_with_next(data: T, next: SinglyLinkedNode<T>) -> SinglyLinkedNode<T>
    {
        SinglyLinkedNode {
            data,
            next: Some(Box::new(next)),
        }
    }

    pub fn get_next_data(&self) -> Option<&T>
    {
        self.next.as_ref().map(|x| &x.data)
    }

    pub fn get_next_data_mut(&mut self) -> Option<&mut T>
    {
        self.next.as_mut().map(|x| &mut x.data)
    }

    pub fn set_next_with_data(&mut self, data: T)
    {
        self.next = Some(Box::new(SinglyLinkedNode::new(data)));
    }

    pub fn delete_cur(&mut self)
    {
        self.next.take().map(|node| {
            *self = *node;
        });
    }

    pub fn insert(&mut self, other: SinglyLinkedNode<T>)
    {
        let old_next = self.next.take();
        self.next = Some(Box::new(other));
        self.next.as_mut().unwrap().next = old_next;
    }

    pub fn iter(&'a self) -> SinglyLinkedNodeIterator<'a, T>
    {
        SinglyLinkedNodeIterator { next: Some(self) }
    }

    pub fn iter_mut(&'a mut self) -> SinglyLinkedNodeMutIterator<'a, T>
    {
        SinglyLinkedNodeMutIterator { next: Some(self) }
    }
}

#[derive(Debug)]
pub struct SinglyLinkedNodeIterator<'a, T>
{
    next: Option<&'a SinglyLinkedNode<T>>,
}

impl<'a, T> Iterator for SinglyLinkedNodeIterator<'a, T>
{
    type Item = &'a SinglyLinkedNode<T>;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            node
        })
    }
}

pub struct SinglyLinkedNodeMutIterator<'a, T: 'a>
{
    next: Option<&'a mut SinglyLinkedNode<T>>,
}

impl<'a, T> Iterator for SinglyLinkedNodeMutIterator<'a, T>
{
    type Item = &'a mut SinglyLinkedNode<T>;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.next.take().map(|node| {
            let ret = unsafe { std::ptr::read(&node) };
            self.next = node.next.as_deref_mut();
            ret
        })
    }
}

pub struct SinglyLinkedNodeOwnedIterator<T>
{
    next: Option<SinglyLinkedNode<T>>,
}

impl<T> Iterator for SinglyLinkedNodeOwnedIterator<T>
{
    type Item = SinglyLinkedNode<T>;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.next.take().map(|mut node| {
            self.next = node.next.take().map(|n| *n);
            node
        })
    }
}

impl<T> IntoIterator for SinglyLinkedNode<T>
{
    type Item = SinglyLinkedNode<T>;
    type IntoIter = SinglyLinkedNodeOwnedIterator<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        SinglyLinkedNodeOwnedIterator {
            next: self.next.map(|n| *n),
        }
    }
}
