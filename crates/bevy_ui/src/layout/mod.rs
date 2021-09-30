use std::iter::Rev;

use bevy_ecs::{prelude::*, system::{Command, EntityCommands}};
use bevy_math::Vec2;
use bevy_transform::components::{Parent, Transform};
use bevy_utils::HashMap;
use bevy_window::Windows;
use smallvec::SmallVec;
use super::{Node, Style};

use morphorm::{Cache, Hierarchy, LayoutType, PositionType, Units};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeEntity(pub Entity);

impl NodeEntity {
    pub fn entity(&self) -> Entity {
        self.0
    }
}

impl<'world> morphorm::Node<'world> for NodeEntity {
    type Data = Query<'world, 'world, &'static Style>;

    fn layout_type(&self, query: &Self::Data) -> Option<LayoutType> {
        query.get(self.entity()).map_or(None, |style| Some(style.layout_type))
    }

    fn position_type(&self, query: &Self::Data) -> Option<PositionType> {
        query.get(self.entity()).map_or(None, |style| Some(style.position_type))
    }

    fn width(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.width))
    }

    fn height(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.height))
    }

    fn left(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.left))
    }
    fn right(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.right))
    }
    fn top(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.top))
    }
    fn bottom(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.bottom))
    }

    fn child_left(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.child_left))
    }

    fn child_right(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.child_right))
    }

    fn child_top(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.child_top))
    }

    fn child_bottom(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.child_bottom))
    }

    fn min_left(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.min_left))
    }

    fn min_right(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.min_right))
    }

    fn min_top(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.min_top))
    }

    fn min_bottom(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.min_bottom))
    }

    fn max_left(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.max_left))
    }

    fn max_right(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.max_right))
    }

    fn max_top(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.max_top))
    }

    fn max_bottom(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.max_bottom))
    }

    fn min_width(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.min_width))
    }

    fn max_width(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.max_width))
    }

    fn min_height(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.min_height))
    }

    fn max_height(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.max_height))
    }

    fn row_between(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.row_between))
    }

    fn col_between(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.col_between))
    }

    fn grid_rows(&self, query: &Self::Data) -> Option<Vec<Units>> {
        query.get(self.entity()).map_or(None, |style| Some(style.grid_rows.clone()))
    }

    fn grid_cols(&self, query: &Self::Data) -> Option<Vec<Units>> {
        query.get(self.entity()).map_or(None, |style| Some(style.grid_cols.clone()))
    }

    fn row_index(&self, query: &Self::Data) -> Option<usize> {
        query.get(self.entity()).map_or(None, |style| Some(style.row_index))
    }

    fn col_index(&self, query: &Self::Data) -> Option<usize> {
        query.get(self.entity()).map_or(None, |style| Some(style.col_index))
    }

    fn row_span(&self, query: &Self::Data) -> Option<usize> {
        query.get(self.entity()).map_or(None, |style| Some(style.row_span))
    }

    fn col_span(&self, query: &Self::Data) -> Option<usize> {
        query.get(self.entity()).map_or(None, |style| Some(style.col_span))
    }

    fn border_left(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.border))
    }

    fn border_right(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.border))
    }

    fn border_top(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.border))
    }

    fn border_bottom(&self, query: &Self::Data) -> Option<Units> {
        query.get(self.entity()).map_or(None, |style| Some(style.border))
    }
}

pub struct NodeParent(pub NodeEntity);
pub struct NodeFirstChild(pub NodeEntity);
pub struct NodeNextSibling(pub NodeEntity);


pub struct Tree<'borrow, 'world, 'state> {
    root: NodeEntity,
    parent_query: &'borrow Query<'world, 'state, &'static NodeParent>,
    first_child_query: &'borrow Query<'world, 'state, &'static NodeFirstChild>,
    next_sibling_query: &'borrow Query<'world, 'state, &'static NodeNextSibling>,
}

impl<'borrow,'world,'state> Tree<'borrow,'world,'state>
{
    pub fn new(root: NodeEntity, parent_query: &'borrow Query<'world, 'state, &'static NodeParent>, first_child_query: &'borrow Query<'world, 'state, &'static NodeFirstChild>, next_sibling_query: &'borrow Query<'world, 'state, &'static NodeNextSibling>) -> Self {
        Self {
            root,
            parent_query,
            first_child_query,
            next_sibling_query,
        }
    }
}

