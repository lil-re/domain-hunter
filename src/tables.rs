use ratatui::{
  layout::{Margin, Rect},
  style::{Color, Modifier, Style},
  text::Text,
  widgets::{
    Block, BorderType, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation,
    ScrollbarState, TableState,
  }, Frame,
};
use ratatui::style::palette::tailwind;

/// Constants
const MAIN_COLOR: tailwind::Palette = tailwind::BLUE;
const ITEM_HEIGHT: usize = 3;
const INFO_TEXT: [&str; 1] = [
  "(Esc) quit | (↑) move up | (↓) move down | (a) Add to selected extensions",
];

/// Table colors
pub struct TableColors {
  pub(crate) buffer_bg: Color,
  pub(crate) header_bg: Color,
  pub(crate) header_fg: Color,
  pub(crate) row_fg: Color,
  pub(crate) selected_row_style_fg: Color,
  pub(crate) normal_row_color: Color,
  pub(crate) alt_row_color: Color,
  pub(crate) footer_border_color: Color,
}

impl TableColors {
  pub const fn new(color: &tailwind::Palette) -> Self {
    Self {
      buffer_bg: tailwind::SLATE.c950,
      header_bg: color.c900,
      header_fg: tailwind::SLATE.c200,
      row_fg: tailwind::SLATE.c200,
      selected_row_style_fg: color.c400,
      normal_row_color: tailwind::SLATE.c950,
      alt_row_color: tailwind::SLATE.c900,
      footer_border_color: color.c400,
    }
  }
}

/// Helper functions to generate the table content
pub fn get_header_style(colors: &TableColors) -> Style {
  Style::default()
      .fg(colors.header_fg)
      .bg(colors.header_bg)
}

pub fn get_table_headers(labels: Vec<&str>, header_style: Style) -> Row {
  labels.into_iter()
      .map(Cell::from)
      .collect::<Row>()
      .style(header_style)
      .height(1)
}

pub fn get_selected_row_style(colors: &TableColors) -> Style {
  Style::default()
      .add_modifier(Modifier::REVERSED)
      .fg(colors.selected_row_style_fg)
}

pub fn get_row_style(index: usize, colors: &TableColors) -> (Color, Color) {
  let fg = colors.row_fg;
  let bg = match index % 2 {
    0 => colors.normal_row_color,
    _ => colors.alt_row_color,
  };
  (fg, bg)
}

pub fn get_table_row(row_values: Vec<&str>, row_style: (Color, Color)) -> Row {
  row_values.into_iter()
      .map(|content| Cell::from(Text::from(format!("\n{}\n", content))))
      .collect::<Row>()
      .style(Style::new().fg(row_style.0).bg(row_style.1))
      .height(3)
}

/// Table
pub trait TableBehavior {
  fn next_row(&mut self);
  fn previous_row(&mut self);
  fn set_color(&mut self);
  fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect);
  fn render_footer(&self, frame: &mut Frame, area: Rect);
}

/// BaseTable
/// Common table state for both DomainTable and ExtensionTable
pub struct BaseTable<T> {
  pub(crate) state: TableState,
  pub(crate) items: Vec<T>, // List of domain or extension
  pub(crate) scroll_state: ScrollbarState,
  pub(crate) colors: TableColors,
}

impl<T> BaseTable<T> {
  /// Create a new instance of the Table
  pub fn new(items: Vec<T>) -> Self {
    Self {
      state: TableState::default().with_selected(0),
      scroll_state: ScrollbarState::new((items.len() - 1) * ITEM_HEIGHT),
      colors: TableColors::new(&MAIN_COLOR),
      items,
    }
  }
}

impl<T> TableBehavior for BaseTable<T> {
  fn next_row(&mut self) {
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

  fn previous_row(&mut self) {
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

  fn set_color(&mut self) {
    self.colors = TableColors::new(&MAIN_COLOR);
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
