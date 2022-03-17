use tui::style::{Color, Modifier, Style};

pub struct Theme {
  pub procs_item: Style,
  pub procs_item_active: Style,
}

impl Theme {
  pub fn pane_title(&self, active: bool) -> Style {
    let style = Style::default();
    if active {
      style.fg(Color::White).add_modifier(Modifier::BOLD)
    } else {
      style.fg(Color::Rgb(180, 180, 180))
    }
  }
  pub fn pane_border(&self, active: bool) -> Style {
    let style = Style::default();
    if active {
      style.fg(Color::Rgb(230, 230, 230))
    } else {
      style.fg(Color::Rgb(180, 180, 180))
    }
  }

  pub fn get_procs_item(&self, active: bool) -> Style {
    if active {
      self.procs_item_active
    } else {
      self.procs_item
    }
  }
}

impl Default for Theme {
  fn default() -> Self {
    Self {
      procs_item: Style::default(),
      procs_item_active: Style::default().bg(Color::Rgb(50, 50, 50)),
    }
  }
}