impl<'borrow,'world,'state> Tree<'borrow,'world,'state>
{
    pub fn flatten(&self) -> Vec<NodeEntity> {

        let iterator = DownwardIterator {
            parent_query: &self.parent_query,
            first_child_query: &self.first_child_query,
            next_sibling_query: &self.next_sibling_query,
            current_node: Some(self.root),
        };

        iterator.collect::<Vec<_>>()
    }

    pub fn get_first_child(&self, node: NodeEntity) -> Option<NodeEntity> {

        self.first_child_query.get(node.entity()).map_or(None, |first_child| Some(first_child.0))
    }

    pub fn get_next_sibling(&self, node: NodeEntity) -> Option<NodeEntity> {
        self.next_sibling_query.get(node.entity()).map_or(None, |next_sibling| Some(next_sibling.0))
    }
}

impl<'borrow,'world,'state> morphorm::Hierarchy<'borrow> for Tree<'borrow,'world,'state> 
{
    type Item = NodeEntity;
    type DownIter = std::vec::IntoIter<NodeEntity>;
    type UpIter = Rev<std::vec::IntoIter<NodeEntity>>;
    type ChildIter = ChildIterator<'borrow, 'world, 'state>;

    fn up_iter(&self) -> Self::UpIter {
        self.flatten().into_iter().rev()
    }

    fn down_iter(&self) -> Self::DownIter {
        self.flatten().into_iter()
    }

    fn parent(&self, node: Self::Item) -> Option<Self::Item> {
        self.parent_query.get(node.entity()).map_or(None, |parent| Some(parent.0))
    }

    fn child_iter(&'borrow self, node: Self::Item) -> Self::ChildIter {
        ChildIterator {
            next_sibling_query: &self.next_sibling_query,
            current_node: self.get_first_child(node),
        }
    }

    fn is_first_child(&self, node: Self::Item) -> bool {
        if let Some(parent) = self.parent(node) {
            if let Some(first_child) = self.get_first_child(node) {
                if first_child == node {
                    return true;
                }
            }
        }

        false
    }

    fn is_last_child(&self, node: Self::Item) -> bool {
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

pub struct DownwardIterator<'borrow,'world,'state> {
    parent_query: &'borrow Query<'world, 'state, &'static NodeParent>,
    first_child_query: &'borrow Query<'world, 'state, &'static NodeFirstChild>,
    next_sibling_query: &'borrow Query<'world, 'state, &'static NodeNextSibling>,
    current_node: Option<NodeEntity>,
}

impl<'borrow,'world,'state> Iterator for DownwardIterator<'borrow,'world,'state> {
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

pub struct ChildIterator<'borrow, 'world,'state> {
    pub next_sibling_query: &'borrow Query<'world, 'state, &'static NodeNextSibling>,
    pub current_node: Option<NodeEntity>,
}

impl<'borrow, 'world, 'state> Iterator for ChildIterator<'borrow, 'world, 'state> {
    type Item = NodeEntity;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entity) = self.current_node {
            //self.current_node = self.tree.next_sibling[entity.index()].as_ref();
            self.current_node = self.next_sibling_query.get(entity.entity()).map_or(None, |next_sibling| Some(next_sibling.0));
            return Some(entity);
        }

