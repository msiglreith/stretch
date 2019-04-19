#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

use std::collections::HashMap;

use crate::geometry::Size;
use crate::number::Number;
use crate::result::{Cache, Layout, Result};
use crate::style::*;

type MeasureFunc = Box<Fn(Size<Number>) -> Result<Size<f32>>>;

pub type NodeId = usize;

pub(crate) type Storage<T> = HashMap<NodeId, T>;

struct Allocator {
    new_id: NodeId,
    free_ids: Vec<NodeId>,
}

impl Allocator {
    pub fn new() -> Self {
        Allocator {
            new_id: 0,
            free_ids: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> NodeId {
        match self.free_ids.pop() {
            Some(id) => id,
            None => {
                let id = self.new_id;
                self.new_id += 1;
                id
            }
        }
    }
}

pub struct Stretch {
    nodes: Allocator,
    pub(crate) style: Storage<Style>,
    pub(crate) parents: Storage<Vec<NodeId>>,
    pub(crate) children: Storage<Vec<NodeId>>,
    pub(crate) measure: Storage<Option<MeasureFunc>>,
    pub(crate) layout: Storage<Layout>,
    pub(crate) layout_cache: Storage<Option<Cache>>,
    pub(crate) is_dirty: Storage<bool>,
}

impl Stretch {
    pub fn new() -> Self {
        Stretch {
            nodes: Allocator::new(),
            style: Storage::new(),
            parents: Storage::new(),
            children: Storage::new(),
            measure: Storage::new(),
            layout: Storage::new(),
            layout_cache: Storage::new(),
            is_dirty: Storage::new(),
        }
    }

    pub fn create_leaf(&mut self, style: Style, measure: Option<MeasureFunc>) -> NodeId {
        let node = self.nodes.allocate();
        self.style.insert(node, style);
        self.parents.insert(node, Vec::with_capacity(1));
        self.children.insert(node, Vec::with_capacity(0));
        self.measure.insert(node, measure);
        self.layout.insert(node, Layout::new());
        self.layout_cache.insert(node, None);
        self.is_dirty.insert(node, true);
        node
    }

    pub fn create_node(&mut self, style: Style, children: Vec<NodeId>) -> NodeId {
        let node = self.nodes.allocate();

        for child in &children {
            self.parents.get_mut(&child).unwrap().push(node);
        }

        self.style.insert(node, style);
        self.parents.insert(node, Vec::with_capacity(1));
        self.children.insert(node, children);
        self.measure.insert(node, None);
        self.layout.insert(node, Layout::new());
        self.layout_cache.insert(node, None);
        self.is_dirty.insert(node, true);

        node
    }

    pub fn add_child(&mut self, node: NodeId, child: NodeId) {
        self.children.get_mut(&node).unwrap().push(child);
        self.parents.get_mut(&node).unwrap().push(node);
        *self.is_dirty.get_mut(&node).unwrap() = true;
    }

    pub fn set_children(&mut self, node: NodeId, children: Vec<&NodeId>) {
        // for child in &self.0.borrow().children {
        //     let position = child
        //         .borrow()
        //         .parents
        //         .iter()
        //         .position(|x| Rc::ptr_eq(&x.upgrade().unwrap(), &self.0))
        //         .unwrap();
        //     child.borrow_mut().parents.remove(position);
        // }

        // self.0.borrow_mut().children = Vec::with_capacity(children.len());

        // for child in children {
        //     child.0.borrow_mut().parents.push(Rc::downgrade(&self.0));
        //     self.0.borrow_mut().children.push(Rc::clone(&child.0));
        // }

        // self.mark_dirty();

        unimplemented!()
    }

    pub fn remove_child(&mut self, child: NodeId) {
        // self.remove_child_at_index({
        //     let parent = self.0.borrow();
        //     parent
        //         .children
        //         .iter()
        //         .position(|x| Rc::ptr_eq(&x, &child.0))
        //         .unwrap()
        // })

        unimplemented!()
    }

    pub fn remove_child_at_index(&mut self, index: usize) {
        // let child = {
        //     let mut parent = self.0.borrow_mut();
        //     let child = parent.children.remove(index);
        //     let position = child
        //         .borrow()
        //         .parents
        //         .iter()
        //         .position(|x| Rc::ptr_eq(&x.upgrade().unwrap(), &self.0))
        //         .unwrap();
        //     child.borrow_mut().parents.remove(position);
        //     child
        // };

        // self.mark_dirty();
        // Node(child)

        unimplemented!()
    }

    pub fn replace_child_at_index(&mut self, index: usize, child: NodeId) {
        // child.0.borrow_mut().parents.push(Rc::downgrade(&self.0));
        // let old_child = std::mem::replace(
        //     &mut self.0.borrow_mut().children[index],
        //     Rc::clone(&child.0),
        // );

        // let position = old_child
        //     .borrow()
        //     .parents
        //     .iter()
        //     .position(|x| Rc::ptr_eq(&x.upgrade().unwrap(), &self.0))
        //     .unwrap();
        // old_child.borrow_mut().parents.remove(position);

        // self.mark_dirty();

        // Node(old_child)

        unimplemented!()
    }

    pub fn children(&self, node: NodeId) -> Vec<NodeId> {
        self.children[&node].clone()
    }

    pub fn set_style(&mut self, node: NodeId, style: Style) {
        if let Some(s) = self.style.get_mut(&node) {
            *s = style;
            *self.is_dirty.get_mut(&node).unwrap() = true;
        }
    }

    pub fn style(&self, node: NodeId) -> Style {
        self.style[&node]
    }

    pub fn layout(&self, node: NodeId) -> Layout {
        self.layout[&node]
    }

    pub fn mark_dirty(&mut self, node: NodeId) {
        fn mark_dirty_impl(
            node: NodeId,
            layout_cache: &mut Storage<Option<Cache>>,
            is_dirty: &mut Storage<bool>,
            parents: &Storage<Vec<NodeId>>,
        ) {
            *layout_cache.get_mut(&node).unwrap() = None;
            *is_dirty.get_mut(&node).unwrap() = true;

            for parent in &parents[&node] {
                mark_dirty_impl(*parent, layout_cache, is_dirty, parents);
            }
        }

        mark_dirty_impl(
            node,
            &mut self.layout_cache,
            &mut self.is_dirty,
            &self.parents,
        );
    }

    pub fn dirty(&self, node: NodeId) -> bool {
        self.is_dirty[&node]
    }

    pub fn compute_layout(&mut self, node: NodeId, size: Size<Number>) -> Result<()> {
        self.compute(node, size)
    }
}
