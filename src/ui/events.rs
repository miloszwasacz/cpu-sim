use crate::ui::error::SimError;

use ratatui::crossterm::event::Event;

pub trait EventHandler<P> {
    type EventResult;

    #[allow(unused_variables)]
    fn handle_event(&mut self, event: Event, payload: P) -> EventResult<Self::EventResult> {
        EventResult::Ignored
    }
}

#[derive(Default)]
pub enum EventResult<T> {
    #[default]
    Ignored,
    Handled(T),
    Err(SimError),
}

impl<T> EventResult<T> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> EventResult<U> {
        match self {
            EventResult::Ignored => EventResult::Ignored,
            EventResult::Handled(r) => EventResult::Handled(f(r)),
            EventResult::Err(err) => EventResult::Err(err),
        }
    }
}
