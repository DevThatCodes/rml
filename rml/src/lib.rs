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

// implement functions for RmlElement
impl RmlElement {
    // returns a reference to the children field
    pub fn get_children(&self) -> Vec<&RmlElement> {
        let mut output : Vec<&RmlElement> = Vec::new();
        let _ = &self.children.iter().for_each(|child| output.push(child));
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
    pub fn value(&self) -> String {
        String::from(&self.value)   
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_element_creation() {
        assert_eq!(RmlElement::default(), RmlElement {
            name: String::from("DefaultElement"),
            children: Vec::new(),
            tags: Vec::new()
        })
    }
}