        None
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Rect {
    pub posx: f32,
    pub posy: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Space {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}


#[derive(Default)]
pub struct LayoutCache {
    // Computed Outputs
    pub rect: HashMap<NodeEntity, Rect>,

    // Intermediate Values
    space: HashMap<NodeEntity, Space>,
    size: HashMap<NodeEntity, Size>,

    child_width_max: HashMap<NodeEntity, f32>,
    child_height_max: HashMap<NodeEntity, f32>,
    child_width_sum: HashMap<NodeEntity, f32>,
    child_height_sum: HashMap<NodeEntity, f32>,

    grid_row_max: HashMap<NodeEntity, f32>,
    grid_col_max: HashMap<NodeEntity, f32>,

    horizontal_free_space: HashMap<NodeEntity, f32>,
    horizontal_stretch_sum: HashMap<NodeEntity, f32>,

    vertical_free_space: HashMap<NodeEntity, f32>,
    vertical_stretch_sum: HashMap<NodeEntity, f32>,

    stack_first_child: HashMap<NodeEntity, bool>,
    stack_last_child: HashMap<NodeEntity, bool>,

}

impl LayoutCache {
    pub fn add(&mut self, entity: NodeEntity) {
        self.rect.insert(entity, Default::default());

        self.space.insert(entity, Default::default());
        self.size.insert(entity, Default::default());

        self.child_width_max.insert(entity, Default::default());
        self.child_height_max.insert(entity, Default::default());
        self.child_width_sum.insert(entity, Default::default());
        self.child_height_sum.insert(entity, Default::default());

        self.grid_row_max.insert(entity, Default::default());
        self.grid_col_max.insert(entity, Default::default());

        self.horizontal_free_space
            .insert(entity, Default::default());
        self.horizontal_stretch_sum
            .insert(entity, Default::default());

        self.vertical_free_space.insert(entity, Default::default());
        self.vertical_stretch_sum.insert(entity, Default::default());

        self.stack_first_child.insert(entity, Default::default());
        self.stack_last_child.insert(entity, Default::default());
    }
}

impl morphorm::Cache for LayoutCache {
    type Item = NodeEntity;


    fn visible(&self, node: Self::Item) -> bool {
        true
    }

    fn set_visible(&mut self, node: Self::Item, value: bool) {
        // TODO
    }

    fn geometry_changed(&self, node: Self::Item) -> morphorm::GeometryChanged {
        morphorm::GeometryChanged::empty()
    }

    fn set_geo_changed(&mut self, node: Self::Item, flag: morphorm::GeometryChanged, value: bool) {
        // TODO
    }

    fn new_width(&self, node: Self::Item) -> f32 {
        if let Some(size) = self.size.get(&node) {
            return size.width;
        }

        0.0
    }

    fn new_height(&self, node: Self::Item) -> f32 {
        if let Some(size) = self.size.get(&node) {
            return size.height;
        }

        0.0
    }

    fn set_new_width(&mut self, node: Self::Item, value: f32) {
        if let Some(size) = self.size.get_mut(&node) {
            size.width = value;
        }
    }

    fn set_new_height(&mut self, node: Self::Item, value: f32) {
        if let Some(size) = self.size.get_mut(&node) {
            size.height = value;
        }
    }

    fn width(&self, node: Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.width;
        }

        0.0
    }

    fn height(&self, node: Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.height;
        }

        0.0
    }

    fn posx(&self, node: Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.posx;
        }

