//declare submodules
//app.rs is the logic and ui.rs handles rendering of the app

mod app;
mod ui;

//import applications state 'app' and 'question' struct from app module
use app::App;

//crossterm provides terminal control and input handling on windows/macos/linux
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

//Ratatui is used to render the terminal UI widgets
use ratatui::{Terminal, backend::CrosstermBackend};

use std::io::{self, stdout}; //used for interacting with terminal output

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //enable raw mode to bypass line buffering and directly capture key input
    //real-time interactivity rather than waiting for the enter key
    terminal::enable_raw_mode()?;

    //switch to alternate terminal screen so we don mes up the user UI <---- CHANGE THIS LATER
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    //crossterm backend acts as teh engine for drawing in the terminal
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //instatiate the app state (loads the questions and sets defaults)
    let mut app = App::default();

    // main UI loop
    loop {
        //draw the current fream by calling our UI rendering function
        terminal.draw(|f| ui::draw_ui(f, &app))?;

        //poll the keyboard events, with a 200ms wait for efficiency
        if event::poll(std::time::Duration::from_millis(200))? {
            //read next event from the terminal input
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => app.next_option(), //down arrow - move to next choice
                    KeyCode::Up => app.previous_option(), //up arrow - move to previous choice
                    KeyCode::Enter => app.check_answer(), //enter key - mark selected answer
                    _ => {}                             //ignore other keys
                }
            }
        }
    }
    //cleanup
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal::disable_raw_mode()?;

    Ok(())
}
