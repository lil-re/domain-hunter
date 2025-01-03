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
use crossterm::event::KeyModifiers;
use itertools::Itertools;
use ratatui::{
  crossterm::event::{self, Event, KeyCode, KeyEventKind},
  layout::{Constraint, Layout, Margin, Rect},
  style::{self, Color, Modifier, Style, Stylize},
  text::Text,
  widgets::{
    Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
    ScrollbarState, Table, TableState,
  },
  DefaultTerminal, Frame,
};
use serde::Deserialize;
use style::palette::tailwind;
use unicode_width::UnicodeWidthStr;

const MAIN_COLOR: tailwind::Palette = tailwind::BLUE;
const ITEM_HEIGHT: usize = 3;
const INFO_TEXT: [&str; 1] = [
  "(Esc) quit | (↑) move up | (↓) move down | (w) Add to wishlist",
];


pub fn display_extensions(data: Vec<Extension>) -> Result<()> {
  color_eyre::install()?;
  let terminal = ratatui::init();
  let app_result = App::new(data).run(terminal);
  ratatui::restore();
  app_result
}
struct TableColors {
  buffer_bg: Color,
  header_bg: Color,
  header_fg: Color,
  row_fg: Color,
  selected_row_style_fg: Color,
  selected_column_style_fg: Color,
  selected_cell_style_fg: Color,
  normal_row_color: Color,
  alt_row_color: Color,
  footer_border_color: Color,
}

impl TableColors {
  const fn new(color: &tailwind::Palette) -> Self {
    Self {
      buffer_bg: tailwind::SLATE.c950,
      header_bg: color.c900,
      header_fg: tailwind::SLATE.c200,
      row_fg: tailwind::SLATE.c200,
      selected_row_style_fg: color.c400,
      selected_column_style_fg: color.c400,
      selected_cell_style_fg: color.c600,
      normal_row_color: tailwind::SLATE.c950,
      alt_row_color: tailwind::SLATE.c900,
      footer_border_color: color.c400,
    }
  }
}

#[derive(Deserialize, Debug)]
pub struct Extension {
  pub(crate) tld: String,
  pub(crate) name: String,
  #[serde(default = "default_selected")]
  pub(crate) selected: bool
}

fn default_selected() -> bool {
  false
}

impl Extension {
  fn tld(&self) -> &str {
    &self.tld
  }

  fn name(&self) -> &str {
    &self.name
  }

  fn selected(&self) -> &str {
    if self.selected {
      "Selected"
    } else {
      "Not selected"
    }
  }
}

struct App {
  state: TableState,
  items: Vec<Extension>,
  longest_item_lens: (u16, u16, u16), // order is (tld, name, selected)
  scroll_state: ScrollbarState,
  colors: TableColors,
  color_index: usize,
}

impl App {
  fn new(data: Vec<Extension>) -> Self {
    // let data_vec = generate_fake_names();
    Self {
      state: TableState::default().with_selected(0),
      longest_item_lens: constraint_len_calculator(&data),
      scroll_state: ScrollbarState::new((data.len() - 1) * ITEM_HEIGHT),
      colors: TableColors::new(&MAIN_COLOR),
      color_index: 0,
      items: data,
    }
  }
  pub fn next_row(&mut self) {
    let i = match self.state.selected() {
      Some(i) => {
        if i >= self.items.len() - 1 {
          0
        } else {
          i + 1
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
    self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
  }

  pub fn previous_row(&mut self) {
    let i = match self.state.selected() {
      Some(i) => {
        if i == 0 {
          self.items.len() - 1
        } else {
          i - 1
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
    self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
  }

  pub fn set_color(&mut self) {
    self.colors = TableColors::new(&MAIN_COLOR);
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
    let header_style = Style::default()
        .fg(self.colors.header_fg)
        .bg(self.colors.header_bg);
    let selected_row_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(self.colors.selected_row_style_fg);
    let selected_col_style = Style::default().fg(self.colors.selected_column_style_fg);
    let selected_cell_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(self.colors.selected_cell_style_fg);

    let header = ["Extension", "Extension", "Status"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);
    let rows = self.items.iter().enumerate().map(|(i, data)| {
      let color = match i % 2 {
        0 => self.colors.normal_row_color,
        _ => self.colors.alt_row_color,
      };
      let item = [&data.tld, &data.name, &data.selected.to_string()];
      item.into_iter()
          .map(|content| Cell::from(Text::from(format!("\n{}\n", content))))
          .collect::<Row>()
          .style(Style::new().fg(self.colors.row_fg).bg(color))
          .height(3)
    });
    let bar = " > ";
    let t = Table::new(
      rows,
      [
        // + 1 is for padding.
        Constraint::Length(self.longest_item_lens.0 + 1),
        Constraint::Min(self.longest_item_lens.1 + 1),
        Constraint::Length(self.longest_item_lens.0 + 1)
      ]
    )
        .header(header)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from(vec![
          "".into(),
          bar.into(),
          "".into(),
        ]))
        .bg(self.colors.buffer_bg)
        .highlight_spacing(HighlightSpacing::Always);
    frame.render_stateful_widget(t, area, &mut self.state);
  }

  fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
    frame.render_stateful_widget(
      Scrollbar::default()
          .orientation(ScrollbarOrientation::VerticalRight)
          .begin_symbol(None)
          .end_symbol(None),
      area.inner(Margin {
        vertical: 1,
        horizontal: 1,
      }),
      &mut self.scroll_state,
    );
  }

  fn render_footer(&self, frame: &mut Frame, area: Rect) {
    let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
        .style(
          Style::new()
              .fg(self.colors.row_fg)
              .bg(self.colors.buffer_bg),
        )
        .centered()
        .block(
          Block::bordered()
              .border_type(BorderType::Double)
              .border_style(Style::new().fg(self.colors.footer_border_color)),
        );
    frame.render_widget(info_footer, area);
  }
}

fn constraint_len_calculator(items: &[Extension]) -> (u16, u16, u16) {
  let tld_len = items
      .iter()
      .map(Extension::tld)
      .map(UnicodeWidthStr::width)
      .max()
      .unwrap_or(0);
  let name_len = items
      .iter()
      .map(Extension::name)
      .map(UnicodeWidthStr::width)
      .max()
      .unwrap_or(0);
  let selected_len = items
      .iter()
      .map(Extension::selected)
      .map(UnicodeWidthStr::width)
      .max()
      .unwrap_or(0);

  #[allow(clippy::cast_possible_truncation)]
  (tld_len as u16, name_len as u16, selected_len as u16)
}

// #[cfg(test)]
// mod tests {
//   use crate::Extension;
//
//   #[test]
//   fn constraint_len_calculator() {
//     let test_data = vec![
//       Extension {
//         name: "Emirhan Tala".to_string(),
//         address: "Cambridgelaan 6XX\n3584 XX Utrecht".to_string(),
//         email: "tala.emirhan@gmail.com".to_string(),
//       },
//       Extension {
//         name: "thistextis26characterslong".to_string(),
//         address: "this line is 31 characters long\nbottom line is 33 characters long"
//             .to_string(),
//         email: "thisemailis40caharacterslong@ratatui.com".to_string(),
//       },
//     ];
//     let (longest_name_len, longest_tld_len, longest_selected_len) =
//         crate::constraint_len_calculator(&test_data);
//
//     assert_eq!(33, longest_tld_len);
//     assert_eq!(26, longest_name_len);
//     assert_eq!(40, longest_selected_len);
//   }
// }