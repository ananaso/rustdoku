use sudoku::Sudoku;
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
    let sudoku_boxes = Sudoku::from(sudoku_raw);

    let mut sudoku_rows = Vec::new();
    for index in 0..9 {
        if let Some(el_row) = sudoku_boxes.row(index) {
            let str_row: Vec<String> = el_row.iter().map(|el| el.to_string()).collect();
            sudoku_rows.push(Row::new(str_row));
        }
    }

    let table = Table::new(sudoku_rows)
        .block(Block::default().title("Table"))
        .widths(&[Constraint::Length(2); 9])
        .column_spacing(1);

    terminal.draw(|f| {
        let size = f.size();
        f.render_widget(table, size);
    })?;
    Ok(())
}
