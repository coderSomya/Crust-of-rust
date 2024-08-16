trait Sorter{
    fn sort<T>(slice: &mut [T]) where T: Ord;
}

fn sort<T, S> (slice: &mut [T]) where T: Ord, S: Sorter{
    S::sort(slice)
}

mod bubblesort;
mod insertionsort;


#[cfg(test)]
mod tests{

    use super::*;

    struct StdSorter;
    impl Sorter for StdSorter{
        fn sort<T>(slice: &mut [T]) where T: Ord{
            slice.sort();
        }   
    }

    #[test]
    fn std_works(){
        let mut things = vec![4,2,1,3];
        sort::<_, StdSorter>(&mut things);
        assert_eq!(things, vec![1,2,3,4]);
    }
}