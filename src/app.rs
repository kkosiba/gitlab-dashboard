use crossterm::event::KeyEvent;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::action::Action;
use crate::components::header_component::HeaderComponent;
use crate::components::project_selector_component::ProjectSelectorComponent;
use crate::components::Component;
use crate::config::Config;
use crate::state::State;
use crate::tui::{Event, Tui};
use color_eyre::Result;

pub struct App {
    config: Config,
    header: HeaderComponent,
    components: Vec<Box<dyn Component>>,
    should_quit: bool,
    mode: Mode,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
    state: State,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        let state = State::default();
        Ok(Self {
            config,
            components: vec![Box::new(ProjectSelectorComponent::new())],
            header: HeaderComponent::new(),
            // Pre-define all components to use
            components: vec![
                Box::new(ProjectSelectorComponent::new()),
                Box::new(LoadingComponent::new()),
                Box::new(PipelinesViewerComponent::new()),
            ],
            should_quit: false,
            mode: Mode::Home,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
            state,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;
        // .mouse(true) // uncomment this line to enable mouse support
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(self.action_tx.clone())?;
        }
        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }
        for component in self.components.iter_mut() {
            component.init(tui.size()?)?;
        }

        self.header.init(&self.state)?;
        loop {
            self.handle_events(&mut tui).await?;
            self.handle_actions(&mut tui)?;
            if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };
        let action_tx = self.action_tx.clone();
        match event {
            Event::Quit => action_tx.send(Action::Quit)?,
            Event::Tick => action_tx.send(Action::Tick)?,
            Event::Render => action_tx.send(Action::Render)?,
            Event::Key(key) => self.handle_key_event(key)?,
            _ => {}
        }
        for component in self.components.iter_mut() {
            if let Some(action) = component.handle_events(Some(event.clone()))? {
                action_tx.send(action)?;
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();
        let Some(keymap) = self.config.keybindings.get(&self.mode) else {
            return Ok(());
        };
        match keymap.get(&vec![key]) {
            Some(action) => {
                action_tx.send(action.clone())?;
            }
            _ => {
                // If the key was not handled as a single key action,
                // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // Check for multi-key combinations
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                    action_tx.send(action.clone())?;
                }
            }
        }
        Ok(())
    }

    fn handle_actions(&mut self, tui: &mut Tui) -> Result<()> {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                // TODO: Find a way to update App.components through component action. The code
                // below doesn't work.
                //Action::ReplaceComponents(new_components) => {
                //    self.components = new_components; // Replace the components
                //
                //    // Initialize the new components
                //    for component in self.components.iter_mut() {
                //        component.register_action_handler(self.action_tx.clone())?;
                //        component.register_config_handler(self.config.clone())?;
                //        component.init(tui.size()?)?;
                //    }
                //}
                Action::Tick => {
                    self.last_tick_key_events.drain(..);
                }
                Action::Quit => self.should_quit = true,
                Action::Render => self.render(tui)?,
                _ => {}
            }
            for component in self.components.iter_mut() {
                if let Some(action) = component.update(action.clone())? {
                    self.action_tx.send(action)?
                };
            }
        }
        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            for component in self.components.iter_mut() {
                if let Err(err) = component.draw(frame, frame.area()) {
                    let _ = self
                        .action_tx
                        .send(Action::Error(format!("Failed to draw: {:?}", err)));
                }
            }
        })?;
        Ok(())
    }
}
