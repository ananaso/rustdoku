use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use sudoku::Sudoku;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Cell, Row, Table, TableState};
use tui::Terminal;
mod sudoku;

/*
let cursor = terminal.get_cursor()?;
terminal.set_cursor(0, 40)?;
println!("{}", string);
terminal.set_cursor(cursor.0, cursor.1)?;
*/

pub struct StatefulGrid {
    state: TableState,
}

impl StatefulGrid {
    fn new() -> StatefulGrid {
        let mut table = StatefulGrid {
            state: TableState::default(),
        };
        table.state.select(Some(0));
        return table;
    }

    pub fn right(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i % 9 < 8 {
                    i + 1
                } else {
                    i - 8
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn left(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i % 9 > 0 {
                    i - 1
                } else {
                    i + 8
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn down(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i < 72 {
                    i + 9
                } else {
                    i - 72
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn up(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i > 8 {
                    i - 9
                } else {
                    i + 72
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn main() -> Result<(), std::io::Error> {
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    enable_raw_mode()?;

    let sudoku_raw = vec![
        [5, 3, 0, 6, 0, 0, 0, 9, 8],
        [0, 7, 0, 1, 9, 5, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 4, 0, 0, 7, 0, 0],
        [0, 6, 0, 8, 0, 3, 0, 2, 0],
        [0, 0, 3, 0, 0, 1, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 4, 1, 9, 0, 8, 0],
        [2, 8, 0, 0, 0, 5, 0, 7, 9],
    ];
    let sudoku_boxes = Sudoku::from(sudoku_raw);

    let mut table_state = StatefulGrid::new();

    terminal.clear()?;
    println!("TODO: table or sudoku_boxes doesn't seem to build exactly from sudoku_raw correctly");
    loop {
        let mut sudoku_rows = Vec::new();
        for index in 0..9 {
            if let Some(el_row) = sudoku_boxes.row(index) {
                let row_cells: Vec<Cell> = el_row
                    .iter()
                    .enumerate()
                    .map(|(i, el)| {
                        let mut new_cell = Cell::from(el.to_string());
                        if let Some(selected_cell_index) = table_state.state.selected() {
                            if selected_cell_index == (index * 9) + i {
                                new_cell = Cell::from(el.to_string()).style(
                                    Style::default()
                                        .fg(Color::Black)
                                        .bg(Color::White)
                                        .add_modifier(Modifier::BOLD),
                                );
                            }
                        }
                        return new_cell;
                    })
                    .collect();
                sudoku_rows.push(Row::new(row_cells));
            }
        }

        let table = Table::new(sudoku_rows.to_vec())
            .block(Block::default().title("Sudoku").borders(Borders::ALL))
            .widths(&[
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
            ])
            .column_spacing(1);

        let width = 3 * 9 + 1;
        let height = 3 * 3 + 2;

        terminal.draw(|f| {
            // let size = f.size();
            let size = Rect {
                x: 0,
                y: 0,
                width,
                height,
            };
            f.render_stateful_widget(table, size, &mut table_state.state);
        })?;

        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Right => table_state.right(),
                KeyCode::Left => table_state.left(),
                KeyCode::Down => table_state.down(),
                KeyCode::Up => table_state.up(),
                KeyCode::Esc | KeyCode::Char('q') => {
                    terminal.set_cursor(0, height)?;
                    disable_raw_mode()?;
                    break;
                }
                KeyCode::Enter => {
                    let cursor = terminal.get_cursor()?;
                    terminal.set_cursor(0, 40)?;
                    let selected = table_state.state.selected().unwrap_or_else(|| 81);
                    if let Some(element) = sudoku_boxes.get_element(selected) {
                        println!("{:?}", element);
                    } else {
                        println!("Uh-oh, you selected an element that doesn't exist");
                    }
                    terminal.set_cursor(cursor.0, cursor.1)?;
                }
                other => println!("\r\n{:?}", other),
            },
            Event::Mouse(event) => println!("{:?}", event),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }
    }

    Ok(())
}
