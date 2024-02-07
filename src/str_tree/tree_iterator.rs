use crate::str_tree::StaticWord;
use crate::str_tree::StrTree;

pub(crate) struct Walker<'a> {
    node: &'a StrTree,
    child: Option<Box<dyn Iterator<Item = StaticWord> + 'a>>,
    child_idx: usize,
}

impl<'a> Walker<'a> {
    pub(crate) fn new(node: &'a StrTree) -> impl Iterator<Item = StaticWord> + '_ {
        let mut walker = Self {
            node,
            child: None,
            child_idx: 0,
        };
        walker.next_child();

        node.data()
            .into_iter()
            .filter(|_| node.self_is_word())
            .map(|c| {
                let mut ret = StaticWord::default();
                ret.push(c);
                ret
            })
            .chain(walker)
    }

    fn next_child(&mut self) -> Option<()> {
        self.child = Some(Box::new(Walker::new(self.node.child(self.child_idx)?)));
        self.child_idx += 1;
        Some(())
    }
}

impl<'a> Iterator for Walker<'a> {
    type Item = StaticWord;
    fn next(&mut self) -> Option<Self::Item> {
        let walker = self.child.as_mut()?;
        if let Some(mut ret) = walker.next() {
            self.node.data().map(|c| ret.push(c));
            Some(ret)
        } else {
            self.next_child()?;
            self.next()
        }
    }
}
