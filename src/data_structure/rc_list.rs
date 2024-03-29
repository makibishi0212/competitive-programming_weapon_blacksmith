use cargo_snippet::snippet;

#[snippet("@RcList")]
#[snippet("@Grid2d")]
#[derive(Clone, Debug)]
struct RcListInternal<T> {
    value: T,
    parent: RcList<T>,
}

#[snippet("@RcList")]
#[snippet("@Grid2d")]
#[derive(Clone, Debug)]
pub struct RcList<T> {
    relay: Option<std::rc::Rc<RcListInternal<T>>>,
}

#[snippet("@RcList")]
#[snippet("@Grid2d")]
impl<T: Clone> RcList<T> {
    pub fn new() -> Self {
        RcList { relay: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = RcListInternal {
            value,
            parent: self.clone(),
        };
        self.relay = Some(std::rc::Rc::new(new_node));
    }

    pub fn to_vec(&self) -> Vec<T> {
        match self.relay {
            Some(ref relay) => {
                let mut vec = relay.parent.to_vec();
                vec.push(relay.value.clone());
                vec
            }
            None => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_list() {
        let mut list = RcList::new();
        list.push(1);
        list.push(2);

        assert_eq!(list.to_vec(), vec![1, 2]);

        let mut list = RcList::new();
        list.push('l');
        list.push('r');
        list.push('u');
        list.push('d');
        list.push('d');
        list.push('d');
        assert_eq!(list.to_vec(), vec!['l', 'r', 'u', 'd', 'd', 'd']);
    }
}
