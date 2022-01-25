use std::fmt;

/*
    Note:   Modulo (%) by 9 to get the index within a row (i.e. horizontal movement)
            Divide (/) by 9 to get the index within a col (i.e. vertical movement)
*/

#[derive(Debug)]
pub struct Sudoku {
    boxes: [Box; 9],
}

impl From<Vec<[u8; 9]>> for Sudoku {
    fn from(sudoku_raw: Vec<[u8; 9]>) -> Self {
        let mut boxes = [Box::default(); 9];
        for index in 0..boxes.len() {
            boxes[index] = Box::new(sudoku_raw[index]);
        }
        Sudoku { boxes }
    }
}

impl Sudoku {
    pub fn row(&self, index: usize) -> Option<Vec<&Element>> {
        if index <= 8 {
            let mut row: Vec<&Element> = Vec::new();
            let box_index = (index / 3) * 3;
            let box_row_index = index % 3;
            for i in box_index..(box_index + 3) {
                let mut block_row = self.boxes[i].row(box_row_index);
                row.append(&mut block_row);
            }
            Some(row)
        } else {
            None
        }
    }

    // pub fn col(&self, index: usize) -> Option<Vec<Element>> {
    //     if index <= 8 {
    //         let mut col = Vec::new();
    //         let block_index = index / 3;
    //         let col_index = index % 3;
    //         for i in 0..3 {
    //             let block_col = self.boxes[block_index + i].col(col_index);
    //             block_col.iter().for_each(|el| col.push(el.to_owned()));
    //         }
    //         Some(col)
    //     } else {
    //         None
    //     }
    // }

    pub fn get_element(&self, index: usize) -> Option<&Element> {
        if let Some((box_index, element_index)) = Sudoku::deconstruct_index(index) {
            let selected_box = &self.boxes[box_index];
            if let Some(element) = selected_box.get_element(element_index) {
                Some(element)
            } else {
                None
            }
        } else {
            None
        }
    }

    // Returns two indexes:
    //  the first is the box index
    //  the second is the element index within that box
    fn deconstruct_index(index: usize) -> Option<(usize, usize)> {
        if index <= 80 {
            // break down index from whole sudoku grid into x/y
            let sudoku_index_row = index % 9;
            let sudoku_index_col = index / 9;

            // reduce grid x/y down to 9-box equivalent
            let box_index_row = sudoku_index_row / 3;
            let box_index_col = sudoku_index_col / 3;
            // combine box x/y into singular array index
            // Note: not eqv. to "sudoku_index_col + box_index_row"
            //          b/c of how rust math truncates on division
            let box_index = (box_index_col * 3) + box_index_row;

            // reduce grid x/y down to 9-cell equivalent
            let cell_index_row = sudoku_index_row % 3;
            let cell_index_col = sudoku_index_col % 3;
            // combine cell x/y into singular array index
            let cell_index = (cell_index_col * 3) + cell_index_row;
            Some((box_index, cell_index))
        } else {
            None
        }
    }
}

// BOX
#[derive(Clone, Copy, Debug, Default)]
struct Box {
    elements: [Element; 9],
}

impl Box {
    fn new(elements: [u8; 9]) -> Box {
        let mut el_arr = [Element::default(); 9];
        for index in 0..9 {
            let value = elements[index];
            let is_clue = value > 0;
            if let Some(element) = Element::new(value, is_clue) {
                el_arr[index] = element;
            }
        }
        Box { elements: el_arr }
    }

    fn row(&self, value: usize) -> Vec<&Element> {
        let value = value * 3;
        let mut row = Vec::new();
        for index in value..(value + 3) {
            row.push(&self.elements[index]);
        }
        return row;
    }

    // pub fn col(&self, value: usize) -> [Element; 3] {
    //     let mut col = [Element::default(); 3];
    //     for index in 0..3 {
    //         col[index] = self.elements[value + (index * 3)]
    //     }
    //     return col;
    // }

    fn get_element(&self, index: usize) -> Option<&Element> {
        if index <= 8 {
            Some(&self.elements[index])
        } else {
            None
        }
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
#[derive(Clone, Copy, Debug, Default)]
pub struct Element {
    value: u8,
    is_clue: bool,
}

impl Element {
    fn new(value: u8, is_clue: bool) -> Option<Element> {
        if value <= 9 {
            Some(Element { value, is_clue })
        } else {
            None
        }
    }

    pub fn is_clue(&self) -> bool {
        self.is_clue
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value > 0 {
            write!(f, "{}", self.value)
        } else {
            write!(f, " ")
        }
    }
}
