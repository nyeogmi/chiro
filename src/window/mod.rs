mod clock;
mod keyboard;
mod mouse;
mod simple_keyboard;

use std::collections::VecDeque;

use minifb as mfb;

use crate::{aliases::*, color::Color, screen::{PixelFB, Screen}, input::{Event, Input}};

use self::{keyboard::Keyboard, mouse::Mouse, clock::Clock, simple_keyboard::SimpleKeyboard};

macro_rules! handle_resume {
    ( $l:tt, $x:expr ) => {
        // check events
        match $x { 
            Resume::NotYet => {},
            Resume::Exit => { break $l; }
        }
    }
}

pub struct Window {
    // minifb state
    title: String,
    screen: Screen,

    // chiropterm state
    window: Option<mfb::Window>,
    fb: PixelFB, 
    input: Input,

    // drivers
    clock: Clock,
    keyboard: Keyboard,
    mouse: Mouse,
    simple_keyboard: SimpleKeyboard,

    // event loop state
    input_events: VecDeque<Event>,
}

struct EventLoop<'a> {
    on_input: Box<dyn 'a+FnMut(&mut Window, Event) -> Resume>,
    on_exit: Box<dyn 'a+FnMut(&mut Window) -> Resume>,
}


const HANDLE_INPUT_EVERY: usize = 4166; // 240 FPS

impl Window {
    pub fn new(title: String, size: ZelSize, bg: Color, fg: Color) -> Self {
        Window {
            title,

            window: None,

            fb: PixelFB::new(), screen: Screen::new(size, bg, fg),
            input: Input::new(),

            clock: Clock::new(),
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
            simple_keyboard: SimpleKeyboard::new(),

            input_events: VecDeque::new(),
        }
    }

    fn reconstitute_mfb_window(&mut self) {
        if let Some(_) = &mut self.window { return; }

        let mut opts = mfb::WindowOptions::default();
        opts.scale = mfb::Scale::FitScreen;
        opts.scale_mode = mfb::ScaleMode::Stretch;
        opts.resize = true;

        let wsz = self.screen.size.to_pixels();
        
        let mut window = mfb::Window::new(
            &self.title,
            wsz.width as usize, wsz.height as usize,
            opts
        ).unwrap_or_else(|e| {
            panic!("{}", e) // TODO: Handle?
        });
        let bg = self.screen.bg.to_rgb();
        window.limit_update_rate(Some(std::time::Duration::from_micros(HANDLE_INPUT_EVERY as u64))); 
        window.set_background_color(bg.0 as usize, bg.1 as usize, bg.2 as usize);

        self.keyboard.add_hooks(&mut window);
        self.window = Some(window);
    }

    pub fn screen(&mut self) -> &mut Screen {
        &mut self.screen
    }

    pub fn getch(&mut self) -> char {
        self.run_loop(EventLoop { 
            on_input: Box::new(|_, _| Resume::NotYet),
            on_exit: Box::new(|_| Resume::Exit) 
        });
        '?'
    }

    fn run_loop<'a>(&mut self, mut evt: EventLoop<'a>) {
        let mut first_tick = true;

        'main: loop {
            // make sure the window exists
            self.reconstitute_mfb_window();
            let win = self.window.as_mut().expect("just reconstituted it");

            if !win.is_open() {
                self.window = None;
                handle_resume!('main, (evt.on_exit)(self));
                continue;
            }

            // now update window 
            let win = self.window.as_mut().unwrap();

            // from clock
            let mut is_new_tick = false;

            self.clock.update();
            if let Some(_) = self.clock.peek_event() { is_new_tick = true; }

            // from keyboard
            self.keyboard.update(win);
            while let Some(key_event) = self.keyboard.pop_event() {
                self.input_events.push_back(Event::Keyboard(key_event))
            }

            // from simple keyboard
            self.simple_keyboard.update(win);
            while let Some(key_event) = self.simple_keyboard.pop_event() {
                self.input_events.push_back(Event::SimpleKeyboard(key_event))
            }

            // and from mouse
            self.mouse.update(self.screen.size, win, is_new_tick, |xy| self.screen.get(xy.x, xy.y).affordance);

            while let Some(mouse_event) = self.mouse.pop_event() {
                self.input_events.push_back(Event::Mouse(mouse_event))
            }

            // and finally from clock 
            while let Some(clock_event) = self.clock.pop_event() {
                self.input_events.push_back(Event::Tick(clock_event));
                is_new_tick = true;
            }


            // physically redraw if needed
            let needs_physical_redraw = (first_tick && is_new_tick) || self.mouse.selection_changed();  // TODO
            if needs_physical_redraw {
                first_tick = false;
                let touched = self.fb.draw(self.screen.clone(), self.mouse.selection());
                if touched {
                    let (buf, sz) = self.fb.view_buffer();
                    win.update_with_buffer(buf, sz.width as usize, sz.height as usize).unwrap();
                } else {
                    win.update()
                }
            } else {
                win.update()
            }

            while let Some(i_evt) = self.input_events.pop_front() {
                self.input.on_event(i_evt);
                handle_resume!('main, (evt.on_input)(self, i_evt))
            }
        }

        if let Some(win) = self.window.as_mut() {
            win.update()
        }
    }
}

enum Resume {
    NotYet,
    Exit,
}
