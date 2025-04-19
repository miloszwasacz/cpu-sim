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
