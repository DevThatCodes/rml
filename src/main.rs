use rml::parser::parse_element_string;

pub mod rml {

    #[derive(Debug)]
    pub struct RmlTag {
        pub name: String,
        pub value: String
    }
    
    #[derive(Debug)]
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

    // parsing and deparsing stuff
    pub mod parser {
        use crate::rml::RmlTag;

        use super::RmlElement;

        pub fn is_start_tag(tag: String) -> bool {
            !tag.contains("!") && is_tag(tag)
        }

        pub fn is_end_tag(tag: String) -> bool {
            !is_start_tag(tag)
        }

        pub fn is_tag(tag: String) -> bool {
            tag.contains("[") && tag.contains("]")
        }

        pub fn get_tag_name(tag: String) -> String {
            tag.replace("[", "").replace("]", "").replace("!", "").to_string()
        }

        // element parser ( ex. : [exampleElement exampleTag="exampleVal"]example content[!exampleElement] )
        pub fn parse_element_string(element_string: String) -> RmlElement {
            let mut in_tag = false;
            let mut substring : String = String::from("");
            let mut output : Vec<String> = Vec::new();
            let mut tags : Vec<String> = Vec::new();
            let mut tags_as_rml_tags : Vec<RmlTag> = Vec::new();
            let mut children : Vec<Box<RmlElement>> = Vec::new();
            element_string.chars().for_each(|char| {
                if char == '[' {
                    if !in_tag {
                        in_tag = true
                    } else if !substring.is_empty() {
                            output.push(substring.clone());
                            substring = String::from("");
                    }
                }
                substring += &char.to_string();
                if char == ']' {
                    output.push(get_tag_name(substring.clone()));
                    substring = String::from("");
                }
            });
            if output.len() != 3 {
                return RmlElement::default();
            } else {
                output[0].split(" ").for_each(|possible_tag| if possible_tag.contains("=") {tags.push(possible_tag.to_string())});
                tags.iter().for_each(|tag| {
                    tags_as_rml_tags.push(RmlTag {
                        name: tag.split("=").next().unwrap().to_string(),
                        value: tag.split("=").nth(1).unwrap().replace("\"", "").to_string()
                    })
                });
                if is_tag(output[1].clone()) {
                    // doesnt ever get here tho
                    children.push(Box::new(parse_element_string(output[1].clone())))
                } else {
                    tags_as_rml_tags.push(RmlTag {
                        name: "*content*".to_string(),
                        value: output[1].clone()
                    })
                }
            }
            RmlElement {
                name: output[0].split(" ").next().unwrap().to_string(),
                children,
                tags: tags_as_rml_tags
            }
        }
    }
}

fn main() {
    println!("{:#?}", parse_element_string("[exampleElement exampleTag=\"exampleVal\"]example content[!exampleElement]".to_string()));
    println!("test element with children:{:#?}", parse_element_string("[container][text]test[!text][!container]".to_string()))
} 
