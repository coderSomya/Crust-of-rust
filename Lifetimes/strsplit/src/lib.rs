// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit <'haystack, 'delimiter>{
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter>{
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self
    {
        Self{
            remainder: Some(haystack),
            delimiter
        }
    }
}

impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder{
            if let Some(next_delim) = remainder.find(self.delimiter){
                let until_delimiter = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delimiter)
            }else{
                self.remainder.take()
            }
        }
        else{
            None
        }
    }
}

fn until_char(s: &str, c: char) -> &str{
    StrSplit::new(s,&format!("{}",c))
    .next()
    .expect("strplit always returns somthing")
}

#[test]
fn it_works(){
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn tail(){
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}

#[test]
fn until_char_test(){
    assert_eq!(until_char("hello world", 'o'), "hell");
}