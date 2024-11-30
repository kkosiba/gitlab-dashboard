use crossterm::event::KeyEvent;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::action::Action;
use crate::components::loading_component::LoadingComponent;
use crate::components::Component;
use crate::config::Config;
use crate::tui::{Event, Tui};
use color_eyre::Result;

pub struct App {
    config: Config,
    components: Vec<Box<dyn Component>>,
    should_quit: bool,
    mode: Mode,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        Ok(Self {
            config,
            components: vec![Box::new(LoadingComponent::new())],
            should_quit: false,
            mode: Mode::Home,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
        })
    }

    // TODO: to be carried over to the new style
    //fn next(&mut self) {
    //    match &self.state.pipelines_data {
    //        PipelinesData::Loading | PipelinesData::Errors(_) => {}
    //        PipelinesData::Loaded(pipelines) => {
    //            if self.state.active_operation_index < pipelines.len() - 1 {
    //                self.state.active_operation_index += 1;
    //            }
    //        }
    //    }
    //}
    //
    //fn previous(&mut self) {
    //    match &self.state.pipelines_data {
    //        PipelinesData::Loading | PipelinesData::Errors(_) => {}
    //        PipelinesData::Loaded(_) => {
    //            if self.state.active_operation_index > 0 {
    //                self.state.active_operation_index -= 1;
    //            }
    //        }
    //    }
    //}

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

    // TODO: migrate this to new style
    //fn handle_event(&mut self) -> Result<bool> {
    //    // TODO: This method has grown a bit already, consider refactoring it and maybe even moving
    //    // event handling to a separate module
    //    if event::poll(Duration::from_millis(100))? {
    //        if let Event::Key(key) = event::read()? {
    //            if self.state.render_project_selector {
    //                match key.code {
    //                    KeyCode::Char('q') => return Ok(true),
    //                    KeyCode::Char('j') | KeyCode::Down => {
    //                        let projects = &self.config.core.gitlab_projects;
    //                        if self.state.active_operation_index < projects.len() - 1 {
    //                            self.state.active_operation_index += 1;
    //                        }
    //                    }
    //                    KeyCode::Char('k') | KeyCode::Up => {
    //                        if self.state.active_operation_index > 0 {
    //                            self.state.active_operation_index -= 1;
    //                        }
    //                    }
    //                    KeyCode::Enter => {
    //                        let projects = &self.config.core.gitlab_projects;
    //                        self.state.active_project =
    //                            Some(projects[self.state.active_operation_index].clone());
    //                        self.state.render_project_selector = false;
    //                        // Reset index for pipelines view
    //                        self.state.active_operation_index = 0;
    //                    }
    //                    _ => {}
    //                }
    //            } else {
    //                match key.code {
    //                    KeyCode::Char('q') => return Ok(true),
    //                    KeyCode::Char('j') | KeyCode::Down => self.next(),
    //                    KeyCode::Char('k') | KeyCode::Up => self.previous(),
    //                    _ => {}
    //                }
    //            }
    //        }
    //    }
    //    Ok(false)
    //}

    // TODO: this is now render method below, need to move functionality across
    //fn draw(&self, f: &mut Frame) {
    //    if self.state.render_project_selector {
    //        let projects = &self.config.core.gitlab_projects;
    //        render_project_selector(f, &self.state, projects);
    //    } else {
    //        render_pipelines_view(f, &self.state);
    //    }
    //}

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
