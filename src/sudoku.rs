use std::fmt;

// BLOCK
#[derive(Default)]
pub struct Box {
    elements: [Element; 9],
}

impl Box {
    pub fn new(elements: [u8; 9]) -> Box {
        let mut el_arr = [Element::default(); 9];
        for index in 0..9 {
            let value = elements[index];
            if let Some(element) = Element::new(value) {
                el_arr[index] = element;
            }
        }
        Box { elements: el_arr }
    }

    pub fn row(&self, value: usize) -> [Element; 3] {
        let value = value * 3;
        let mut row = [Element::default(); 3];
        for index in 0..3 {
            row[index] = self.elements[index + value];
        }
        return row;
    }

    pub fn col(&self, value: usize) -> [Element; 3] {
        let mut col = [Element::default(); 3];
        for index in 0..3 {
            col[index] = self.elements[value + (index * 3)]
        }
        return col;
    }
}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut block_str = String::from("");
        let mut index = 0;
        for element in self.elements.iter() {
            block_str.push_str(element.to_string().as_str());
            if index % 3 == 2 && index != 8 {
                block_str.push_str("\n");
            } else {
                block_str.push_str(" ");
            }
            index += 1;
        }
        write!(f, "{}", block_str)
    }
}

// ELEMENT
#[derive(Clone, Copy, Default)]
struct Element {
    value: u8,
}

impl Element {
    fn new(value: u8) -> Option<Element> {
        if value <= 9 {
            Some(Element { value })
        } else {
            None
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
