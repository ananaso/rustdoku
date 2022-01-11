use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use sudoku::Sudoku;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Rect};
use tui::widgets::{Block, Borders, Cell, Row, Table, TableState};
use tui::Terminal;
mod sudoku;

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

    let mut table_state = TableState::default();

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
            f.render_widget(table, size);
        })?;

        if poll(Duration::from_secs(0))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        disable_raw_mode()?;
                        break;
                    }
                    other => println!("\r\n{:?}", other),
                },
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }
        }
    }

    Ok(())
}
