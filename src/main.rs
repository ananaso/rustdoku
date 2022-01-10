use sudoku::Box;
use tui::backend::CrosstermBackend;
use tui::layout::Constraint;
use tui::widgets::{Block, Row, Table};
use tui::Terminal;
mod sudoku;

fn main() -> Result<(), std::io::Error> {
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

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
    let mut sudoku_boxes = Vec::new();
    for index in 0..sudoku_raw.len() {
        let block_raw = sudoku_raw[index];
        let block = Box::new(block_raw);
        sudoku_boxes.push(block);
    }

    let sudoku_rows = vec![Row::default(); 9];

    // println!("{:?}", block0_data);

    let table = Table::new(sudoku_rows)
        .block(Block::default().title("Table"))
        .widths(&[
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .column_spacing(1);

    terminal.draw(|f| {
        let size = f.size();
        f.render_widget(table, size);
    })?;
    Ok(())
}
