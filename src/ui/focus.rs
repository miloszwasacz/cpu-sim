pub trait Focus {
    fn next(self) -> Self;
    fn prev(self) -> Self;
}

pub trait FocusHandler: Focusable {
    fn focused(&mut self) -> Option<&mut dyn Focusable>;
    fn focus_next(&mut self);
    fn focus_prev(&mut self);
}

pub trait Focusable {
    fn unfocus(&mut self);
    fn focus_next(&mut self) -> HasFocus;
    fn focus_prev(&mut self) -> HasFocus;
}

impl<T: FocusHandler> Focusable for T {
    fn unfocus(&mut self) {
        while self.focused().is_some() {
            Focusable::focus_prev(self);
        }
    }

    fn focus_next(&mut self) -> HasFocus {
        if let Some(current) = self.focused() {
            if Focusable::focus_next(current) {
                return true;
            }
        }

        loop {
            FocusHandler::focus_next(self);
            match self.focused() {
                None => return false,
                Some(focused) => {
                    if Focusable::focus_next(focused) {
                        return true;
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    fn focus_prev(&mut self) -> HasFocus {
        {
            if let Some(current) = self.focused() {
                if Focusable::focus_prev(current) {
                    return true;
                }
            }

            loop {
                FocusHandler::focus_prev(self);
                match self.focused() {
                    None => return false,
                    Some(focused) => {
                        if Focusable::focus_prev(focused) {
                            return true;
                        } else {
                            continue;
                        }
                    }
                }
            }
        }
    }
}

pub type HasFocus = bool;
