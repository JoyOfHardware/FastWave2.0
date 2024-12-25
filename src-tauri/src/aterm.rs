use std::result;
use std::sync::{mpsc, Arc};

use alacritty_terminal::event::{Event, EventListener};
use alacritty_terminal::event_loop::{EventLoop, Notifier};
use alacritty_terminal::sync::FairMutex;
use alacritty_terminal::term::{self, Term};
use alacritty_terminal::term::cell::Cell;
use alacritty_terminal::{tty, Grid};
use tauri::Emitter;
use shared::term::{TerminalDownMsg, TerminalScreen};

use crate::terminal_size;

#[derive(Clone)]
pub struct EventProxy(mpsc::Sender<Event>);
impl EventListener for EventProxy {
    fn send_event(&self, event: Event) {
        let _ = self.0.send(event.clone());
    }
}

pub struct ATerm {
    pub term: Arc<FairMutex<Term<EventProxy>>>,

    rows : u16,
    cols : u16,

    /// Use tx to write things to terminal instance from outside world
    pub tx: Notifier,

    /// Use rx to read things from terminal instance.
    /// Rx only has data when terminal state has changed,
    /// otherwise, `std::sync::mpsc::recv` will block and sleep
    /// until there is data.
    pub rx: mpsc::Receiver<(u64, Event)>,
}

impl ATerm {
    pub fn new() -> result::Result<ATerm, std::io::Error> {
        let (rows, cols) = (21, 90);
        let id = 1;
        let pty_config = tty::Options {
            shell: Some(tty::Shell::new("/bin/bash".to_string(), vec![])),
            ..tty::Options::default()
        };
        let config = term::Config::default();
        let terminal_size = terminal_size::TerminalSize::new(rows, cols);
        let pty = tty::new(&pty_config, terminal_size.into(), id)?;
        let (event_sender, event_receiver) = mpsc::channel();
        let event_proxy = EventProxy(event_sender);
        let term = Term::new::<terminal_size::TerminalSize>(
            config,
            &terminal_size.into(),
            event_proxy.clone(),
        );
        let term = Arc::new(FairMutex::new(term));
        let pty_event_loop = EventLoop::new(term.clone(), event_proxy, pty, false, false)?;
        let notifier = Notifier(pty_event_loop.channel());
        let (pty_proxy_sender, pty_proxy_receiver) = std::sync::mpsc::channel();
        // Start pty event loop
        pty_event_loop.spawn();
        std::thread::Builder::new()
            .name(format!("pty_event_subscription_{}", id))
            .spawn(move || loop {
                if let Ok(event) = event_receiver.recv() {
                    if let Event::Exit = event {
                        break;
                    }
                    else {
                        if let Some(app_handle) = crate::APP_HANDLE.read().unwrap().clone() {
                            let term = crate::TERM.lock().unwrap();
                            let content = terminal_instance_to_string(&term);
                            let payload = TerminalScreen {
                                cols: term.cols,
                                rows: term.rows,
                                content: content
                            };
                            let payload = TerminalDownMsg::FullTermUpdate(payload);
                            let payload = serde_json::json!(payload);
                            app_handle.emit("term_content", payload).unwrap();
                        }
                    }
                }
            })?;
        Ok(ATerm {
            term,
            rows,
            cols,
            tx: notifier,
            rx: pty_proxy_receiver,
        })
    }
}

fn terminal_instance_to_string(terminal_instance: &ATerm) -> String {
    let (rows, cols) = (terminal_instance.rows, terminal_instance.cols);
    let term = terminal_instance.term.lock();
    let grid = term.grid().clone();

    return term_grid_to_string(&grid, rows, cols);
}

fn term_grid_to_string(grid: &Grid<Cell>, rows: u16, cols: u16) -> String {
    let mut term_content = String::with_capacity((rows*cols) as usize);

    // Populate string from grid
    for indexed in grid.display_iter() {
        let x = indexed.point.column.0 as usize;
        let y = indexed.point.line.0 as usize;
        if y < rows as usize && x < cols as usize {
            term_content.push(indexed.c);
        }
    }
    return term_content;
}
