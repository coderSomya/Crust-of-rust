use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        loop {  
                let mut swapped = false;
                for i in 0..(slice.len() - 1) {
                    if slice[i] > slice[i + 1] {
                        swapped = true;
                        slice.swap(i, i + 1);
                    }
                }
                if swapped == false {
                    break;
                }
        }
    }
}

#[test]
fn bubble_works() {
    let mut things = vec![4, 3, 5, 2, 1];

    super::sort::<_, BubbleSort>(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
