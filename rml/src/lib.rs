use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct RmlTag {
    pub name: String,
    pub value: String
}
    
#[derive(Debug, PartialEq, Clone)]
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
    // builder
    pub fn new(name: String) -> Self {
        RmlElement {
            name,
            tags: Vec::new(),
            children: Vec::new()
        }
    }

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

pub fn is_start_tag(tag: String) -> bool {
    !tag.contains("!") && is_tag(tag)
}

pub fn is_end_tag(tag: String) -> bool {
    !is_start_tag(tag)
}

pub fn is_tag(tag: String) -> bool {
    tag.contains("[") && tag.contains("]")
}

pub fn is_rmltag(tag: String) -> bool {
    tag.contains("=")
}

pub fn get_tag_name(tag: String) -> String {
    tag.replace("[", "").replace("]", "").replace("!", "").to_string()
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
//  content = parse_to_vec(data)
//  scope = new vector
//  current_parent = RmlElement
//  child_hashmap = new hashmap()
//  for thing in content do:
//      if is_start_tag(thing) do:
//          scope.add(thing)
//          current_parent = new RmlElement(thing)
//      else if is_end_tag(thing) do:
//          scope.remove(thing)
//          if not scope.is_empty() do:
//              child_hashmap.key(scope.last()).add(thing)
//          else:
//              return current_parent
// }

fn parse_to_vec(data: String) -> Vec<String> {
    let mut output : Vec<String> = Vec::new();
    let mut substring = String::from("");
    let mut in_tag = false;
    data.chars().for_each(|char| {
        if char == '[' {
            if !in_tag {
                in_tag = true
            } else if !substring.is_empty() {
                output.push(substring.clone());
                substring.clear();
            }
        }
        substring += &char.to_string();
        if char == ']' {
            output.push(substring.clone());
            substring.clear();
        }
    });
    output
}

pub fn parse_tag_from_string(data: String) -> RmlTag {
    RmlTag { name: data.split("=").next().unwrap().to_string(), value: data.split("=").nth(1).unwrap().to_string() }
}

fn get_index<T: PartialEq>(vector: Vec<T>, value: T) -> usize {
    vector.iter().position(|element| *element==value).unwrap()
}

pub fn parse_from_string(data: String) -> RmlElement {
    let content = parse_to_vec(data);
    let mut scope : Vec<String> = Vec::new();
    let mut child_hashmap : HashMap<String, Vec<RmlElement>> = HashMap::new();
    let mut current_parent : Vec<RmlElement> = Vec::new();
    let mut current_parent_names : Vec<String> = Vec::new();
    content.iter().for_each(|tag_or_content_in| {
        let mut tag_or_content = tag_or_content_in.split(" ").next().unwrap().to_string();
        let tags: Vec<String> = get_tag_name(tag_or_content_in.to_string()).split(" ").filter(|tag| is_rmltag(tag.to_string())).map(|tag| tag.to_string()).collect::<Vec<_>>();
        if tag_or_content_in.contains("]"){
            tag_or_content += "]";
        }
        if is_tag(tag_or_content.to_string()) {
            if is_start_tag(tag_or_content.to_string()) {
                scope.push(get_tag_name(tag_or_content.to_string()));
                current_parent.push(RmlElement::new(get_tag_name(tag_or_content.to_string())));
                tags.iter().for_each(|tag| current_parent.last_mut().unwrap().add_tag(parse_tag_from_string(tag.to_string())));
                current_parent_names.push(get_tag_name(tag_or_content.to_string()));
            } else if is_end_tag(tag_or_content.to_string()) {
                let index = scope.iter().position(|element| *element==*get_tag_name(tag_or_content.to_string()));
                scope.remove(index.unwrap());
                if child_hashmap.contains_key(&get_tag_name(tag_or_content.to_string())) {
                    child_hashmap.get(&get_tag_name(tag_or_content.to_string())).expect("tag_or_content doesnt exist in child_hashmap").iter().for_each(|child| {
                    current_parent.get_mut(get_index(current_parent_names.clone(), get_tag_name(tag_or_content.to_string()))).unwrap().add_child(child.clone());
                        
                    });
                }
                if !scope.is_empty() {
                    child_hashmap.entry(scope.last().unwrap().to_string()).or_insert_with(|| {Vec::new()}).push(current_parent.iter().last().unwrap().clone());

                }
            }
            

        } else {
            current_parent.last_mut().unwrap().add_tag(RmlTag { name: "*content*".to_string(), value: tag_or_content.to_string() });
        }
    });
    current_parent.first().unwrap().clone()
}

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
