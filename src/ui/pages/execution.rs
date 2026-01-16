use self::console::Console;
use self::csrs::CsrDashboard;
use self::dashboard::CpuDashboard;
use self::tabs::ExecutionTabs;
use super::Page;
use crate::ui::focus::Focus;
use crate::ui::model::CpuModel;
use crate::ui::{Cpu, EventHandler, EventResult, FocusHandler, Focusable, StatefulComponent};

use cpu_sim::components::cpu::ExitCode;
use cpu_sim::components::diagnostics::Diagnostics;
use cpu_sim_derive::Focus;
use ratatui::crossterm::event::Event;
use ratatui::prelude::*;
use ratatui::symbols::DOT;
use ratatui::widgets::{Block, Borders, Cell, Row, Table, Tabs};
use std::num::NonZeroUsize;

mod console;
mod csrs;
mod dashboard;
mod tabs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Focus)]
enum Focused {
    #[none]
    Content,
    Tabs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Focus)]
enum Tab {
    #[none]
    Dashboard,
    CSRs,
    Console,
}

impl Tab {
    pub fn titles() -> [&'static str; 3] {
        ["Dashboard", "CSRs", "Console"]
    }

    pub fn index(&self) -> usize {
        match self {
            Tab::Dashboard => 0,
            Tab::CSRs => 1,
            Tab::Console => 2,
        }
    }
}

type HelpItem<'a, 'b> = (&'a str, &'b str);

pub struct ExecutionPage<'c> {
    cpu: &'c mut Cpu,
    model: CpuModel,
    tabs: ExecutionTabs,
    dashboard: CpuDashboard,
    csrs: CsrDashboard,
    console: Console,
    focused: Focused,
    tab: Tab,
}

impl<'c> ExecutionPage<'c> {
    pub fn new(cpu: &'c mut Cpu) -> Self {
        let model = cpu.diagnostics().into();
        let scheduler_count =
            NonZeroUsize::new(cpu.scheduler_count()).expect("the should be at least one scheduler");
        let tab = Default::default();

        let mut page = Self {
            cpu,
            model,
            tabs: ExecutionTabs::new(tab),
            dashboard: CpuDashboard::new(scheduler_count),
            csrs: Default::default(),
            console: Default::default(),
            focused: Default::default(),
            tab,
        };
        match page.focused {
            Focused::Content => match page.tab {
                Tab::Dashboard => Focusable::focus_next(&mut page.dashboard),
                Tab::CSRs => Focusable::focus_next(&mut page.csrs),
                Tab::Console => Focusable::focus_next(&mut page.console),
            },
            Focused::Tabs => Focusable::focus_next(&mut page.tabs),
        };
        page
    }

    fn help(&self) -> Table<'_> {
        let mut tips = vec![("Tab", "Focus next"), ("Shift+Tab", "Focus previous")];
        match self.focused {
            Focused::Content => match self.tab {
                Tab::Dashboard => tips.extend(self.dashboard.help()),
                Tab::CSRs => tips.extend(self.csrs.help()),
                Tab::Console => tips.extend(self.console.help()),
            },
            Focused::Tabs => tips.extend(self.tabs.help()),
        }

        let row = Row::new(tips.iter().map(|(keys, description)| {
            Cell::new(Line::from(vec![
                Span::from(*keys).cyan(),
                Span::from(" "),
                Span::from(*description),
            ]))
        }));
        let widths = tips
            .iter()
            .map(|(key, desc)| key.len() + desc.len() + 1)
            .map(|len| Constraint::Length(len as _));

        const SPACING: u16 = 5;
        Table::new([row], widths)
            .block(Block::default().borders(Borders::TOP))
            .column_spacing(SPACING)
    }
}

impl<'c> Page<&'c mut Cpu> for ExecutionPage<'c> {
    fn close(self) -> &'c mut Cpu {
        self.cpu
    }
}

impl Widget for &mut ExecutionPage<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [tabs, content, help] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ])
        .areas(area);

        Tabs::new(Tab::titles())
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Color::default()),
            )
            .highlight_style(
                match self.focused {
                    Focused::Tabs => Style::default().light_blue(),
                    Focused::Content => Style::default().white(),
                }
                .bold(),
            )
            .select(self.tab.index())
            .divider(DOT)
            .dark_gray()
            .render(tabs, buf);

        match self.tab {
            Tab::Dashboard => self.dashboard.render(&self.model, content, buf),
            Tab::CSRs => self.csrs.render(self.model.csr_file(), content, buf),
            Tab::Console => self.console.render(self.cpu.os(), content, buf),
        }

        Widget::render(self.help(), help, buf);
    }
}

impl EventHandler<()> for ExecutionPage<'_> {
    type EventResult = Option<ExitCode>;

    fn handle_event(&mut self, event: Event, _: ()) -> EventResult<Self::EventResult> {
        match self.focused {
            Focused::Content => match self.tab {
                Tab::Dashboard => {
                    let payload = EventPayload {
                        cpu: self.cpu,
                        model: &mut self.model,
                    };
                    self.dashboard.handle_event(event, payload)
                }
                Tab::CSRs => self.csrs.handle_event(event, ()).map(|_| None),
                Tab::Console => self
                    .console
                    .handle_event(event, self.cpu.os())
                    .map(|_| None),
            },
            Focused::Tabs => self.tabs.handle_event(event, ()).map(|tab| {
                self.tab = tab;
                None
            }),
        }
    }
}

pub struct EventPayload<'c, 'm> {
    cpu: &'c mut Cpu,
    model: &'m mut CpuModel,
}

impl FocusHandler for ExecutionPage<'_> {
    fn focused(&mut self) -> Option<&mut dyn Focusable> {
        Some(match self.focused {
            Focused::Content => match self.tab {
                Tab::Dashboard => &mut self.dashboard,
                Tab::CSRs => &mut self.csrs,
                Tab::Console => &mut self.console,
            },
            Focused::Tabs => &mut self.tabs,
        })
    }

    fn focus_next(&mut self) {
        self.focused = self.focused.next();
    }

    fn focus_prev(&mut self) {
        self.focused = self.focused.prev();
    }
}
