use bevy_ecs::{prelude::{Bundle, Commands, Entity, World}, system::{Command, EntityCommands}};
use bevy_transform::hierarchy::PushChildren;
use bevy_utils::HashMap;
use bevy_ecs::{
    query::{Changed, FilterFetch, With, Without, WorldQuery},
    system::{Query, Res, ResMut},
};

use super::Style;
use super::UINode;

use bevy_window::Windows;
use morphorm::{Cache, layout};
use smallvec::SmallVec;

use self::iterators::{NodeEntity, NodeFirstChild, NodeNextSibling, NodeParent, Tree};

mod iterators;


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


#[derive(Default)]
pub struct LayoutCache {
    // Computed Outputs
    pub rect: HashMap<NodeEntity, Rect>,

    // Intermediate Values
    space: HashMap<NodeEntity, Space>,

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

impl<'a> Cache<'a> for LayoutCache {
    type Item = NodeEntity;

    fn reset(&mut self) {
        
    }

    fn width(&self, node: &Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(node) {
            return rect.width;
        }

        0.0
    }

    fn height(&self, node: &Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(node) {
            return rect.height;
        }

        0.0
    }

    fn posx(&self, node: &Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(node) {
            return rect.posx;
        }

        0.0
    }

    fn posy(&self, node: &Self::Item) -> f32 {
        if let Some(rect) = self.rect.get(node) {
            return rect.posy;
        }

        0.0
    }

    fn left(&self, node: &Self::Item) -> f32 {
        if let Some(space) = self.space.get(node) {
            return space.left;
        }

        0.0
    }

    fn right(&self, node: &Self::Item) -> f32 {
        if let Some(space) = self.space.get(node) {
            return space.right;
        }

        0.0
    }

    fn top(&self, node: &Self::Item) -> f32 {
        if let Some(space) = self.space.get(node) {
            return space.top;
        }

        0.0
    }

    fn bottom(&self, node: &Self::Item) -> f32 {
        if let Some(space) = self.space.get(node) {
            return space.bottom;
        }

        0.0
    }

    fn child_width_max(&self, node: &Self::Item) -> f32 {
        *self.child_width_max.get(node).unwrap()
    }

    /// Get the computed sum of the widths of the child nodes
    fn child_width_sum(&self, node: &Self::Item) -> f32 {
        *self.child_width_sum.get(node).unwrap()
    }

    /// Get the computed maximum width of the child nodes
    fn child_height_max(&self, node: &Self::Item) -> f32 {
        *self.child_height_max.get(node).unwrap()
    }

    /// Get the computed sum of the widths of the child nodes
    fn child_height_sum(&self, node: &Self::Item) -> f32 {
        *self.child_height_sum.get(node).unwrap()
    }

    /// Get the computed maximum grid row
    fn grid_row_max(&self, node: &Self::Item) -> f32 {
        *self.grid_row_max.get(node).unwrap()
    }

    /// Get the computed maximum grid column
    fn grid_col_max(&self, node: &Self::Item) -> f32 {
        *self.grid_col_max.get(node).unwrap()
    }

    // Setters
    fn set_child_width_sum(&mut self, node: &Self::Item, value: f32) {
        *self.child_width_sum.get_mut(node).unwrap() = value;
    }

    fn set_child_height_sum(&mut self, node: &Self::Item, value: f32) {
        *self.child_height_sum.get_mut(node).unwrap() = value;
    }

    fn set_child_width_max(&mut self, node: &Self::Item, value: f32) {
        *self.child_width_max.get_mut(node).unwrap() = value;
    }

    fn set_child_height_max(&mut self, node: &Self::Item, value: f32) {
        *self.child_height_max.get_mut(node).unwrap() = value;
    }

    fn horizontal_free_space(&self, node: &Self::Item) -> f32 {
        *self.horizontal_free_space.get(node).unwrap()
    }
    fn set_horizontal_free_space(&mut self, node: &Self::Item, value: f32) {
        *self.horizontal_free_space.get_mut(node).unwrap() = value;
    }
    fn vertical_free_space(&self, node: &Self::Item) -> f32 {
        *self.vertical_free_space.get(node).unwrap()
    }
    fn set_vertical_free_space(&mut self, node: &Self::Item, value: f32) {
        *self.vertical_free_space.get_mut(node).unwrap() = value;
    }

