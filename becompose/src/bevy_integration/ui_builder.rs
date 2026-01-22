//! UI Builder
//!
//! Provides a fluent API for building Bevy UI using BECOMPOSE patterns.

use bevy::prelude::*;
use std::sync::Arc;

use crate::components::*;
use crate::layout::*;
use crate::modifier::Modifiers;

/// Extension trait for Commands to spawn BECOMPOSE UI
pub trait BecomposeCommands {
    /// Spawn a UI root for BECOMPOSE
    fn spawn_ui_root(&mut self) -> UiBuilder;
}

impl<'w, 's> BecomposeCommands for Commands<'w, 's> {
    fn spawn_ui_root(&mut self) -> UiBuilder {
        UiBuilder::new()
    }
}

/// Builder for constructing UI hierarchies
pub struct UiBuilder {
    children: Vec<UiElement>,
}

impl UiBuilder {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        // Create root node
        let root = commands
            .spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },))
            .id();

        // Build children
        for child in self.children {
            let child_entity = child.build(commands);
            commands.entity(root).add_child(child_entity);
        }

        root
    }
}

impl Default for UiBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a UI element that can be built
pub enum UiElement {
    Text(TextElement),
    Button(ButtonElement),
    Column(ColumnElement),
    Row(RowElement),
    Box(BoxElement),
    Spacer(SpacerElement),
}

impl UiElement {
    pub fn build(self, commands: &mut Commands) -> Entity {
        match self {
            UiElement::Text(e) => e.build(commands),
            UiElement::Button(e) => e.build(commands),
            UiElement::Column(e) => e.build(commands),
            UiElement::Row(e) => e.build(commands),
            UiElement::Box(e) => e.build(commands),
            UiElement::Spacer(e) => e.build(commands),
        }
    }
}

/// Text element builder
pub struct TextElement {
    pub text: String,
    pub style: TextStyle,
    pub modifier: Modifiers,
}

impl TextElement {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: TextStyle::default(),
            modifier: Modifiers::default(),
        }
    }

    pub fn with_style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_modifier(mut self, modifier: Modifiers) -> Self {
        self.modifier = modifier;
        self
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        let mut node_style = Node::default();
        self.modifier.apply_to_node(&mut node_style);

        commands
            .spawn((
                Text::new(self.text),
                TextFont {
                    font_size: self.style.font_size,
                    ..default()
                },
                TextColor(self.style.color),
                TextNode {
                    config: TextConfig::new(""),
                },
            ))
            .id()
    }
}

/// Button element builder
pub struct ButtonElement {
    pub on_click: Arc<dyn Fn() + Send + Sync>,
    pub modifier: Modifiers,
    pub enabled: bool,
    pub children: Vec<UiElement>,
}

impl ButtonElement {
    pub fn new<F: Fn() + Send + Sync + 'static>(on_click: F) -> Self {
        Self {
            on_click: Arc::new(on_click),
            modifier: Modifiers::default(),
            enabled: true,
            children: Vec::new(),
        }
    }

    pub fn with_modifier(mut self, modifier: Modifiers) -> Self {
        self.modifier = modifier;
        self
    }

    pub fn with_child(mut self, child: UiElement) -> Self {
        self.children.push(child);
        self
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        let node_style = Node {
            padding: UiRect::all(Val::Px(12.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let mut bg = BackgroundColor(Color::srgb(0.25, 0.25, 0.3));
        self.modifier.apply_to_background(&mut bg);

        let button = commands
            .spawn((
                Button,
                node_style,
                bg,
                BorderRadius::all(Val::Px(4.0)),
                Clickable {
                    on_click: self.on_click,
                },
            ))
            .id();

        for child in self.children {
            let child_entity = child.build(commands);
            commands.entity(button).add_child(child_entity);
        }

        button
    }
}

/// Column element builder
pub struct ColumnElement {
    pub layout: ColumnLayout,
    pub modifier: Modifiers,
    pub children: Vec<UiElement>,
}

impl ColumnElement {
    pub fn new() -> Self {
        Self {
            layout: ColumnLayout::default(),
            modifier: Modifiers::default(),
            children: Vec::new(),
        }
    }

    pub fn with_arrangement(mut self, arrangement: VerticalArrangement) -> Self {
        self.layout = self.layout.with_arrangement(arrangement);
        self
    }

    pub fn with_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.layout = self.layout.with_alignment(alignment);
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.layout = self.layout.with_spacing(spacing);
        self
    }

    pub fn with_modifier(mut self, modifier: Modifiers) -> Self {
        self.modifier = modifier;
        self
    }

    pub fn with_child(mut self, child: UiElement) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_children(mut self, children: Vec<UiElement>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        let mut node_style = Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: self.layout.vertical_arrangement.to_justify_content(),
            align_items: self.layout.horizontal_alignment.to_align_items(),
            row_gap: Val::Px(self.layout.spacing),
            ..Default::default()
        };

        // Apply modifiers
        self.modifier.apply_to_node(&mut node_style);

        let mut bg = BackgroundColor(Color::NONE);
        self.modifier.apply_to_background(&mut bg);

        let column = commands
            .spawn((
                node_style,
                bg,
                ColumnNode {
                    layout: self.layout,
                },
            ))
            .id();

        for child in self.children {
            let child_entity = child.build(commands);
            commands.entity(column).add_child(child_entity);
        }

        column
    }
}

