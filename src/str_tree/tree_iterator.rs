use crate::str_tree::StaticWord;
use crate::str_tree::StrTree;

enum WalkerState {
    Node,
    Children,
}

pub(crate) struct Walker<'a> {
    node: &'a StrTree,
    child: Option<Box<Walker<'a>>>,
    state: WalkerState,
    child_idx: usize,
}

impl<'a> Walker<'a> {
    pub(crate) fn new(node: &'a StrTree) -> Self {
        let initial_state = if node.self_is_word() && node.data().is_some() {
            WalkerState::Node
        } else {
            WalkerState::Children
        };

        Self {
            node,
            child: None,
            child_idx: 0,
            state: initial_state,
        }
    }

    #[cfg(test)]
    pub(crate) fn as_words(self) -> impl Iterator<Item = String> + 'a {
        self.map(|mut static_word| static_word.into_word().iter().rev().collect())
    }

    fn next_child(&mut self) -> Option<()> {
        self.state = WalkerState::Children;
        self.child = Some(Box::new(Walker::new(self.node.child(self.child_idx)?)));
        self.child_idx += 1;
        Some(())
    }
}

impl<'a> Iterator for Walker<'a> {
    type Item = StaticWord;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            WalkerState::Node => {
                self.next_child();
                let mut ret = StaticWord::default();
                ret.push(self.node.data().unwrap());
                return Some(ret);
            }

            WalkerState::Children => match &mut self.child {
                Some(ref mut walker) => {
                    if let Some(mut ret) = walker.next() {
                        self.node.data().map(|c| ret.push(c));
                        Some(ret)
                    } else {
                        self.child = None;
                        self.next()
                    }
                }
                None => {
                    self.next_child()?;
                    self.next()
                }
            },
        }
    }
}
