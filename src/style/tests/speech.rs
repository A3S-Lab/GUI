use super::support::*;

#[test]
fn parses_css_speech_properties() {
    let web = WebProps::new()
        .style("speak", "auto")
        .style("speakAs", "spell-out")
        .style("pause", "200ms 400ms")
        .style("pauseBefore", "250ms")
        .style("pauseAfter", "500ms")
        .style("rest", "weak medium")
        .style("restBefore", "strong")
        .style("restAfter", "2s")
        .style("cue", "url('/open.ogg') none")
        .style("cueBefore", "url('/before.ogg')")
        .style("cueAfter", "none")
        .style("voiceFamily", "male 1")
        .style("voiceBalance", "left")
        .style("voiceDuration", "auto")
        .style("voicePitch", "medium")
        .style("voiceRange", "high")
        .style("voiceRate", "fast")
        .style("voiceStress", "strong")
        .style("voiceVolume", "x-loud");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.speak.as_deref(), Some("auto"));
    assert_eq!(style.speak_as.as_deref(), Some("spell-out"));
    assert_eq!(style.pause.as_deref(), Some("200ms 400ms"));
    assert_eq!(style.pause_before.as_deref(), Some("250ms"));
    assert_eq!(style.pause_after.as_deref(), Some("500ms"));
    assert_eq!(style.rest.as_deref(), Some("weak medium"));
    assert_eq!(style.rest_before.as_deref(), Some("strong"));
    assert_eq!(style.rest_after.as_deref(), Some("2s"));
    assert_eq!(style.cue.as_deref(), Some("url('/open.ogg') none"));
    assert_eq!(style.cue_before.as_deref(), Some("url('/before.ogg')"));
    assert_eq!(style.cue_after.as_deref(), Some("none"));
    assert_eq!(style.voice_family.as_deref(), Some("male 1"));
    assert_eq!(style.voice_balance.as_deref(), Some("left"));
    assert_eq!(style.voice_duration.as_deref(), Some("auto"));
    assert_eq!(style.voice_pitch.as_deref(), Some("medium"));
    assert_eq!(style.voice_range.as_deref(), Some("high"));
    assert_eq!(style.voice_rate.as_deref(), Some("fast"));
    assert_eq!(style.voice_stress.as_deref(), Some("strong"));
    assert_eq!(style.voice_volume.as_deref(), Some("x-loud"));
    assert!(!style.unsupported.contains_key("speak"));
    assert!(!style.unsupported.contains_key("speak-as"));
    assert!(!style.unsupported.contains_key("pause"));
    assert!(!style.unsupported.contains_key("pause-before"));
    assert!(!style.unsupported.contains_key("pause-after"));
    assert!(!style.unsupported.contains_key("rest"));
    assert!(!style.unsupported.contains_key("rest-before"));
    assert!(!style.unsupported.contains_key("rest-after"));
    assert!(!style.unsupported.contains_key("cue"));
    assert!(!style.unsupported.contains_key("cue-before"));
    assert!(!style.unsupported.contains_key("cue-after"));
    assert!(!style.unsupported.contains_key("voice-family"));
    assert!(!style.unsupported.contains_key("voice-balance"));
    assert!(!style.unsupported.contains_key("voice-duration"));
    assert!(!style.unsupported.contains_key("voice-pitch"));
    assert!(!style.unsupported.contains_key("voice-range"));
    assert!(!style.unsupported.contains_key("voice-rate"));
    assert!(!style.unsupported.contains_key("voice-stress"));
    assert!(!style.unsupported.contains_key("voice-volume"));
}

#[test]
fn parses_tailwind_arbitrary_speech_properties() {
    let web = WebProps::new().class_name(
        "[speak:auto] [speak-as:spell-out] [pause:200ms_400ms] \
             [pause-before:250ms] [pause-after:500ms] [rest:weak_medium] \
             [rest-before:strong] [rest-after:2s] [cue:url('/open.ogg')_none] \
             [cue-before:url('/before.ogg')] [cue-after:none] \
             [voice-family:male_1] [voice-balance:left] [voice-duration:auto] \
             [voice-pitch:medium] [voice-range:high] [voice-rate:fast] \
             [voice-stress:strong] [voice-volume:x-loud] \
             hover:[speak:never] focus:[voice-rate:slow] active:[cue-before:none]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.speak.as_deref(), Some("auto"));
    assert_eq!(style.speak_as.as_deref(), Some("spell-out"));
    assert_eq!(style.pause.as_deref(), Some("200ms 400ms"));
    assert_eq!(style.pause_before.as_deref(), Some("250ms"));
    assert_eq!(style.pause_after.as_deref(), Some("500ms"));
    assert_eq!(style.rest.as_deref(), Some("weak medium"));
    assert_eq!(style.rest_before.as_deref(), Some("strong"));
    assert_eq!(style.rest_after.as_deref(), Some("2s"));
    assert_eq!(style.cue.as_deref(), Some("url('/open.ogg') none"));
    assert_eq!(style.cue_before.as_deref(), Some("url('/before.ogg')"));
    assert_eq!(style.cue_after.as_deref(), Some("none"));
    assert_eq!(style.voice_family.as_deref(), Some("male 1"));
    assert_eq!(style.voice_balance.as_deref(), Some("left"));
    assert_eq!(style.voice_duration.as_deref(), Some("auto"));
    assert_eq!(style.voice_pitch.as_deref(), Some("medium"));
    assert_eq!(style.voice_range.as_deref(), Some("high"));
    assert_eq!(style.voice_rate.as_deref(), Some("fast"));
    assert_eq!(style.voice_stress.as_deref(), Some("strong"));
    assert_eq!(style.voice_volume.as_deref(), Some("x-loud"));
    assert!(!style.unsupported.contains_key("speak"));
    assert!(!style.unsupported.contains_key("voice-rate"));
    assert!(!style.unsupported.contains_key("cue-before"));
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("speak"))
            .map(String::as_str),
        Some("never")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("voice-rate"))
            .map(String::as_str),
        Some("slow")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("cue-before"))
            .map(String::as_str),
        Some("none")
    );
}
