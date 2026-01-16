use self::out_stream::OutStream;
use super::HelpItem;
use crate::ui::component::StatefulComponent;
use crate::ui::events::{EventHandler, EventResult};
use crate::ui::focus::{Focus, FocusHandler, Focusable};
use crate::ui::StdStream;

use cpu_sim::os::Os as GenericOs;
use cpu_sim_derive::Focus;
use ratatui::crossterm::event::Event;
use ratatui::prelude::*;

type Os = GenericOs<StdStream, StdStream, StdStream>;

mod out_stream;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Focus)]
enum Focused {
    #[none]
    None,
    Stdout,
    Stderr,
}

pub struct Console {
    stdout: OutStream,
    stderr: OutStream,
    focused: Focused,
}

impl Default for Console {
    fn default() -> Self {
        Self {
            stdout: OutStream::new("STDOUT"),
            stderr: OutStream::new("STDERR"),
            focused: Default::default(),
        }
    }
}

impl Console {
    pub(super) fn help(&self) -> impl IntoIterator<Item = HelpItem<'_, '_>> {
        const SCROLL: (&str, &str) = ("↑↓", "Scroll");
        const HOME: (&str, &str) = ("Home", "Scroll to bottom");
        [SCROLL, HOME]
    }
}

impl StatefulComponent for Console {
    type Model = Os;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        let [stdout, stderr] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

        self.stdout.render(model.stdout(), stdout, buf);
        self.stderr.render(model.stderr(), stderr, buf);
    }
}

impl<'a> EventHandler<&'a mut Os> for Console {
    type EventResult = ();

    fn handle_event(
        &mut self,
        event: Event,
        _payload: &'a mut Os,
    ) -> EventResult<Self::EventResult> {
        match self.focused {
            Focused::None => Default::default(),
            Focused::Stdout => self.stdout.handle_event(event, ()),
            Focused::Stderr => self.stderr.handle_event(event, ()),
        }
    }
}

impl FocusHandler for Console {
    fn focused(&mut self) -> Option<&mut dyn Focusable> {
        Some(match self.focused {
            Focused::None => return None,
            Focused::Stdout => &mut self.stdout,
            Focused::Stderr => &mut self.stderr,
        })
    }

    fn focus_next(&mut self) {
        self.focused = self.focused.next();
    }

    fn focus_prev(&mut self) {
        self.focused = self.focused.prev();
    }
}
