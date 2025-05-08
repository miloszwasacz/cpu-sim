use super::scrolling::{Scrolling, ScrollingModel};
use super::{EXCEPTION_COLOR, NOT_READY_COLOR, READY_COLOR};
use crate::ui::model::RobModel;
use crate::ui::StatefulComponent;

use cpu_sim::components::diagnostics::cpu::{
    RobEntrySnapshot, RobIndex, RobNotReady as NotReady, RobReady as Ready,
};
use cpu_sim::components::diagnostics::fmt_addr;
use cpu_sim::instr::{Ebreak, Ecall, EnvTrap};
use ratatui::prelude::*;

const MISPREDICTED_COLOR: Color = Color::LightMagenta;
const SYSCALL_COLOR: Color = Color::LightBlue;
const BREAK_COLOR: Color = Color::Cyan;

pub type Rob = Scrolling<RobModel>;

impl Rob {
    pub const WIDTH: Constraint = Constraint::Length(45);
}

impl StatefulComponent for Rob {
    type Model = RobModel;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        self.render_scrolling("Reorder Buffer", model, area, buf, |entries| {
            entries.map(|(i, entry)| {
                let mut line = Line::from(format!("{:2}: ", i));
                match entry {
                    RobEntrySnapshot::Empty => {}
                    RobEntrySnapshot::NotReady(entry) => {
                        let text = match entry.data {
                            NotReady::Alu { dest } => format!("ALU    {:#}", dest),
                            NotReady::Jump {
                                link_reg,
                                link_data,
                                ..
                            } => format!("JUMP   {:#}, {}", link_reg, fmt_addr(link_data.addr()),),
                            NotReady::Branch { target, .. } => {
                                format!("BRANCH                {}", fmt_addr(target))
                            }
                            NotReady::Load { dest } => format!("LOAD   {:#}", dest),
                            NotReady::Store { src, .. } => format!("STORE  {:#}", src),
                        };
                        line += Span::from(format!("{}: {}", fmt_addr(entry.addr), text))
                            .fg(NOT_READY_COLOR);
                    }
                    RobEntrySnapshot::Ready(entry) => {
                        let (text, color) = match entry.data {
                            Ok(ready) => match ready {
                                Ready::Alu { dest, value } => {
                                    (format!("ALU    {:#}, {}", dest, value.i()), READY_COLOR)
                                }
                                Ready::Jump {
                                    predicted,
                                    link_reg,
                                    link_data,
                                    target,
                                } => (
                                    format!(
                                        "JUMP   {:#}, {}, {}",
                                        link_reg,
                                        fmt_addr(link_data.addr()),
                                        fmt_addr(target)
                                    ),
                                    if predicted == target {
                                        READY_COLOR
                                    } else {
                                        MISPREDICTED_COLOR
                                    },
                                ),
                                Ready::Branch {
                                    predicted,
                                    target,
                                    taken,
                                    pc_plus_4: _,
                                } => {
                                    let color = if predicted == taken {
                                        READY_COLOR
                                    } else {
                                        MISPREDICTED_COLOR
                                    };
                                    line += Span::from(format!(
                                        "{}: BRANCH                ",
                                        fmt_addr(entry.addr)
                                    ))
                                    .fg(color);
                                    let mut target = Span::from(fmt_addr(target)).fg(color);
                                    if !taken {
                                        target = target.crossed_out().dim();
                                    }
                                    line += target;
                                    return line;
                                }
                                Ready::Load { dest, value } => {
                                    (format!("LOAD   {:#}, {}", dest, value.i()), READY_COLOR)
                                }
                                Ready::Store { src, addr, .. } => {
                                    (format!("STORE  {:#}, {}", src, fmt_addr(addr)), READY_COLOR)
                                }
                                Ready::Fence => ("FENCE".to_string(), READY_COLOR),
                            },
                            Err(trap) => match trap {
                                EnvTrap::Syscall => {
                                    (Ecall::DISPLAY_NAME.to_ascii_uppercase(), SYSCALL_COLOR)
                                }
                                EnvTrap::Break => {
                                    (Ebreak::DISPLAY_NAME.to_ascii_uppercase(), BREAK_COLOR)
                                }
                                EnvTrap::Exception(_) => ("Exception".to_string(), EXCEPTION_COLOR),
                            },
                        };
                        line += Span::from(format!("{}: {}", fmt_addr(entry.addr), text)).fg(color);
                    }
                }
                line
            })
        })
    }
}

impl ScrollingModel for RobModel {
    type Entry = (RobIndex, RobEntrySnapshot);

    #[inline(always)]
    fn entries(&self) -> &[Self::Entry] {
        self.entries()
    }
}
