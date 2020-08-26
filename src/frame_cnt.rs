use bevy::prelude::*;

pub struct FrameCntPlugin;
pub struct FrameCnt(usize);

impl FrameCnt {
    pub fn do_it(&self) -> bool {
        (self.0 % 4) == 0
    }
}

impl Plugin for FrameCntPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(FrameCnt(0))
            .add_stage_before("update", "frame_cnt")
            .add_system_to_stage("frame_cnt", frame_cnt_system.system())
            ;
    }
}
fn frame_cnt_system(
    mut frame_cnt: ResMut<FrameCnt>,
) {
    frame_cnt.0 += 1;
    println!("#################### {} ###################", frame_cnt.0);
}
