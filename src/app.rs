use chrono::NaiveDateTime;
use crossterm::event::KeyCode;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::client::{
    audit::AuditInfo,
    channel::{ChannelInfo, ChannelState},
    node::NodeInfo,
    Client,
};

pub type AppMutex = Arc<Mutex<App>>;

pub struct App {
    pub client: Client,
    pub db: sled::Db,

    pub tabs: Vec<String>,
    pub tab_index: usize,

    pub errors: Vec<String>,

    pub node_info: NodeInfo,
    pub active_chans: usize,
    pub pending_chans: usize,
    pub sleeping_chans: usize,

    pub screen_width: u16,
    pub relays_amounts_line: Vec<u64>,
    pub relays_volumes_line: Vec<u64>,

    pub channels: Vec<ChannelInfo>,
    pub audit: AuditInfo,
}

impl App {
    pub async fn new(client: Client, db: sled::Db) -> Result<App, Box<dyn Error>> {
        let node_info = client.get_info().await?;

        Ok(App {
            client,
            db,
            tabs: vec![
                "Dashboard".to_owned(),
                "Peers".to_owned(),
                "Onchain".to_owned(),
                "Routing".to_owned(),
            ],
            tab_index: 0,
            errors: vec![],
            node_info,
            active_chans: 0,
            pending_chans: 0,
            sleeping_chans: 0,
            screen_width: 80,
            relays_amounts_line: vec![],
            relays_volumes_line: vec![],
            channels: vec![],
            audit: AuditInfo::default(),
        })
    }

    pub fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.tabs.len();
    }

    pub fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.tabs.len() - 1;
        }
    }

    pub fn react_hotkey(&mut self, k: KeyCode) {
        match k {
            KeyCode::Char('d') => self.tab_index = 0,
            KeyCode::Char('p') => self.tab_index = 1,
            KeyCode::Char('o') => self.tab_index = 2,
            KeyCode::Char('r') => self.tab_index = 3,
            _ => (),
        }
    }

    pub fn get_active_chans(&self) -> usize {
        self.channels
            .iter()
            .filter(|c| c.state == ChannelState::Normal)
            .count()
    }

    pub fn get_pending_chans(&self) -> usize {
        self.channels
            .iter()
            .filter(|c| {
                c.state == ChannelState::Closing
                    || c.state == ChannelState::Opening
                    || c.state == ChannelState::Syncing
            })
            .count()
    }

    pub fn get_sleeping_chans(&self) -> usize {
        self.channels
            .iter()
            .filter(|c| c.state == ChannelState::Offline)
            .count()
    }

    const LINE_PERIOD: u64 = 24 * 3600;
    const LINE_MARGINS: u64 = 2;

    pub fn get_relays_amounts_line(&mut self) -> Vec<u64> {
        let now = chrono::offset::Utc::now().timestamp();
        let mut relays: Vec<u64> = self
            .audit
            .relayed
            .iter()
            .filter(|s| s.timestamp/1000 > (now - App::LINE_PERIOD as i64) as u64)
            .map(|s| s.timestamp)
            .collect();
        relays.sort_by(|a, b| a.partial_cmp(&b).unwrap());

        let line_width = self.screen_width as u64 - App::LINE_MARGINS;
        let mut result = vec![0; line_width as usize + 1];
        if !relays.is_empty() {
            let t0 = relays[0];
            let t1 = relays[relays.len()-1];
            for t in relays.iter() {
                let i = (((t - t0) as f64)/((t1 - t0) as f64) * (line_width as f64)) as usize;
                result[i] += 1;
            }

            let max_relay = *result.iter().max().unwrap_or(&1) as f64;
            result = result.iter().map(|a| (100.0 * (*a as f64)/max_relay) as u64).collect();
        }
        result
    }

    pub fn get_relays_volumes_line(&mut self) -> Vec<u64> {
        let now = chrono::offset::Utc::now().timestamp();
        let mut relays: Vec<(u64, u64)> = self
            .audit
            .relayed
            .iter()
            .filter(|s| s.timestamp/1000 > (now - App::LINE_PERIOD as i64) as u64)
            .map(|s| (s.amount_in, s.timestamp))
            .collect();
        relays.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let line_width = self.screen_width as u64 - App::LINE_MARGINS;
        let mut result = vec![0; line_width as usize + 1];
        if !relays.is_empty() {
            let t0 = relays[0].1;
            let t1 = relays[relays.len()-1].1;
            for (amount, t) in relays.iter() {
                let i = (((t - t0) as f64)/((t1 - t0) as f64) * (line_width as f64)) as usize;
                result[i] += amount;
            }

            let max_relay = *result.iter().max().unwrap_or(&1) as f64;
            result = result.iter().map(|a| (100.0 * (*a as f64)/max_relay) as u64).collect();
        }
        result
    }

    pub async fn start_workers(mapp: AppMutex) {
        tokio::spawn({
            let mapp = mapp.clone();
            async move {
                loop {
                    let res = query_node_info(mapp.clone()).await;
                    match res {
                        Err(e) => {
                            let estr = format!("App worker failed with: {}", e);
                            // println!("{}", estr);
                            let mut app = mapp.lock().unwrap();
                            app.errors.push(estr);
                        }
                        _ => {
                            // let mut app = mapp.lock().unwrap();
                            // let num_chans = app.channels.len();
                            // app.errors.push(format!("All is ok! Got channels: {}", num_chans))
                        }
                    }
                    tokio::time::sleep(Duration::from_secs(20)).await;
                }
            }
        });
    }

    pub fn resize(&mut self, new_width: u16) {
        if self.screen_width != new_width {
            self.screen_width = new_width;
            self.relays_amounts_line = self.get_relays_amounts_line();
            self.relays_volumes_line = self.get_relays_volumes_line();
        }
    }
}

pub async fn query_node_info(mapp: AppMutex) -> Result<(), super::client::Error> {
    let client = mapp.lock().unwrap().client.clone();
    let chan_info = client.get_channels().await?;
    let audit_info = client.get_audit().await?;
    {
        let mut app = mapp.lock().unwrap();
        app.channels = chan_info;
        app.active_chans = app.get_active_chans();
        app.pending_chans = app.get_pending_chans();
        app.sleeping_chans = app.get_sleeping_chans();

        app.audit = audit_info;
        app.relays_amounts_line = app.get_relays_amounts_line();
        app.relays_volumes_line = app.get_relays_volumes_line();
    }
    Ok(())
}
