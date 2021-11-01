use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

  struct Node<T> {
    val: T,
    suivant: Option<Box<Node<T>>>,
}
  
  impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
    Self{
            head: None
        }
    }

pub fn is_empty(&self) -> bool {
        if self.len()> 0{
            false
        } else {
            true
        }
    }




  
pub fn len(&self) -> usize {
        let mut count = 0;
        while self.head{::Some {
            count=count +1;
        }}
        count
    }

   pub fn pop(&mut self) -> Option<T> {
      let mut elem = self.entete.take();
    }

    pub fn peek(&self) -> Option<&T> {
        let elem = self.self.entete.as_ref();
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
       let mut liste = self
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        unimplemented!()
    }
}