    fn horizontal_stretch_sum(&self, node: &Self::Item) -> f32 {
        *self.horizontal_stretch_sum.get(node).unwrap()
    }
    fn set_horizontal_stretch_sum(&mut self, node: &Self::Item, value: f32) {
        *self.horizontal_stretch_sum.get_mut(node).unwrap() = value;
    }
    fn vertical_stretch_sum(&self, node: &Self::Item) -> f32 {
        *self.vertical_stretch_sum.get(node).unwrap()
    }
    fn set_vertical_stretch_sum(&mut self, node: &Self::Item, value: f32) {
        *self.vertical_stretch_sum.get_mut(node).unwrap() = value;
    }

    fn set_width(&mut self, node: &Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(node) {
            rect.width = value;
        }
    }
    fn set_height(&mut self, node: &Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(node) {
            rect.height = value;
        }
    }
    fn set_posx(&mut self, node: &Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(node) {
            rect.posx = value;
        }
    }
    fn set_posy(&mut self, node: &Self::Item, value: f32) {
        if let Some(rect) = self.rect.get_mut(node) {
            rect.posy = value;
        }
    }

    fn set_left(&mut self, node: &Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(node) {
            space.left = value;
        }
    }
    fn set_right(&mut self, node: &Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(node) {
            space.right = value;
        }
    }
    fn set_top(&mut self, node: &Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(node) {
            space.top = value;
        }
    }
    fn set_bottom(&mut self, node: &Self::Item, value: f32) {
        if let Some(space) = self.space.get_mut(node) {
            space.bottom = value;
        }
    }

    fn stack_first_child(&self, node: &Self::Item) -> bool {
        *self.stack_first_child.get(node).unwrap()
    }

    fn set_stack_first_child(&mut self, node: &Self::Item, value: bool) {
        *self.stack_first_child.get_mut(node).unwrap() = value;
    }

    fn stack_last_child(&self, node: &Self::Item) -> bool {
        *self.stack_last_child.get(node).unwrap()
    }

    fn set_stack_last_child(&mut self, node: &Self::Item, value: bool) {
        *self.stack_last_child.get_mut(node).unwrap() = value;
    }
}

pub fn layout_node_system(
    mut layout_cache: ResMut<LayoutCache>,
    root_node_query: Query<Entity, (With<UINode>, Without<NodeParent>)>,
    parent_query: Query<&NodeParent>,
    first_child_query: Query<&NodeFirstChild>,
    next_sibling_query: Query<&NodeNextSibling>,
    style_query: Query<(Entity, &Style), (With<UINode>, Changed<Style>)>,
) {



    // Loop over the root UI nodes (nodes without parents)
    for root_node in root_node_query.iter() {
        // Construct a tree for each root node
        let tree = Tree::new(NodeEntity(root_node), &parent_query, &first_child_query, &next_sibling_query);

        // Layout the tree
        layout(layout_cache.as_mut(), &tree, &style_query);


    }
    

}

pub struct PushNodes {
    parent: Entity,
    children: SmallVec<[Entity; 8]>,
}

impl Command for PushNodes {
    fn write(self, world: &mut World) {
        let iter = self.children.iter().enumerate().peekable();
        for (index, child) in iter {
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

pub struct NodeBuilder<'a, 'b> {
    commands: &'b mut Commands<'a>,
    push_children: PushNodes,
}

impl<'a, 'b> NodeBuilder<'a, 'b> {
    pub fn spawn_bundle(&mut self, bundle: impl Bundle) -> EntityCommands<'a, '_> {
        let e = self.commands.spawn_bundle(bundle);
        self.push_children.children.push(e.id());
        e
    }

    pub fn spawn(&mut self) -> EntityCommands<'a, '_> {
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

impl<'a, 'b> TreeBuilder for EntityCommands<'a, 'b> {
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