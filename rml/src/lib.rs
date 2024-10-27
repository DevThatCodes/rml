#[derive(Debug, PartialEq)]
pub struct RmlTag {
    pub name: String,
    pub value: String
}
    
#[derive(Debug, PartialEq)]
pub struct RmlElement {
    pub name: String,
    pub tags: Vec<RmlTag>,
    pub children: Vec<Box<RmlElement>>
}

// implement default for RmlElement
impl Default for RmlElement {
    fn default() -> Self {
        RmlElement {
            name: String::from("DefaultElement"),
            tags: Vec::new(),
            children: Vec::new()
        } 
    }
}

// implement default for RmlTag
impl Default for RmlTag {
    fn default() -> Self {
        RmlTag { name: String::from("DefaultTag"), value: String::from("DefaultValue") }
    }
}

// implement functions for RmlElement
impl RmlElement {
    // returns a reference to the children field
    pub fn get_children(&self) -> Vec<&RmlElement> {
        let mut output : Vec<&RmlElement> = Vec::new();
        self.children.iter().for_each(|child| output.push(child));
        output
    }
    // returns name as a string
    pub fn name(&self) -> String {
        String::from(&self.name)
    }
    // returns a reference to the tags field
    pub fn get_tags(&self) -> &Vec<RmlTag> {
        &self.tags
    }

    pub fn add_tag(&mut self, tag: RmlTag) {
        self.tags.push(tag);
    }

    // adds a child to the RmlElement
    pub fn add_child(&mut self, element: RmlElement) {
        let _ = &self.children.push(Box::new(element));
    }
}

// implement functions for RmlTag
impl RmlTag {
    // returns name as a string
    pub fn name(&self) -> String {
        String::from(&self.name)
    }
    // returns value as a string
    pub fn get(&self) -> String {
        String::from(&self.value)   
    }

    // sets the value of the tag
    pub fn set(&mut self, value: String) {
        self.value = value;
    }
}

// TODO: 2, 3.
// DONE: 1.
// 1. Figure out the algorithm to do it.
// 2. Pseudocode it.
// 3. Translate pseudocode to rust code.
//
// How it will work:
// finds a start tag and adds that to the scope and adds the key to the child hashmap,
// puts all content in the child hashmap with the key as the most recently added tag to the scope,
// then once the end tag of the most recently added tag to the scope is found, the children and
// content is added to the element and either added to the key in the child hashmap with the next
// tag in the scope, or returns the element.
//
// Pseudocode:
// parse_from_string(string data) {
//  
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_element_creation() {
        assert_eq!(RmlElement::default(), RmlElement {
            name: "DefaultElement".to_string(),
            children: Vec::new(),
            tags: Vec::new()
        })
    }

    #[test]
    fn test_default_tag_creation() {
        assert_eq!(RmlTag::default(), RmlTag {
            name: "DefaultTag".to_string(),
            value: "DefaultValue".to_string()
        })
    }

    #[test]
    fn test_add_tag() {
        let mut default_element_added_tag = RmlElement::default();
        default_element_added_tag.add_tag(RmlTag::default());
        let element_with_tag = RmlElement {
            name: "DefaultElement".to_string(),
            children: Vec::new(),
            tags: vec![RmlTag::default()]
        };
        assert_eq!(default_element_added_tag, element_with_tag);
    }
}
