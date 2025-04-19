use ratatui::prelude::*;

pub trait Component {
    type Model;

    fn render(&self, model: &Self::Model, area: Rect, buf: &mut Buffer);
}

pub trait StatefulComponent {
    type Model;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer);
}
