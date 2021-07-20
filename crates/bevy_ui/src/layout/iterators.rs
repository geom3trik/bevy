
use std::iter::Rev;

use morphorm::*;

use bevy_ecs::{entity::{Entity}, prelude::{Changed, Query, With}};

use crate::{Style, UINode};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeEntity(pub Entity);

impl NodeEntity {
    fn entity(&self) -> Entity {
        self.0
    }
}

impl<'a> Node<'a> for NodeEntity {
    type Data = Query<'a, (Entity, &'a Style), (With<UINode>, Changed<Style>)>;

    fn is_visible(&self, query: &Self::Data) -> bool {
        true
    }

    fn layout_type(&self, query: &Self::Data) -> Option<LayoutType> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.layout_type))
    }

    fn position_type(&self, query: &Self::Data) -> Option<PositionType> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.position_type))
    }

    fn width(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.width))
    }

    fn height(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.height))
    }

    fn left(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.left))
    }
    fn right(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.right))
    }
    fn top(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.top))
    }
    fn bottom(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.bottom))
    }

    fn child_left(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.child_left))
    }

    fn child_right(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.child_right))
    }

    fn child_top(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.child_top))
    }

    fn child_bottom(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.child_bottom))
    }

    fn min_left(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.min_left))
    }

    fn min_right(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.min_right))
    }

    fn min_top(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.min_top))
    }

    fn min_bottom(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.min_bottom))
    }

    fn max_left(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.max_left))
    }

    fn max_right(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.max_right))
    }

    fn max_top(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.max_top))
    }

    fn max_bottom(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.max_bottom))
    }

    fn min_width(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.min_width))
    }

    fn max_width(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.max_width))
    }

    fn min_height(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.min_height))
    }

    fn max_height(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.max_height))
    }

    fn row_between(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.row_between))
    }

    fn col_between(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.col_between))
    }

    fn grid_rows(&self, query: &Self::Data) -> Option<Vec<Units>> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.grid_rows))
    }

    fn grid_cols(&self, query: &Self::Data) -> Option<Vec<Units>> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.grid_cols))
    }

    fn row_index(&self, query: &Self::Data) -> Option<usize> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.row_index))
    }

    fn col_index(&self, query: &Self::Data) -> Option<usize> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.col_index))
    }

    fn row_span(&self, query: &Self::Data) -> Option<usize> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.row_span))
    }

    fn col_span(&self, query: &Self::Data) -> Option<usize> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.col_span))
    }

    fn border_left(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.border))
    }

    fn border_right(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.border))
    }

    fn border_top(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.border))
    }

    fn border_bottom(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |(_, style)| Some(style.border))
    }


}

pub struct NodeParent(pub NodeEntity);
pub struct NodeFirstChild(pub NodeEntity);
pub struct NodeNextSibling(pub NodeEntity);


pub struct Tree<'a> {
    root: NodeEntity,
    parent_query: &'a Query<'a, &'a NodeParent>,
    first_child_query: &'a Query<'a, &'a NodeFirstChild>,
    next_sibling_query: &'a Query<'a, &'a NodeNextSibling>,
}

impl<'a> Tree<'a> {
    pub fn new(root: NodeEntity, parent_query: &'a Query<'a, &'a NodeParent>, first_child_query: &'a Query<'a, &'a NodeFirstChild>, next_sibling_query: &'a Query<'a, &'a NodeNextSibling>) -> Self {
        Self {
            root,
            parent_query,
            first_child_query,
            next_sibling_query,
        }
    }
}

impl<'a> Tree<'a> {
    pub fn flatten(&self) -> Vec<NodeEntity> {

        let iterator = DownwardIterator {
            parent_query: self.parent_query,
            first_child_query: self.first_child_query,
            next_sibling_query: self.next_sibling_query,
            current_node: Some(self.root),
        };

        iterator.collect::<Vec<_>>()
    }

    pub fn get_first_child(&self, node: &NodeEntity) -> Option<&'a NodeEntity> {

        self.first_child_query.get(node.entity()).map_or(None, |first_child| Some(&first_child.0))
    }

    pub fn get_next_sibling(&self, node: &NodeEntity) -> Option<&'a NodeEntity> {
        self.next_sibling_query.get(node.entity()).map_or(None, |next_sibling| Some(&next_sibling.0))
    }
}

impl<'a> Hierarchy<'a> for Tree<'a> {
    type Item = NodeEntity;
    type DownIter = std::vec::IntoIter<NodeEntity>;
    type UpIter = Rev<std::vec::IntoIter<NodeEntity>>;
    type ChildIter = std::slice::Iter<'a, NodeEntity>;

    fn up_iter(&self) -> Self::UpIter {
        self.flatten().into_iter().rev()
    }

    fn down_iter(&self) -> Self::DownIter {
        self.flatten().into_iter()
    }

    fn parent(&self, node: &Self::Item) -> Option<&Self::Item> {
        self.parent_query.get(node.entity()).map_or(None, |parent| Some(&parent.0))
    }

    fn child_iter(&'a self, node: &Self::Item) -> Self::ChildIter {
        todo!()
    }

    fn is_first_child(&self, node: &Self::Item) -> bool {
        if let Some(parent) = self.parent(node) {
            if let Some(first_child) = self.get_first_child(node) {
                if first_child == node {
                    return true;
                }
            }
        }

        false
    }

    fn is_last_child(&self, node: &Self::Item) -> bool {
        if let Some(parent) = self.parent(node) {
            if let Some(mut temp) = self.get_first_child(parent) {
                while let Some(next_sibling) = self.get_next_sibling(temp) {
                    temp = next_sibling;
                }

                if temp == node {
                    return true;
                }
            }
        }

        false
    }

}

// fn construct_tree(tree: &mut Vec<Entity>, query: &mut query, entity: Entity) {
//     if let Some(mut children) = query.get_mut::<Children>(entity) {
//         for e in std::mem::take(&mut children.0) {
//             construct_tree(tree, query, e);
//         }
//     }
// }

pub struct DownwardIterator<'a> {
    parent_query: &'a Query<'a, &'a NodeParent>,
    first_child_query: &'a Query<'a, &'a NodeFirstChild>,
    next_sibling_query: &'a Query<'a, &'a NodeNextSibling>,
    current_node: Option<NodeEntity>,
}

impl<'a> Iterator for DownwardIterator<'a> {
    type Item = NodeEntity;
    fn next(&mut self) -> Option<NodeEntity> {

        let r = self.current_node;

        if let Some(current) = self.current_node {

            if let Ok(first_child) = self.first_child_query.get(current.entity()) {
                self.current_node = Some(first_child.0);
            } else {
                let mut temp = Some(current);
                while temp.is_some() {
                    if let Ok(next_sibling) = self.next_sibling_query.get(temp.unwrap().entity()) {
                        self.current_node = Some(next_sibling.0);
                        return r;
                    } else {
                        temp = self.parent_query.get(temp.unwrap().entity()).map_or(None, |parent| Some(parent.0));
                    }
                }

                self.current_node = None;
            }
        }

        return r;
    }
}


fn morphorm_system(

) {
    // Construct the visual tree

    // 
}

