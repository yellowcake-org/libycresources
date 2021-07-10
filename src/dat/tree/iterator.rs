use super::super::Directory;

pub struct TreeIterator<'a> {
    children: &'a [Directory],
    parent: Option<Box<Self>>,
    depth: usize,
}

impl Directory {
    pub fn iter(&self) -> TreeIterator<'_> {
        TreeIterator {
            children: std::slice::from_ref(self),
            parent: None,
            depth: 0,
        }
    }
}

impl<'a> Iterator for TreeIterator<'a> {
    type Item = (usize, &'a Directory);

    fn next(&mut self) -> Option<Self::Item> {
        match self.children.get(0) {
            None => match self.parent.take() {
                Some(parent) => {
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
            Some(node) => {
                self.children = &self.children[1..];

                let depth = self.depth;
                *self = Self {
                    children: node.children.as_slice(),
                    parent: Some(Box::new(std::mem::take(self))),
                    depth: depth + 1,
                };

                Some((self.depth - 1, node))
            }
        }
    }
}

impl Default for TreeIterator<'_> {
    fn default() -> Self {
        Self {
            children: &[],
            parent: None,
            depth: 0,
        }
    }
}
