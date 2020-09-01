use bevy::prelude::*;
use std::time::{Duration, Instant};

pub struct FrameLimiterPlugin {
    pub fps: f32,
}
struct FrameLimiterResource {
    fps: f32,
    last_time: Option<Instant>,
}

impl Plugin for FrameLimiterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(FrameLimiterResource {
            last_time: None,
            fps: self.fps,
        })
        .add_stage_before("update", "frame_limiter")
        .add_system_to_stage("frame_limiter", frame_limiter_system.system());
    }
}
fn frame_limiter_system(mut frame_limiter: ResMut<FrameLimiterResource>) {
    let now = Instant::now();
    if let Some(t) = frame_limiter.last_time {
        let dt = now.duration_since(t).as_secs_f32();
        let dt_left = f32::max(1.0f32 / frame_limiter.fps - dt, 0.0);
        std::thread::sleep(Duration::from_secs_f32(dt_left));
    }
    frame_limiter.last_time = Some(Instant::now());
}
