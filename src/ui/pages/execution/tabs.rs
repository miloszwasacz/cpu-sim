use super::{HelpItem, Tab};
use crate::ui::events::EventResult;
use crate::ui::focus::{Focus, Focusable, HasFocus};

use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};

pub struct ExecutionTabs {
    tab: Tab,
    focused: bool,
}

impl ExecutionTabs {
    pub fn new(tab: Tab) -> Self {
        Self {
            tab,
            focused: Default::default(),
        }
    }

    pub(super) fn help(&self) -> impl IntoIterator<Item = HelpItem<'_, '_>> {
        const SWITCH_TAB: (&str, &str) = ("←→", "Switch Tab");
        [SWITCH_TAB]
    }

    pub fn handle_event(&mut self, event: Event, _: ()) -> EventResult<Tab> {
        match event {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                KeyCode::Left if !matches!(self.tab, Tab::Dashboard) => {
                    self.tab = self.tab.prev();
                    EventResult::Handled(self.tab)
                }
                KeyCode::Right if !matches!(self.tab, Tab::Console) => {
                    self.tab = self.tab.next();
                    EventResult::Handled(self.tab)
                }
                _ => Default::default(),
            },
            _ => Default::default(),
        }
    }
}

impl Focusable for ExecutionTabs {
    fn unfocus(&mut self) {
        self.focused = false;
    }

    fn focus_next(&mut self) -> HasFocus {
        self.focused = !self.focused;
        self.focused
    }

    fn focus_prev(&mut self) -> HasFocus {
        self.focused = !self.focused;
        self.focused
    }
}
