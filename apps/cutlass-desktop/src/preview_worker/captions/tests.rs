use super::*;

/// A text lane holding one caption cue over `[start, start + duration)`.
fn lane_with_cue(engine: &mut Engine, start: i64, duration: i64) -> TrackId {
    let track = create_track(engine, TrackKind::Text, 0).expect("text lane");
    let rate = engine.project().timeline().frame_rate;
    engine
        .apply(Command::Edit(EditCommand::AddCaptionGroup {
            group: Box::new(CaptionGroupSpec::manual(track, "Captions")),
            cues: vec![CaptionCueSpec::new(
                "hello",
                TimeRange::at_rate(start, duration, rate),
            )],
        }))
        .expect("caption group");
    track
}

fn cues(engine: &Engine, spans: &[(i64, i64)]) -> Vec<CaptionCueSpec> {
    let rate = engine.project().timeline().frame_rate;
    spans
        .iter()
        .map(|&(start, duration)| {
            CaptionCueSpec::new("line", TimeRange::at_rate(start, duration, rate))
        })
        .collect()
}

#[test]
fn a_busy_text_lane_is_not_offered_to_an_overlapping_batch() {
    let mut engine = Engine::new(EngineConfig::default()).expect("engine");
    lane_with_cue(&mut engine, 0, 60);

    let batch = cues(&engine, &[(30, 30), (90, 30)]);
    assert_eq!(free_text_lane(&engine, &batch), None);
}

#[test]
fn a_batch_that_clears_the_existing_cues_reuses_the_lane() {
    let mut engine = Engine::new(EngineConfig::default()).expect("engine");
    let lane = lane_with_cue(&mut engine, 0, 60);

    let batch = cues(&engine, &[(60, 30), (120, 30)]);
    assert_eq!(free_text_lane(&engine, &batch), Some(lane));
}

#[test]
fn a_second_batch_falls_through_to_the_free_lane() {
    let mut engine = Engine::new(EngineConfig::default()).expect("engine");
    let busy = lane_with_cue(&mut engine, 0, 60);
    let free = create_track(&mut engine, TrackKind::Text, 0).expect("second text lane");

    let batch = cues(&engine, &[(0, 60)]);
    let picked = free_text_lane(&engine, &batch);
    assert_eq!(picked, Some(free));
    assert_ne!(picked, Some(busy));
}
