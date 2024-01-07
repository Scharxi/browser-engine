use crate::dom::node::{AttrMap, ElementData, Node, NodeType};

pub mod dom;

fn main() {
    let mut attributes = AttrMap::new();
    attributes.insert("id".to_string(), "main".to_string());
    attributes.insert("class".to_string(), "container".to_string());

    let root = Node {
        children: vec![
            Node {
                children: vec![
                    Node {
                        children: vec![],
                        node_type: NodeType::Text("Some text content".to_string()),
                    },
                    Node {
                        children: vec![],
                        node_type: NodeType::Comment("A comment".to_string()),
                    },
                ],
                node_type: NodeType::Element(ElementData {
                    tag_name: "div".to_string(),
                    attributes: AttrMap::new(),
                }),
            },
            Node {
                children: vec![],
                node_type: NodeType::Element(ElementData {
                    tag_name: "p".to_string(),
                    attributes,
                }),
            },
        ],
        node_type: NodeType::Element(ElementData {
            tag_name: "body".to_string(),
            attributes: AttrMap::new(),
        }),
    };

    root.pretty_print(0);
}
