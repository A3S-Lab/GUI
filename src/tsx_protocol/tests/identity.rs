use super::*;

fn render_with_action_id(id: impl Into<String>) -> TsxClientMessageV1 {
    let mut render = counter_render(2, 1);
    let TsxClientMessageV1::Render { payload, .. } = &mut render else {
        unreachable!()
    };
    payload.actions[0].id = id.into();
    render
}

#[test]
fn render_action_identity_contract_fails_before_session_mutation() {
    let invalid_ids = [
        "a3s:manual".to_string(),
        "a3s:c1:4:root7:index:0".to_string(),
        "a3s:a1:4:save8:onPress".to_string(),
        "10".to_string(),
        "save\0now".to_string(),
        "界".repeat(342),
    ];
    for id in invalid_ids {
        let error = render_with_action_id(id).validate().unwrap_err();
        assert!(error.to_string().contains("TSX render action id"));
    }

    render_with_action_id("a3s:a1:9:increment7:onPress")
        .validate()
        .unwrap();

    let mut session = TsxHostApplicationSessionV1::new(&negotiated(4096)).unwrap();
    let error = session
        .accept_render(&render_with_action_id("a3s:manual"))
        .unwrap_err();
    assert!(error.to_string().contains("reserved"));
    assert_eq!(session.last_client_message_id(), 1);
    assert_eq!(session.committed_render_revision(), 0);
    assert!(session.pending_render().is_none());
}