        0.0
    }

    fn posy(&self, node: Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(&node) {
            return rect.posy;
        }

        0.0
    }

    fn left(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.left;
        }

        0.0
    }

    fn right(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.right;
        }

        0.0
    }

    fn top(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.top;
        }

        0.0
    }

    fn bottom(&self, node: Self::Item) -> f32 {
        if let Some(space) = self.space.get(&node) {
            return space.bottom;
        }

        0.0
    }

    fn child_width_max(&self, node: Self::Item) -> f32 {
        *self.child_width_max.get(&node).unwrap()
    }

    /// Get the computed sum of the widths of the child nodes
    fn child_width_sum(&self, node: Self::Item) -> f32 {
        *self.child_width_sum.get(&node).unwrap()
    }

    /// Get the computed maximum width of the child nodes
    fn child_height_max(&self, node: Self::Item) -> f32 {
        *self.child_height_max.get(&node).unwrap()
    }

    /// Get the computed sum of the widths of the child nodes
    fn child_height_sum(&self, node: Self::Item) -> f32 {
        *self.child_height_sum.get(&node).unwrap()
    }

    /// Get the computed maximum grid row
    fn grid_row_max(&self, node: Self::Item) -> f32 {
        *self.grid_row_max.get(&node).unwrap()
    }

    /// Get the computed maximum grid column
    fn grid_col_max(&self, node: Self::Item) -> f32 {
        *self.grid_col_max.get(&node).unwrap()
    }

    // Setters
    fn set_child_width_sum(&mut self, node: Self::Item, value: f32) {
        if let Some(child_width_sum) = self.child_width_sum.get_mut(&node) {
            *child_width_sum = value;
        } else {
            self.child_width_sum.insert(node, value);
        }
        //*self.child_width_sum.get_mut(node).unwrap() = value;
    }

    fn set_child_height_sum(&mut self, node: Self::Item, value: f32) {
        //*self.child_height_sum.get_mut(node).unwrap() = value;
        if let Some(child_height_sum) = self.child_height_sum.get_mut(&node) {
            *child_height_sum = value;
        } else {
            self.child_height_sum.insert(node, value);
        }
    }

    fn set_child_width_max(&mut self, node: Self::Item, value: f32) {
        //*self.child_width_max.get_mut(node).unwrap() = value;
        if let Some(child_width_max) = self.child_width_max.get_mut(&node) {
            *child_width_max = value;
        } else {
            self.child_width_max.insert(node, value);
        }
    }

    fn set_child_height_max(&mut self, node: Self::Item, value: f32) {
        //*self.child_height_max.get_mut(node).unwrap() = value;
        if let Some(child_height_max) = self.child_height_max.get_mut(&node) {
            *child_height_max = value;
        } else {
            self.child_height_max.insert(node, value);
        }
    }

    fn horizontal_free_space(&self, node: Self::Item) -> f32 {
        *self.horizontal_free_space.get(&node).unwrap()
    }
    fn set_horizontal_free_space(&mut self, node: Self::Item, value: f32) {
        *self.horizontal_free_space.get_mut(&node).unwrap() = value;
    }
    fn vertical_free_space(&self, node: Self::Item) -> f32 {
        *self.vertical_free_space.get(&node).unwrap()
    }
    fn set_vertical_free_space(&mut self, node: Self::Item, value: f32) {
        *self.vertical_free_space.get_mut(&node).unwrap() = value;
    }

    fn horizontal_stretch_sum(&self, node: Self::Item) -> f32 {
        *self.horizontal_stretch_sum.get(&node).unwrap()
    }
    fn set_horizontal_stretch_sum(&mut self, node: Self::Item, value: f32) {
        *self.horizontal_stretch_sum.get_mut(&node).unwrap() = value;
    }
    fn vertical_stretch_sum(&self, node: Self::Item) -> f32 {
        *self.vertical_stretch_sum.get(&node).unwrap()
    }
    fn set_vertical_stretch_sum(&mut self, node: Self::Item, value: f32) {
        *self.vertical_stretch_sum.get_mut(&node).unwrap() = value;
    }

    fn set_width(&mut self, node: Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.width = value;
        }
    }
    fn set_height(&mut self, node: Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.height = value;
        }
    }
    fn set_posx(&mut self, node: Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.posx = value;
        }
    }
    fn set_posy(&mut self, node: Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(&node) {
            rect.posy = value;
        }
    }

    fn set_left(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.left = value;
        }
    }
    fn set_right(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.right = value;
        }
    }
    fn set_top(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.top = value;
        }
    }
    fn set_bottom(&mut self, node: Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(&node) {
            space.bottom = value;
        }
    }

    fn stack_first_child(&self, node: Self::Item) -> bool {
        *self.stack_first_child.get(&node).unwrap()
    }

    fn set_stack_first_child(&mut self, node: Self::Item, value: bool) {
        *self.stack_first_child.get_mut(&node).unwrap() = value;
    }

    fn stack_last_child(&self, node: Self::Item) -> bool {
        *self.stack_last_child.get(&node).unwrap()
    }

    fn set_stack_last_child(&mut self, node: Self::Item, value: bool) {
        *self.stack_last_child.get_mut(&node).unwrap() = value;
    }
}



pub struct PushNodes {
    parent: Entity,
    children: SmallVec<[Entity; 8]>,
}

