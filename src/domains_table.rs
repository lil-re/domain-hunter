//! # [Ratatui] Table example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use color_eyre::Result;
use ratatui::{
  crossterm::event::{self, Event, KeyCode, KeyEventKind},
  layout::{Constraint, Layout, Rect},
  style::{Stylize},
  text::Text,
  widgets::{HighlightSpacing, Table},
  DefaultTerminal, Frame,
};
use crate::models::{Domain, Selectable};
use crate::tables::{get_header_style, get_row_style, get_selected_row_style, get_table_headers, get_table_row, BaseTable, TableBehavior};
use crate::wishlist_file::{add_to_wishlist, remove_from_wishlist};

pub fn display_domains(data: Vec<Domain>) -> Result<()> {
  color_eyre::install()?;
  let terminal = ratatui::init();
  let app_result = BaseTable::new(data).run(terminal);
  ratatui::restore();
  app_result
}

impl BaseTable<Domain> {
  pub fn update_row_status(&mut self) {
    if let Some(index) = self.state.selected() {
      self.items[index].toggle_status();

      if self.items[index].selected {
        add_to_wishlist(self.items[index].clone())
      } else {
        remove_from_wishlist(self.items[index].clone())
      }
    }
  }

  fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
    loop {
      terminal.draw(|frame| self.draw(frame))?;

      if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
          match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
            KeyCode::Char('j') | KeyCode::Down => self.next_row(),
            KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
            KeyCode::Char('a') => self.update_row_status(),
            _ => {}
          }
        }
      }
    }
  }

  fn draw(&mut self, frame: &mut Frame) {
    let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
    let rects = vertical.split(frame.area());

    self.set_color();
    self.render_table(frame, rects[0]);
    self.render_scrollbar(frame, rects[0]);
    self.render_footer(frame, rects[1]);
  }

  fn render_table(&mut self, frame: &mut Frame, area: Rect) {
    let header_labels = vec!["Domain", "Extension", "Status", "Wishlist"];
    let header_style = get_header_style(&self.colors);
    let header = get_table_headers(header_labels, header_style);

    let selected_row_style = get_selected_row_style(&self.colors);

    let rows = self.items.iter().enumerate().map(|(i, data)| {
      let row_values = vec![&data.domain, &data.tld, data.available(), data.selected()];
      let row_style = get_row_style(i, &self.colors);
      get_table_row(row_values, row_style)
    });

    let widths = vec![
      Constraint::Min(10),
      Constraint::Min(20),
      Constraint::Min(20),
      Constraint::Min(20),
    ];

    let t = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(selected_row_style)
        .highlight_symbol(Text::from(vec![
          "".into(),
          " > ".into(),
          "".into(),
        ]))
        .bg(self.colors.buffer_bg)
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(t, area, &mut self.state);
  }
}
