use std::{any, borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::anyhow;

#[derive(Debug, Default)]
pub struct CritNode {
    value: Option<String>,
    children: HashMap<String, Rc<RefCell<CritNode>>>,
}

#[derive(Debug, Default)]
pub struct CritBit {
    root: Rc<RefCell<CritNode>>,
}

impl CritBit {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&self, str: String) -> anyhow::Result<()> {
        let mut node = Rc::clone(&self.root);

        for ch in str.chars() {
            let ch_str = ch.to_string();
            if !node.borrow_mut().children.contains_key(&ch_str) {
                node.borrow_mut()
                    .children
                    .insert(ch_str.clone(), Rc::new(RefCell::new(CritNode::default())));
            }
            let temp_node: Rc<RefCell<CritNode>> = Rc::clone(&node);
            node = Rc::clone(temp_node.as_ref().borrow().children.get(&ch_str).unwrap());
        }
        node.borrow_mut().value = Some(str);

        Ok(())
    }

    pub fn search(&self, value: &str) -> anyhow::Result<bool> {
        let mut node = Rc::clone(&self.root);

        for ch in value.chars() {
            if node
                .as_ref()
                .borrow()
                .children
                .contains_key(&ch.to_string())
            {
                let temp_node = Rc::clone(
                    node.as_ref()
                        .borrow()
                        .children
                        .get(&ch.to_string())
                        .ok_or(anyhow!("err"))?,
                );
                node = temp_node;
            } else {
                return Ok(false);
            }
        }

        if node.as_ref().borrow().value.is_some() {
            return Ok(true);
        }

        return Ok(false);
    }

    // pub fn display(&self) -> anyhow::Result<()> {
    //     let mut node = Rc::clone(&self.root);

    //     // print!(">>{}", node.as_ref().borrow().value.as_ref().unwrap());
    //     let mut temp_node: Rc<RefCell<CritNode>>;
    //     while node.as_ref().borrow().value.is_some() {
    //         print!(">>{}", node.as_ref().borrow().value.as_ref().unwrap());
    //         for (k, v) in node.as_ref().borrow().children.iter() {
    //             print!(">>{}", node.as_ref().borrow().value.as_ref().unwrap());
    //             temp_node = Rc::clone(v);
    //         }
    //         node = Rc::clone(&temp_node);
    //     }
    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use super::CritBit;

    #[test]
    fn test_display() -> anyhow::Result<()> {
        let ct = CritBit::new();
        let s = String::from("this");
        ct.insert(s.clone())?;
        let res = ct.search(&s)?;
        println!("Response: {res}");
        // ct.display()?;
        Ok(())
    }
}
