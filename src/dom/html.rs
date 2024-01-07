use std::collections::HashMap;
use crate::dom::node::{AttrMap, element, Node, text};

/// A parser for handling HTML document strings.
///
/// This struct keeps track of the current position (`pos`) within the input string (`input`).
/// It provides methods to parse and extract nodes and elements from the HTML document string.
pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    /// Returns the next character in the input string at the current position.
    ///
    /// # Panics
    ///
    /// Panics if called when the current position is at the end of the input string.
    ///
    /// # Returns
    ///
    /// The next character at the current position.
    pub fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Checks if the input string starting from the current position starts with the given prefix.
    ///
    /// # Arguments
    ///
    /// * `prefix` - A reference to the prefix string to check.
    ///
    /// # Returns
    ///
    /// `true` if the input starts with the given prefix at the current position, otherwise `false`.
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.input[self.pos..].starts_with(prefix)
    }

    /// Checks if the current position is at the end of the input string (EOF - End Of File).
    ///
    /// # Returns
    ///
    /// `true` if the current position is at or beyond the end of the input string, otherwise `false`.
    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// Consumes and returns the character at the current position in the input string.
    ///
    /// Advances the position to the next character in the string.
    ///
    /// # Returns
    ///
    /// The character at the current position in the input string.
    /// If the position is at the end of the string, returns a space character.
    pub fn consume_char(&mut self) -> char {
        if let Some((idx, ch)) = self.input[self.pos..].char_indices().next() {
            self.pos += idx + ch.len_utf8();
            ch
        } else {
            ' '
        }
    }

    /// Consumes characters from the input string while the provided predicate function returns true.
    ///
    /// The method iterates through the input string, consuming characters and appending them
    /// to the result string as long as the predicate returns true for the next character.
    ///
    /// # Arguments
    ///
    /// * `predicate` - A function that takes a character and returns a boolean.
    ///                It determines whether to continue consuming characters.
    ///
    /// # Returns
    ///
    /// A `String` containing characters from the input that satisfy the predicate
    /// until a character is encountered for which the predicate returns false
    /// or the end of the input string is reached.
    pub fn consume_while<F>(&mut self, predicate: F) -> String
        where F: Fn(char) -> bool {
        let mut res = String::new();
        while !self.eof() && predicate(self.next_char()) {
            res.push(self.consume_char());
        }
        res
    }

    /// Consumes whitespace characters from the input string.
    ///
    /// Advances the position in the input string until a non-whitespace character is encountered.
    pub fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Parses and returns a tag name from the input string.
    ///
    /// The method consumes characters from the input string while they match
    /// the conditions for a valid tag name (letters 'a' to 'z', 'A' to 'Z', and digits '0' to '9').
    ///
    /// # Returns
    ///
    /// A `String` containing the parsed tag name from the input string.
    pub fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    /// Parses a node in the HTML document based on the next character encountered.
    ///
    /// If the next character is '<', it parses an element node using `parse_element()`.
    /// Otherwise, it parses text content using `parse_text()`.
    ///
    /// # Returns
    ///
    /// A `Node` representing either an element or text, based on the next character in the input.
    pub fn parse_node(&mut self) -> Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text()
        }
    }

    /// Parses multiple nodes in the HTML document until encountering an end tag or reaching the end of the input.
    ///
    /// This method continuously parses nodes using `parse_node()` until it encounters an end tag (`"</"`) or reaches the end of the input.
    ///
    /// # Returns
    ///
    /// A vector containing the parsed nodes. If an end tag is found or the input ends, the method returns all the parsed nodes up to that point.
    pub fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    /// Parses an HTML element node and its children, including attributes and closing tag.
    ///
    /// This method assumes the current position in the input string points to the start of an HTML element ('<').
    /// It proceeds to parse the element's tag name, attributes, content (children nodes), and closing tag.
    ///
    /// # Panics
    ///
    /// This method will panic if the expected characters ('<', '>', '</') are not found or do not match the expected structure of an HTML element.
    ///
    /// # Returns
    ///
    /// A `Node` representing the parsed HTML element, containing its tag name, attributes, and children nodes.
    fn parse_element(&mut self) -> Node {
        assert_eq!(self.consume_char(), '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert_eq!(self.consume_char(), '>');

        // Content
        let children = self.parse_nodes();

        // Closing Tag
        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.parse_tag_name(), tag_name);
        assert_eq!(self.consume_char(), '>');

        element(tag_name, attrs, children)
    }

    /// Parses text content until encountering an HTML tag ('<').
    ///
    /// This method consumes characters from the input string until it encounters an HTML tag ('<').
    /// It constructs a `Node` representing the parsed text content.
    ///
    /// # Returns
    ///
    /// A `Node` representing the parsed text content until the start of an HTML tag.
    fn parse_text(&mut self) -> Node {
        text(self.consume_while(|c| c != '<'))
    }

    /// Parses a single attribute name-value pair within an HTML element tag.
    ///
    /// This method parses an attribute name using `parse_tag_name()`,
    /// followed by the equality sign '=', and then parses the attribute value using `parse_attr_value()`.
    ///
    /// # Panics
    ///
    /// This method will panic if the expected characters ('=') are not found or if the attribute name or value parsing fails.
    ///
    /// # Returns
    ///
    /// A tuple containing the parsed attribute name and its corresponding value as strings.
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert_eq!(self.consume_char(), '=');
        let value = self.parse_attr_value();
        (name, value)
    }

    /// Parses the value of an HTML attribute within an element tag.
    ///
    /// This method expects the attribute value to be enclosed in single (''') or double ('"') quotes.
    /// It parses characters until it encounters the same type of quote that opened the attribute value.
    ///
    /// # Panics
    ///
    /// This method will panic if the opening quote (' or ") is not found or if the closing quote does not match the opening one.
    ///
    /// # Returns
    ///
    /// A string containing the parsed value of the attribute.
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert_eq!(self.consume_char(), open_quote);
        value
    }

    /// Parses all attributes within an HTML element tag.
    ///
    /// This method iterates through the input string, parsing attribute name-value pairs using `parse_attr()`.
    /// It stops when encountering the '>' character, which indicates the end of the attributes.
    ///
    /// # Returns
    ///
    /// A `HashMap` containing the parsed attributes where keys are attribute names and values are attribute values.
    fn parse_attributes(&mut self) -> AttrMap {
        let mut attrs = AttrMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attrs.insert(name, value);
        }
        attrs
    }

    /// Parses an HTML document string and returns the root element of the parsed DOM tree.
    ///
    /// # Arguments
    ///
    /// * `source` - A string containing the HTML document to be parsed.
    ///
    /// # Returns
    ///
    /// A `Node` representing the root element of the parsed DOM tree.
    /// If the document contains a single root element, it is directly returned.
    /// Otherwise, if the document has multiple root nodes, an 'html' element is created to contain all parsed nodes.
    pub fn parse(source: String) -> Node {
        let mut nodes = Parser { pos: 0, input: source }.parse_nodes();

        // If the document contains a root element, just return it. Otherwise, create one.
        if nodes.len() == 1 {
            nodes.swap_remove(0)
        } else {
            element("html".to_string(), HashMap::new(), nodes)
        }
    }
}
