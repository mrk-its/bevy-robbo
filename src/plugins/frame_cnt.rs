use bevy::prelude::*;

pub struct FrameCntPlugin {
    key_frame_interval: usize,
}

impl FrameCntPlugin {
    pub fn new(key_frame_interval: usize) -> FrameCntPlugin {
        FrameCntPlugin { key_frame_interval }
    }
}

impl Default for FrameCntPlugin {
    fn default() -> Self {
        FrameCntPlugin::new(8)
    }
}

pub struct FrameCnt {
    cnt: usize,
    key_frame_interval: usize,
}

impl FrameCnt {
    pub fn is_keyframe(&self) -> bool {
        (self.cnt % self.key_frame_interval) == 0
    }

    pub fn value(&self) -> usize {
        self.cnt
    }
    pub fn inc(&mut self) -> usize {
        self.cnt += 1;
        self.cnt
    }
}

impl Plugin for FrameCntPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(FrameCnt {
            key_frame_interval: self.key_frame_interval,
            cnt: 0,
        })
        .add_stage_before(stage::LAST, "frame_cnt", SystemStage::parallel())
        .add_system_to_stage("frame_cnt", frame_cnt_system.system());
    }
}
fn frame_cnt_system(mut frame_cnt: ResMut<FrameCnt>) {
    frame_cnt.inc();
}
