use std::collections::{HashMap, HashSet};
use std::ops::Index;

/// Represents a mapping of attribute names to their corresponding values.
pub type AttrMap = HashMap<String, String>;

/// Represents the data associated with an HTML element.
pub struct ElementData {
    /// The tag name of the HTML element.
    pub(crate) tag_name: String,
    /// The attributes associated with the HTML element.
    pub(crate) attributes: AttrMap,
}

impl ElementData {
    /// Returns an optional reference to the `String` associated with the "id" attribute.
    ///
    /// # Returns
    ///
    /// * `Some(reference)` - A reference to the `String` containing the "id" attribute value if found.
    /// * `None` - If the "id" attribute is not present.
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    /// Returns a `HashSet` of references to strings representing classes associated with the element.
    ///
    /// # Returns
    ///
    /// * `HashSet<&str>` - A set containing references to strings representing the classes.
    ///                     If the "class" attribute is not present or empty, an empty set is returned.
    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(class_list) => class_list.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

/// Represents the type of a node in the DOM tree.
pub enum NodeType {
    /// Text node containing the text content.
    Text(String),
    /// Element node containing element-specific data.
    Element(ElementData),
    /// Comment node containing comment text.
    Comment(String),
}

/// Represents a node in the DOM tree, containing child nodes and node type information.
pub struct Node {
    /// The list of child nodes contained within this node.
    pub(crate) children: Vec<Node>,
    /// The type of node, either text or element.
    pub(crate) node_type: NodeType,
}

impl Node {
    pub fn pretty_print(&self, indent: usize) {
        match &self.node_type {
            NodeType::Element(element_data) => {
                println!(
                    "{:indent$}<{}>",
                    "",
                    element_data.tag_name,
                    indent = indent * 2
                );
                for child in &self.children {
                    child.pretty_print(&indent + 1);
                }
                println!("{:indent$}</{}>", "", element_data.tag_name, indent = indent * 2);
            }
            NodeType::Text(text) => {
                println!("{:indent$}{}", "", text, indent = indent * 2);
            }
            NodeType::Comment(comment) => {
                println!("{:indent$}<!-- {} -->", "", comment, indent = indent * 2);
            }
        }
    }
}

/// Creates a new text node with the given text data.
///
/// # Arguments
///
/// * `data` - A `String` containing the text data for the node.
///
/// # Returns
///
/// A `Node` representing a text node with the provided text data and no children.
pub fn text(data: String) -> Node {
    Node { children: vec![], node_type: NodeType::Text(data) }
}

/// Creates a new element node with the given tag name, attributes, and children nodes.
///
/// # Arguments
///
/// * `name` - A `String` representing the tag name of the element.
/// * `attrs` - An `AttrMap` containing the attributes associated with the element.
/// * `children` - A `Vec<Node>` containing the child nodes of the element.
///
/// # Returns
///
/// A `Node` representing an element node with the provided tag name, attributes, and children.
pub fn element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}