impl Command for PushNodes {
    fn write(self, world: &mut World) {
        let mut iter = self.children.iter().enumerate().peekable();

        while let Some((index, child)) = iter.next() {
            world
                .entity_mut(*child)
                .insert(NodeParent(NodeEntity(self.parent)));
            if let Some((_,next_sibling)) = iter.peek() {
                world
                    .entity_mut(*child)
                    .insert(NodeNextSibling(NodeEntity(**next_sibling)));
            }
            if index == 0 {
                world.entity_mut(self.parent).insert(NodeFirstChild(NodeEntity(*child)));
            }            
        }
    }
}

pub struct NodeBuilder<'a, 'b, 'state> {
    commands: &'b mut Commands<'a, 'state>,
    push_children: PushNodes,
}

impl<'a, 'b, 'state> NodeBuilder<'a, 'b, 'state> {
    pub fn spawn_bundle(&mut self, bundle: impl Bundle) -> EntityCommands<'a, 'state, '_> {
        let e = self.commands.spawn_bundle(bundle);
        self.push_children.children.push(e.id());
        e
    }

    pub fn spawn(&mut self) -> EntityCommands<'a, 'state, '_> {
        let e = self.commands.spawn();
        self.push_children.children.push(e.id());
        e
    }

    pub fn parent_entity(&self) -> Entity {
        self.push_children.parent
    }

    pub fn add_command<C: Command + 'static>(&mut self, command: C) -> &mut Self {
        self.commands.add(command);
        self
    }
}


pub trait TreeBuilder {
    fn with_node_children(&mut self, f: impl FnOnce(&mut NodeBuilder)) -> &mut Self;
}

impl<'a, 'b, 'state> TreeBuilder for EntityCommands<'a, 'state, 'b> {
    fn with_node_children(&mut self, spawn_children: impl FnOnce(&mut NodeBuilder)) -> &mut Self {
        let parent = self.id();
        let push_children = {
            let mut builder = NodeBuilder {
                commands: self.commands(),
                push_children: PushNodes {
                    children: SmallVec::default(),
                    parent,
                },
            };
            spawn_children(&mut builder);
            builder.push_children
        };

        self.commands().add(push_children);
        self
    }
}

pub fn root_node_system(
    windows: Res<Windows>,
    root_node_query: Query<Entity, (With<Node>, Without<NodeParent>)>,
    mut style_query: Query<&mut Style>,
) {

    let window = windows.get_primary().unwrap();

    let window_width = window.physical_width() as f32;        
    let window_height = window.physical_height() as f32;

    for root_node in root_node_query.iter() {
        if let Ok(mut style) = style_query.get_mut(root_node) {
            style.width = Units::Pixels(window_width);
            style.height = Units::Pixels(window_height);
        }
    }
}

pub fn layout_node_system(
    windows: Res<Windows>,
    mut layout_cache: ResMut<LayoutCache>,
    root_node_query: Query<Entity, (With<Node>, Without<NodeParent>)>,
    parent_query: Query<&'static NodeParent>,
    first_child_query: Query<&'static NodeFirstChild>,
    next_sibling_query: Query<&'static NodeNextSibling>,
    style_query: Query<&'static Style>,
    mut node_transform_query: Query<(Entity, &mut Node, &mut Transform, Option<&Parent>)>,
) {

    for root_node in root_node_query.iter() {

        let tree = Tree::new(NodeEntity(root_node), &parent_query, &first_child_query, &next_sibling_query);


        for node in tree.down_iter() {
            layout_cache.add(node);
        }

        let window = windows.get_primary().unwrap();       
        let window_height = window.physical_height() as f32;

        morphorm::layout(layout_cache.as_mut(), &tree, &style_query);


    
        for node in tree.down_iter() {
            if let Ok((entity, mut uinode, mut transform, parent)) = node_transform_query.get_mut(node.entity()) {
                let posx = layout_cache.posx(node);
                let posy = layout_cache.posy(node);
                let width = layout_cache.width(node);
                let height = layout_cache.height(node);

                //println!("{:?} {} {} {} {}", node.entity(), posx, posy, width, height);

                uinode.size = Vec2::new(width, height);
                let position = &mut transform.translation;
                position.x = posx + width/2.0;
                position.y = window_height as f32 - (posy + height/2.0);

                
                
            }
        }
    }
}