use color_eyre::Result;
use ratatui::{
  crossterm::event::{self, Event, KeyCode, KeyEventKind},
  layout::{Constraint, Layout, Rect},
  style::{Stylize},
  text::Text,
  widgets::{HighlightSpacing, Table},
  DefaultTerminal, Frame,
};
use crate::database::extensions_api::update_extension;
use crate::models::{Extension, Selectable};
use crate::tables::base_table::{get_header_style, get_row_style, get_selected_row_style, get_table_headers, get_table_row, BaseTable, TableBehavior};

const INFO_TEXTS: [&str; 1] = [
  "(Esc) quit | (↑) move up | (↓) move down | (s) Select or unselect extension",
];

pub fn display_extensions(data: Vec<Extension>) -> Result<()> {
  color_eyre::install()?;
  let terminal = ratatui::init();
  let app_result = BaseTable::new(data).run(terminal);
  ratatui::restore();
  app_result
}

impl BaseTable<Extension> {
  /// Select or unselect an extension
  pub fn update_row_status(&mut self) {
    if let Some(selected) = self.state.selected() {
      self.items[selected].toggle_status();
      update_extension(&self.items[selected]);
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
            KeyCode::Char('s') => self.update_row_status(),
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
    self.render_footer(frame, rects[1], INFO_TEXTS);
  }

  fn render_table(&mut self, frame: &mut Frame, area: Rect) {
    let header_labels = vec!["TLD", "Name", "Selected"];
    let header_style = get_header_style(&self.colors);
    let header = get_table_headers(header_labels, header_style);

    let selected_row_style = get_selected_row_style(&self.colors);

    let rows = self.items.iter().enumerate().map(|(i, data)| {
      let row_values = vec![data.tld.clone(), data.name.clone(), data.is_selected()];
      let row_style = get_row_style(i, &self.colors);
      get_table_row(row_values, row_style)
    });

    let widths = vec![
      Constraint::Min(10),
      Constraint::Min(20),
      Constraint::Min(20)
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
