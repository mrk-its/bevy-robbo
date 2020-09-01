use bevy::prelude::*;

pub struct FrameCntPlugin;
pub struct FrameCnt(usize);

impl FrameCnt {
    pub fn do_it(&self) -> bool {
        (self.0 % 4) == 0
    }
    pub fn do_shooting(&self) -> bool {
        (self.0 % 4) == 3
    }
    pub fn value(&self) -> usize {
        self.0
    }
}

impl Plugin for FrameCntPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(FrameCnt(0))
            .add_system_to_stage(stage::LAST, frame_cnt_system.system());
    }
}
fn frame_cnt_system(mut frame_cnt: ResMut<FrameCnt>) {
    frame_cnt.0 += 1;
}
