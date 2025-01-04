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
use crate::tables::{get_header_style, get_row_style, get_selected_row_style, get_table_headers, get_table_row, set_table_footer, set_table_scrollbar, TableColors};

const MAIN_COLOR: tailwind::Palette = tailwind::BLUE;
const ITEM_HEIGHT: usize = 3;
const INFO_TEXT: [&str; 1] = [
  "(Esc) quit | (↑) move up | (↓) move down | (a) Add to selected extensions",
];


pub fn display_extensions(data: Vec<Extension>) -> Result<()> {
  color_eyre::install()?;
  let terminal = ratatui::init();
  let app_result = App::new(data).run(terminal);
  ratatui::restore();
  app_result
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

  fn toggle_status(&mut self) {
    self.selected = !self.selected;
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

  pub fn update_row_status(&mut self) {
    // if let Some(selected) = self.state.selected() {
    //   let item = &mut items[selected];
    //   item.toggle_status();
    // }
    println!("{:?}", self.state.selected());
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
    let header_labels = ["TLD", "Name", "Selected"];
    let header_style = get_header_style(&self.colors);
    let header = get_table_headers(header_labels, header_style);

    let selected_row_style = get_selected_row_style(&self.colors);

    let rows = self.items.iter().enumerate().map(|(i, data)| {
      let row_values = [&data.tld, &data.name, data.selected()];
      let row_style = get_row_style(i, &self.colors);
      get_table_row(row_values, row_style)
    });

    let widths = vec![
      Constraint::Length(self.longest_item_lens.0 + 1), // + 1 is for padding.
      Constraint::Min(self.longest_item_lens.1 + 1),
      Constraint::Min(self.longest_item_lens.0 + 1)
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

  fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
    set_table_scrollbar(&mut self.scroll_state, frame, area);
  }

  fn render_footer(&self, frame: &mut Frame, area: Rect) {
    set_table_footer(&self.colors, frame, area, INFO_TEXT);
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