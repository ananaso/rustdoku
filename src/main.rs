use block::Block;
use tui::backend::CrosstermBackend;
use tui::widgets::Chart;
use tui::Terminal;
mod block;

fn main() -> Result<(), std::io::Error> {
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let block0 = Block::new([0, 1, 2, 3, 4, 5, 6, 7, 8]);
    let row = block0.row(1);
    let col = block0.col(1);
    println!("Row ->");
    row.iter().for_each(|el| println!("{}", el));
    println!("\nCol ->");
    col.iter().for_each(|el| println!("{}", el));
    println!("\nBlock ->");
    println!("{}", block0);

    // let grid_values = Vec::new();

    // let chart = Chart::new(grid_values);

    // terminal.draw(|f| {
    //     let size = f.size();
    //     f.render_widget(chart, size);
    // })?;
    Ok(())
}
