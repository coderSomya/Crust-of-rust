use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len(){

            //slice[unsorted...] is not sorted

            let smart = false;

            if!smart{
                let mut i = unsorted;

                while i>0 && slice[i-1]>slice[i]{
                    slice.swap(i-1,i);
                    i-=1;
                }
            }else{
                //use binary search to find the index
                //then use .insert to splice i

                let i = match slice[..unsorted].binary_search(&slice[unsorted]){
                    Ok(i) => i,
                    Err(i) => i,
                };

                slice[i..=unsorted].rotate_right(  1);
            }
        }
    }
}

#[test]
fn insertion_works() {
    let mut things = vec![4, 3, 5, 2, 1];

    super::sort::<_, InsertionSort>(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
