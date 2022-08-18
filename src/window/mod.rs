mod clock;
mod type_keyboard;
mod mouse;
mod press_keyboard;

use std::collections::VecDeque;

use minifb as mfb;

use crate::{aliases::*, color::Color, screen::{PixelFB, Screen, Zel}, input::{Event, Input}};

use self::{type_keyboard::Keyboard, mouse::Mouse, clock::Clock, press_keyboard::PressKeyboard};

pub struct Window {
    // minifb state
    title: String,
    screen: Screen,

    // chiropterm state
    window: Option<mfb::Window>,
    fb: PixelFB, 
    input: Input, postponed_event: Option<Event>,
    touched: bool,

    // drivers
    clock: Clock,
    keyboard: Keyboard,
    mouse: Mouse,
    simple_keyboard: PressKeyboard,

    // event loop state
    input_events: VecDeque<Event>,
}


const HANDLE_INPUT_EVERY: usize = 4166; // 240 FPS

impl Window {
    pub fn new(title: String, size: ZelSize, bg: Color, fg: Color) -> Self {
        Window {
            title,

            window: None,

            fb: PixelFB::new(), screen: Screen::new(size, bg, fg),
            input: Input::new(), postponed_event: None,
            touched: true,

            clock: Clock::new(),
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
            simple_keyboard: PressKeyboard::new(),

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
}


impl Eventable for Window {
    fn next_event<'a>(&mut self) -> Event {
        if let Some(postponed) = self.postponed_event.take() { self.input.on_event(postponed) }

        let evt = 'main: loop {
            // make sure all events that are waiting around have been dealt with
            if let Some(i_evt) = self.input_events.pop_front() {
                if let Event::Tick(_) = &i_evt {
                    // leave input in the immediately pre-event state so it will be at the right state for the end of this tick
                    self.postponed_event = Some(i_evt) 
                } else {
                    self.input.on_event(i_evt)
                }
                break 'main i_evt
            }

            // make sure the window exists
            self.reconstitute_mfb_window();
            let win = self.window.as_mut().expect("just reconstituted it");

            if !win.is_open() {
                self.window = None;
                break Event::Exit;
            }

            // now make sure window is up to date
            let win = self.window.as_mut().unwrap();
            win.update();

            // now find events
            // from clock
            let mut is_new_tick = false;

            self.clock.update();
            if let Some(_) = self.clock.peek_event() { is_new_tick = true; }

            // from keyboard
            self.keyboard.update(win);
            while let Some(key_event) = self.keyboard.pop_event() {
                self.input_events.push_back(Event::Type(key_event))
            }

            // from simple keyboard
            self.simple_keyboard.update(win);
            while let Some(key_event) = self.simple_keyboard.pop_event() {
                self.input_events.push_back(Event::Press(key_event))
            }

            // and from mouse
            self.mouse.update(self.screen.size, win, is_new_tick, |xy| self.screen.view(xy).affordance);

            while let Some(mouse_event) = self.mouse.pop_event() {
                self.input_events.push_back(Event::Mouse(mouse_event))
            }

            // and finally from clock 
            while let Some(clock_event) = self.clock.pop_event() {
                self.input_events.push_back(Event::Tick(clock_event));
            }

            let win = self.window.as_mut().unwrap();

            // physically redraw if needed
            let needs_physical_redraw = self.touched || self.mouse.selection_changed();  // TODO
            if needs_physical_redraw {
                let touched = self.fb.draw(self.screen.clone(), self.mouse.selection());
                if touched {
                    let (buf, sz) = self.fb.view_buffer();
                    win.update_with_buffer(buf, sz.width as usize, sz.height as usize).unwrap();
                } else {
                    win.update()
                }
                self.touched = false;
            } else {
                win.update()
            }
        };

        evt
    }

    fn input(&self) -> &Input {
        &self.input
    }
}

impl Drawable for Window {
    fn raw_view(&self, zp: ZelPointI) -> Zel {
        self.screen.raw_view(zp)
    }

    fn raw_at(&mut self, zp: ZelPointI) -> Option<&mut Zel> {
        self.touched = true;
        self.screen.raw_at(zp)
    }

    fn clear(&mut self) {
        self.touched = true;
        self.screen.clear()
    }
}