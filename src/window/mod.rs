mod clock;
mod type_keyboard;
mod mouse;
mod press_keyboard;

use std::collections::VecDeque;

use minifb as mfb;

use crate::{shared::*, screen::{PixelFB, Screen, Zel, DirtyRegion}, input::{Event, Input}};

use self::{type_keyboard::Keyboard, mouse::Mouse, clock::Clock, press_keyboard::PressKeyboard};

pub struct Window {
    // minifb state
    title: String,
    screen: Screen,

    // chiropterm state
    window: Option<mfb::Window>,
    fb: PixelFB, 
    input: Input, postponed_event: Option<Event>,
    dirty_region: DirtyRegion,

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
    pub fn new(title: String, size: impl ToZelSize, bg: impl ToColor, fg: impl ToColor) -> Self {
        let mut win = Window {
            title,

            window: None,

            fb: PixelFB::new(), screen: Screen::new(size, bg, fg),
            input: Input::new(), postponed_event: None,
            dirty_region: DirtyRegion::new(),

            clock: Clock::new(),
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
            simple_keyboard: PressKeyboard::new(),

            input_events: VecDeque::new(),
        };
        win.dirty_region.saturate();
        win
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
            self.mouse.update(self.screen.size, win, is_new_tick, |xy| self.screen.view(xy));

            while let Some(mouse_event) = self.mouse.pop_event() {
                self.input_events.push_back(Event::Mouse(mouse_event))
            }

            // and finally from clock 
            while let Some(clock_event) = self.clock.pop_event() {
                self.input_events.push_back(Event::Tick(clock_event));
            }

            let win = self.window.as_mut().unwrap();

            // physically redraw if needed
            let mouse_sel_changed = self.mouse.selection_changed();  
            let is_dirty = self.dirty_region.is_dirty();
            let needs_physical_redraw = is_dirty || mouse_sel_changed;

            if needs_physical_redraw {
                // == figure out what kind of redraw: mouse sel has different implications from normal dirty ==
                let touched = if is_dirty && !mouse_sel_changed {
                    self.fb.draw(&self.screen, self.mouse.click_selection(), self.dirty_region.dirty_cells())
                } else {
                    self.fb.draw(&self.screen, self.mouse.click_selection(), None)
                };

                if touched {
                    let (buf, sz) = self.fb.view_buffer();
                    win.update_with_buffer(buf, sz.width as usize, sz.height as usize).unwrap();
                } else {
                    win.update()
                }
                self.dirty_region.reset();
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
    fn affordance(&mut self) -> Affordance { self.screen.affordance() }

    fn raw_view(&self, zp: ZelPointI) -> Zel { self.screen.raw_view(zp) }

    fn raw_touch(&mut self, zp: ZelPointI, format: bool, cb: impl FnOnce(&mut Zel)) {
        self.screen.raw_touch(zp, format, |zel| {
            cb(zel);
            self.dirty_region.record(zp)
        });
    }

    fn clear(&mut self) {
        self.screen.clear();
        self.dirty_region.saturate();
    }

    fn bounds(&mut self) -> ZelRectI {
        self.screen.bounds()
    }

    fn get_font(&self) -> crate::Font {
        self.screen.get_font()
    }
}