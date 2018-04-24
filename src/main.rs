extern crate gate;

use gate::{App, Audio};
use gate::app_info::AppInfo;
use gate::input::{KeyEvent, KeyCode};
use gate::renderer::Renderer;


mod asset_id {
    include!(concat!(env!("OUT_DIR"), "/asset_id.rs"));
}
// use asset_id::{AssetId, SpriteId, TileId, MusicId, SoundId};
use asset_id::{AssetId, MusicId, SoundId};

struct SoundDemo {}

impl App<AssetId> for SoundDemo {
    fn start(&mut self, audio: &mut Audio<AssetId>) {
        println!("loop music");
        audio.loop_music(MusicId::BgMusic);
        // println!("play sound");
        // audio.play_sound(SoundId::Jump);
    }

    fn advance(&mut self, _seconds: f64, _audio: &mut Audio<AssetId>) -> bool {
        true
    }

    fn input(&mut self, _evt: KeyEvent, _key: KeyCode, _audio: &mut Audio<AssetId>) -> bool {
        true
    }

    fn render(&mut self, _renderer: &mut Renderer<AssetId>) {}
}

fn main() {
    let info = AppInfo::with_app_height(48.).title("SoundDemo").build();
    gate::run(info, SoundDemo {});
}