impl Default for ColumnElement {
    fn default() -> Self {
        Self::new()
    }
}

/// Row element builder
pub struct RowElement {
    pub layout: RowLayout,
    pub modifier: Modifiers,
    pub children: Vec<UiElement>,
}

impl RowElement {
    pub fn new() -> Self {
        Self {
            layout: RowLayout::default(),
            modifier: Modifiers::default(),
            children: Vec::new(),
        }
    }

    pub fn with_arrangement(mut self, arrangement: HorizontalArrangement) -> Self {
        self.layout = self.layout.with_arrangement(arrangement);
        self
    }

    pub fn with_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.layout = self.layout.with_alignment(alignment);
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.layout = self.layout.with_spacing(spacing);
        self
    }

    pub fn with_modifier(mut self, modifier: Modifiers) -> Self {
        self.modifier = modifier;
        self
    }

    pub fn with_child(mut self, child: UiElement) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_children(mut self, children: Vec<UiElement>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        let mut node_style = Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: self.layout.horizontal_arrangement.to_justify_content(),
            align_items: self.layout.vertical_alignment.to_align_items(),
            column_gap: Val::Px(self.layout.spacing),
            ..Default::default()
        };

        // Apply modifiers
        self.modifier.apply_to_node(&mut node_style);

        let mut bg = BackgroundColor(Color::NONE);
        self.modifier.apply_to_background(&mut bg);

        let row = commands
            .spawn((
                node_style,
                bg,
                RowNode {
                    layout: self.layout,
                },
            ))
            .id();

        for child in self.children {
            let child_entity = child.build(commands);
            commands.entity(row).add_child(child_entity);
        }

        row
    }
}

impl Default for RowElement {
    fn default() -> Self {
        Self::new()
    }
}

/// Box element builder
pub struct BoxElement {
    pub layout: BoxLayout,
    pub modifier: Modifiers,
    pub children: Vec<UiElement>,
}

impl BoxElement {
    pub fn new() -> Self {
        Self {
            layout: BoxLayout::default(),
            modifier: Modifiers::default(),
            children: Vec::new(),
        }
    }

    pub fn with_alignment(mut self, alignment: Alignment2D) -> Self {
        self.layout = self.layout.with_alignment(alignment);
        self
    }

    pub fn with_modifier(mut self, modifier: Modifiers) -> Self {
        self.modifier = modifier;
        self
    }

    pub fn with_child(mut self, child: UiElement) -> Self {
        self.children.push(child);
        self
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        let mut node_style = Node::default();
        self.layout.apply_to_node(&mut node_style);

        let mut bg = BackgroundColor(Color::NONE);
        self.modifier.apply_to_background(&mut bg);

        let box_node = commands
            .spawn((
                node_style,
                bg,
                BoxNode {
                    layout: self.layout,
                },
            ))
            .id();

        for child in self.children {
            let child_entity = child.build(commands);
            commands.entity(box_node).add_child(child_entity);
        }

        box_node
    }
}

impl Default for BoxElement {
    fn default() -> Self {
        Self::new()
    }
}

/// Spacer element builder
pub struct SpacerElement {
    pub modifier: Modifiers,
}

impl SpacerElement {
    pub fn new() -> Self {
        Self {
            modifier: Modifiers::default(),
        }
    }

    pub fn with_modifier(mut self, modifier: Modifiers) -> Self {
        self.modifier = modifier;
        self
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        let mut node_style = Node {
            flex_grow: 1.0,
            ..default()
        };

        // Apply modifiers
        self.modifier.apply_to_node(&mut node_style);

        commands.spawn((node_style, SpacerNode)).id()
    }
}

impl Default for SpacerElement {
    fn default() -> Self {
        Self::new()
    }
}

// Convenience functions for creating elements

/// Create a text element
pub fn text(content: impl Into<String>) -> UiElement {
    UiElement::Text(TextElement::new(content))
}

/// Create a text element with style
pub fn text_styled(content: impl Into<String>, style: TextStyle) -> UiElement {
    UiElement::Text(TextElement::new(content).with_style(style))
}

/// Create a button element
pub fn button<F: Fn() + Send + Sync + 'static>(on_click: F, content: UiElement) -> UiElement {
    UiElement::Button(ButtonElement::new(on_click).with_child(content))
}

/// Create a column element
pub fn column(children: Vec<UiElement>) -> UiElement {
    UiElement::Column(ColumnElement::new().with_children(children))
}

/// Create a row element
pub fn row(children: Vec<UiElement>) -> UiElement {
    UiElement::Row(RowElement::new().with_children(children))
}

/// Create a spacer element
pub fn spacer() -> UiElement {
    UiElement::Spacer(SpacerElement::new())
}

/// Create a sized spacer
pub fn spacer_sized(width: f32, height: f32) -> UiElement {
    UiElement::Spacer(SpacerElement::new().with_modifier(Modifiers::new().size(width, height)))
}
