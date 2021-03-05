use crate::Terminal;
use termion::event::Key;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    pub fn default() -> Self {
        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
         }
    }

    fn refresh_screen(&self) -> std::io::Result<()> {
        Terminal::clear_screen();
        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0)          
        }
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> std::io::Result<()> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!(e);
}