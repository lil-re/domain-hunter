use std::iter::Map;
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
use ratatui::style::palette::tailwind;

pub struct TableColors {
  pub(crate) buffer_bg: Color,
  pub(crate) header_bg: Color,
  pub(crate) header_fg: Color,
  pub(crate) row_fg: Color,
  pub(crate) selected_row_style_fg: Color,
  pub(crate) selected_column_style_fg: Color,
  pub(crate) selected_cell_style_fg: Color,
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
      selected_column_style_fg: color.c400,
      selected_cell_style_fg: color.c600,
      normal_row_color: tailwind::SLATE.c950,
      alt_row_color: tailwind::SLATE.c900,
      footer_border_color: color.c400,
    }
  }
}

pub fn get_header_style(colors: &TableColors) -> Style {
  Style::default()
      .fg(colors.header_fg)
      .bg(colors.header_bg)
}

pub fn get_table_headers(labels: [&str; 3], header_style: Style) -> Row {
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

pub fn get_table_row(row_values: [&str; 3], row_style: (Color, Color)) -> Row {
  row_values.into_iter()
      .map(|content| Cell::from(Text::from(format!("\n{}\n", content))))
      .collect::<Row>()
      .style(Style::new().fg(row_style.0).bg(row_style.1))
      .height(3)
}

pub fn set_table_scrollbar(scroll_state: &mut ScrollbarState, frame: &mut Frame, area: Rect) {
  frame.render_stateful_widget(
    Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(None)
        .end_symbol(None),
    area.inner(Margin {
      vertical: 1,
      horizontal: 1,
    }),
    scroll_state,
  );
}

pub fn set_table_footer(colors: &TableColors, frame: &mut Frame, area: Rect, text: [&str; 1]) {
  let info_footer = Paragraph::new(Text::from_iter(text))
      .style(
        Style::new()
            .fg(colors.row_fg)
            .bg(colors.buffer_bg),
      )
      .centered()
      .block(
        Block::bordered()
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(colors.footer_border_color)),
      );
  frame.render_widget(info_footer, area);
}
