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
let selected = sudoku_grid.state.selected().unwrap_or_else(|| 81);
if let Some(element) = sudoku_grid.items.get_element(selected) {
    println!("{}", element.is_clue());
}
terminal.set_cursor(cursor.0, cursor.1)?;
*/

pub struct StatefulGrid {
    items: Sudoku,
    state: TableState,
}

impl StatefulGrid {
    fn new(sudoku_raw: Vec<[u8; 9]>) -> StatefulGrid {
        let mut table = StatefulGrid {
            items: Sudoku::from(sudoku_raw),
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

fn create_styled_cell(content: String, style: &str) -> Cell {
    let cell = Cell::from(content);
    match style {
        "selected" => cell.style(
            Style::default()
                .add_modifier(Modifier::REVERSED)
                .add_modifier(Modifier::BOLD),
        ),
        "selected_clue" => cell.style(
            Style::default()
                .bg(Color::Cyan)
                .add_modifier(Modifier::REVERSED),
        ),
        "clue" => cell.style(Style::default().fg(Color::LightCyan)),
        &_ => cell,
    }
}

fn enter_value(sudoku_grid: &mut StatefulGrid, value: u8) -> Result<(), std::io::Error> {
    if let Some(grid_index) = sudoku_grid.state.selected() {
        sudoku_grid.items.set_element(grid_index, value);
    }
    Ok(())
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

    let mut sudoku_grid = StatefulGrid::new(sudoku_raw);

    terminal.clear()?;
    loop {
        let mut sudoku_rows = Vec::new();
        for index in 0..9 {
            if let Some(el_row) = sudoku_grid.items.row(index) {
                let row_cells: Vec<Cell> = el_row
                    .iter()
                    .enumerate()
                    .map(|(i, el)| {
                        let mut style = "default";
                        if let Some(selected_cell_index) = sudoku_grid.state.selected() {
                            if selected_cell_index == (index * 9) + i {
                                if el.is_clue() {
                                    style = "selected_clue";
                                } else {
                                    style = "selected";
                                }
                            } else if el.is_clue() {
                                style = "clue";
                            }
                        }
                        return create_styled_cell(el.to_string(), style);
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
            f.render_stateful_widget(table, size, &mut sudoku_grid.state);
        })?;

        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Right => sudoku_grid.right(),
                KeyCode::Left => sudoku_grid.left(),
                KeyCode::Down => sudoku_grid.down(),
                KeyCode::Up => sudoku_grid.up(),
                KeyCode::Esc | KeyCode::Char('q') => {
                    terminal.set_cursor(0, height)?;
                    disable_raw_mode()?;
                    break;
                }
                KeyCode::Char('1') => enter_value(&mut sudoku_grid, 1)?,
                KeyCode::Char('2') => enter_value(&mut sudoku_grid, 2)?,
                KeyCode::Char('3') => enter_value(&mut sudoku_grid, 3)?,
                KeyCode::Char('4') => enter_value(&mut sudoku_grid, 4)?,
                KeyCode::Char('5') => enter_value(&mut sudoku_grid, 5)?,
                KeyCode::Char('6') => enter_value(&mut sudoku_grid, 6)?,
                KeyCode::Char('7') => enter_value(&mut sudoku_grid, 7)?,
                KeyCode::Char('8') => enter_value(&mut sudoku_grid, 8)?,
                KeyCode::Char('9') => enter_value(&mut sudoku_grid, 9)?,
                KeyCode::Backspace => enter_value(&mut sudoku_grid, 0)?,
                _ => (),
            },
            Event::Mouse(event) => println!("{:?}", event),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }
    }

    Ok(())
}
