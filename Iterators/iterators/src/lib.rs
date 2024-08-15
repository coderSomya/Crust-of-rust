fn main(){
    let mut iter = vec!["a", "b", "c"].into_iter();

    while let Some(e)= iter.next(){
        print!("{}",e);
    }

    let v = vec![1,2,3];

    for x in v {
        // consumes v, owned v
    }

    for x in v.iter(){
        // borrows v, reference to v
    }
}


pub fn flatten(iter: I) -> Flatten<I>{
    Flatten::new(iter)
}

pub struct Flatten<O> {
    outer: O,
}

impl<O> Flatten<O> {
    fn new(iter: O) -> Self{
        Flatten {outer : iter}
    }
}

impl<O> Iterator for Flatten<O> 
where O: Iterator,
O::Item: IntoIterator
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item>{
        None
    }
}