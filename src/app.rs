use std::{io, thread};

use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use ratatui::{prelude::*, widgets::*};
use termion::{event::*, input::TermRead};

use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;

use crate::viewer::{dlg_goto::GotoType, dlg_hex_search::HexSearch, dlg_text_search::TextSearch};

#[derive(Debug, Clone)]
pub enum Events {
    Input(Event),
    Signal(i32),
}

#[derive(Debug, Clone)]
pub enum PubSub {
    // App-wide events
    Error(String),
    Warning(String, String),
    Info(String, String),
    CloseDialog,

    // File viewer events
    FileInfo(String, String, String),
    ToggleHex,

    // Text viewer events
    Highlight(Vec<Vec<(Style, String)>>),

    // Hex viewer events
    FromHexOffset(u64),
    ToHexOffset(u64),
    HVStartSearch,
    HVSearchNext,
    HVSearchPrev,

    // Dialog goto events
    DlgGoto(GotoType),
    Goto(GotoType, String),

    // Dialog text search events
    DlgTextSearch(TextSearch),
    TextSearch(TextSearch),

    // Dialog hex search events
    DlgHexSearch(HexSearch),
    HexSearch(HexSearch),
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Continue,
    Redraw,
    Quit,
    CtrlC,
    SigTerm,
    CtrlZ,
    SigCont,
}

pub trait App {
    fn handle_events(&mut self) -> Result<Action>;
    fn render(&mut self, f: &mut Frame);
}

pub fn init_events() -> Result<(Sender<Events>, Receiver<Events>)> {
    let (tx, rx) = crossbeam_channel::unbounded();
    let input_tx = tx.clone();
    let signals_tx = tx.clone();

    thread::spawn(move || {
        let stdin = io::stdin();
        for event in stdin.events().flatten() {
            if let Err(err) = input_tx.send(Events::Input(event)) {
                eprintln!("{}", err);
                return;
            }
        }
    });

    let mut signals = Signals::new([SIGWINCH, SIGINT, SIGTERM, SIGCONT])?;

    thread::spawn(move || {
        for signal in &mut signals {
            if let Err(err) = signals_tx.send(Events::Signal(signal)) {
                eprintln!("{}", err);
                return;
            }
        }
    });

    Ok((tx, rx))
}

pub fn centered_rect(width: u16, height: u16, r: &Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height.saturating_sub(height) + 1) / 2),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(*r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((r.width.saturating_sub(width) + 1) / 2),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1]
}

pub fn render_shadow(f: &mut Frame, r: &Rect, s: &Style) {
    let area1 = Rect::new(r.x + 2, r.y + r.height, r.width, 1).intersection(f.size());
    let area2 =
        Rect::new(r.x + r.width, r.y + 1, 2, r.height.saturating_sub(1)).intersection(f.size());

    let block = Block::default().style(*s);

    f.render_widget(block.clone(), area1);
    f.render_widget(block, area2);
}
