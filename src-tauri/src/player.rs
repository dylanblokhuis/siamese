use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use libmpv::{
    events::{Event, PropertyData},
    Error, Format, Mpv,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Player {
    #[serde(skip_serializing)]
    mpv: Mpv,
    pub visible: bool,
    pub paused: bool,
    pub volume: i64,
    pub time: i64,
    pub duration: i64,
}

impl Player {
    pub fn new() -> Self {
        let mpv = Mpv::new().unwrap();
        // mpv.event_context().disable_deprecated_events().unwrap();
        mpv.event_context()
            .observe_property("pause", Format::String, 0)
            .unwrap();
        mpv.event_context()
            .observe_property("volume", Format::Int64, 0)
            .unwrap();
        mpv.event_context()
            .observe_property("time-pos", Format::Int64, 0)
            .unwrap();
        mpv.event_context()
            .observe_property("duration", Format::Int64, 0)
            .unwrap();

        Self {
            mpv,
            visible: false,
            paused: true,
            volume: 0,
            time: 0,
            duration: 0,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn update(&mut self) {
        let event = self
            .mpv
            .event_context_mut()
            .wait_event(0.0)
            .unwrap_or(Err(Error::Null));

        match event {
            Ok(Event::PlaybackRestart) => self.visible = true,
            Ok(Event::EndFile(..)) => self.visible = false,
            Ok(Event::PropertyChange {
                name: "pause",
                change: PropertyData::Str(value),
                ..
            }) => match value {
                "yes" => self.paused = true,
                "no" => self.paused = false,
                _ => (),
            },
            Ok(Event::PropertyChange {
                name: "volume",
                change: PropertyData::Int64(value),
                ..
            }) => self.volume = value,
            Ok(Event::PropertyChange {
                name: "time-pos",
                change: PropertyData::Int64(value),
                ..
            }) => self.time = value,
            Ok(Event::PropertyChange {
                name: "duration",
                change: PropertyData::Int64(value),
                ..
            }) => self.duration = value,
            Err(_) => (),
            _ => (),
        }
    }

    pub fn attach(&self, wid: i64) {
        self.mpv
            .set_property("wid", wid)
            .expect("Error attaching to window");
    }

    pub fn load_file(&self, url: &str) {
        let pathbuf = PathBuf::from_str(url).unwrap();
        let path = pathbuf.to_str().unwrap();

        self.mpv
            .command("loadfile", &[format!("{:?}", path).as_str()])
            .expect("Error loading file");
    }

    pub fn stop(&self) {
        self.mpv.command("stop", &[""]).expect("Error stopping");
    }

    pub fn play(&self) {
        self.mpv.unpause().expect("Error while unpause");
    }

    pub fn pause(&self) {
        self.mpv.pause().expect("Error while pausing");
    }

    pub fn set_time_position(&self, time: i64) {
        self.mpv
            .set_property("time-pos", time)
            .expect("Error while setting time-pos");
    }

    pub fn set_volume(&self, amount: i64) {
        self.mpv
            .set_property("volume", amount)
            .expect("Error while setting volume");
    }
}
