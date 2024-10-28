use std::fs::{read_to_string, read_dir};
use std::io::{self, Error};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use crate::ui;

enum FilePath {
}

impl FilePath {
    pub const WISHES: &str = "./wishes";
    pub const ART: &str = "./art";
}

#[derive(Debug, Default)]
pub struct App {
    wishes: Vec<String>,
    art: String,
    art_length: usize,
    log: Vec<String>,
    current_wishes: usize,
    current_dialog: usize,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        let wishes = get_files_in_folder(FilePath::WISHES).unwrap();
        let log = read_file_to_vec(FilePath::WISHES,wishes.get(0).unwrap()).unwrap();
        let art = read_file_to_art(FilePath::ART, wishes.get(0).unwrap());
        let art_length = get_art_length(FilePath::ART, wishes.get(0).unwrap());

        Self {
            wishes,
            log,
            art,
            art_length,
            current_wishes: 0,
            current_dialog: 0,
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| ui::ui(frame, &self))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_current_dialog(&mut self) {
        if self.log.len() - 1 > self.current_dialog {
            self.current_dialog += 1;
        } else if self.wishes.len() - 1 > self.current_wishes {
            self.current_dialog = 0;
            self.current_wishes += 1;
            self.log = read_file_to_vec(FilePath::WISHES, self.get_current_wishes()).unwrap();
            self.art = read_file_to_art(FilePath::ART, self.get_current_wishes());
            self.art_length = get_art_length(FilePath::ART, self.get_current_wishes());
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Right => self.increment_current_dialog(),
            _ => {}
        }
    }

    pub fn get_current_wishes(&self) -> &str {
        self.wishes.get(self.current_wishes).unwrap()
    }

    pub fn get_current_dialog(&self) -> &str {
        self.log.get(self.current_dialog).unwrap()
    }

    pub fn get_art(&self) -> &str {
        &self.art
    }

    pub fn get_art_length(&self) -> usize {
        self.art_length
    }
}

fn read_file_to_art(file_path: &str, file_name: &str) -> String {
    let file_extension = ".txt";
    let path = format!("{}/{}{}", file_path, file_name, file_extension);
    let content = read_to_string(path);

    match content {
        Ok(content) => content,
        Err(_) => "".to_string()
    }
}

fn get_art_length(file_path: &str, file_name: &str) -> usize {
    // Read the entire file content as a single String
    let file_extension: &str = ".txt";
    let path = format!("{}/{}{}", file_path, file_name, file_extension);
    let content = read_to_string(path);

    match content {
        Ok(content) => {
            let max_length = content
                .lines()
                .map(|line| line.len())          // Map each line to its length
                .max()                           // Find the maximum length
                .unwrap_or(0);                   // Default to 0 if the file is empty
            return max_length;
        },
        Err(_) => {
            return 0;
        }
    }
}

fn read_file_to_vec(file_path: &str, file_name: &str) -> Result<Vec<String>, Error> {
    // Read the entire file content as a single String
    let file_extension: &str = ".txt";
    let path = format!("{}/{}{}", file_path, file_name, file_extension);
    let content = read_to_string(path)?;
    
    // Split the content into lines and collect them into a Vec<String>
    let lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();
    
    Ok(lines)
}

fn get_files_in_folder(folder: &str) -> Result<Vec<String>, Error> {
    let mut files = Vec::new();

        // Iterate through each entry in the directory
    for entry in read_dir(folder).unwrap() {

        // Unwrap the entry
        let entry = entry.unwrap();
        let path = entry.path();

        // Check if it's a file with a .txt extension
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("txt") {
            files.push(path.file_stem().unwrap().to_string_lossy().into_owned());
        }
    }
    
    Ok(files)
}