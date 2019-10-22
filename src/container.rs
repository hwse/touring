use crate::Token;
use std::slice::Iter;

#[derive(Debug)]
pub struct ExpandingVec {
    internal_vec: Vec<Token>,
    first_index: i64,
    default: Token
}

impl ExpandingVec {
    pub fn new(vec: Vec<Token>, default: Token) -> ExpandingVec {
        ExpandingVec {
            internal_vec: vec,
            first_index: 0,
            default
        }
    }

    fn to_internal_index(&self, index: i64) -> usize {
        (index - self.first_index) as usize
    }

    pub fn first_index(&self) -> i64 {
        self.first_index
    }

    pub fn last_index(&self) -> i64 {
        self.first_index + self.internal_vec.len() as i64 - 1
    }

    pub fn ensure_available(&mut self, index: i64) {
        // fill up in front if not accessible
        if index < self.first_index {
            while index < self.first_index {
                self.internal_vec.insert(0, self.default);
                self.first_index -= 1;
            }
        } else if index > self.last_index() {
            while index > self.last_index() {
                self.internal_vec.push(self.default);
            }
        }
    }

    pub fn get(&mut self, index: i64) -> &mut char {
        self.ensure_available(index);
        let internal_index = self.to_internal_index(index);
        &mut self.internal_vec[internal_index]
    }

    pub fn iter(&self) -> Iter<'_, char> {
        self.internal_vec.iter()
    }
}


#[test]
fn test_expanding_vec() {
    let mut v = ExpandingVec::new(vec!['0', '1', '2'], '#');
    assert_eq!('0', *v.get(0));
    assert_eq!('1', *v.get(1));
    assert_eq!('2', *v.get(2));
    assert_eq!(0, v.first_index());
    assert_eq!(2, v.last_index());

    assert_eq!('#', *v.get(3));
    assert_eq!(3, v.last_index());

    *v.get(-2) = '-';
    assert_eq!('-', *v.get(-2));
    assert_eq!('#', *v.get(-1));
    assert_eq!(-2, v.first_index())
}
