use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use sudoku::Sudoku;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Rect};
use tui::widgets::{Block, Borders, Cell, Row, Table, TableState};
use tui::Terminal;
mod sudoku;

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
    loop {
        let mut sudoku_rows = Vec::new();
        for index in 0..9 {
            if let Some(el_row) = sudoku_boxes.row(index) {
                let row_cells: Vec<Cell> =
                    el_row.iter().map(|el| Cell::from(el.to_string())).collect();
                let mut row = Row::new(row_cells);
                if index % 3 == 2 && index != 8 {
                    row = row.bottom_margin(1);
                }
                sudoku_rows.push(row);
            }
        }

        let table = Table::new(sudoku_rows)
            .block(Block::default().title("Sudoku").borders(Borders::ALL))
            .widths(&[
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
            ])
            .column_spacing(1);

        let width = 3 * 7 + 2 * 3 + 2;
        let height = 3 * 3 + 2 + 2;

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

        if poll(Duration::from_secs(0))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Right => table_state.right(),
                    KeyCode::Left => table_state.left(),
                    KeyCode::Down => table_state.down(),
                    KeyCode::Up => table_state.up(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        break;
                    }
                    KeyCode::Enter => println!("\r\n{:?}", table_state.state.selected()),
                    other => println!("\r\n{:?}", other),
                },
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }
        }
    }

    Ok(())
}
