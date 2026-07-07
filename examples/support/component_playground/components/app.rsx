use a3s_gui::{ComponentCx, RSX};

use super::super::model::ComponentPlaygroundState;

#[allow(non_snake_case)]
pub fn component_playground(cx: &mut ComponentCx<ComponentPlaygroundState>) -> RSX {
    let interactionCount = cx.use_state("interactionCount", |state: &ComponentPlaygroundState| {
        state.interaction_count.to_string()
    });
    let lastEvent = cx.use_state("lastEvent", |state: &ComponentPlaygroundState| {
        state.last_event.clone()
    });
    let query = cx.use_state("query", |state: &ComponentPlaygroundState| {
        state.query.clone()
    });
    let selectedValue = cx.use_state("selectedValue", |state: &ComponentPlaygroundState| {
        state.selected_value.clone()
    });
    let activeSection = cx.use_state("activeSection", |state: &ComponentPlaygroundState| {
        state.active_section.clone()
    });
    let foundationActive = cx.use_state("foundationActive", |state: &ComponentPlaygroundState| {
        state.active_section == "foundation"
    });
    let controlsActive = cx.use_state("controlsActive", |state: &ComponentPlaygroundState| {
        state.active_section == "controls"
    });
    let collectionsActive = cx
        .use_state("collectionsActive", |state: &ComponentPlaygroundState| {
            state.active_section == "collections"
        });
    let dataActive = cx.use_state("dataActive", |state: &ComponentPlaygroundState| {
        state.active_section == "data"
    });
    let dateColorRangeActive = cx.use_state(
        "dateColorRangeActive",
        |state: &ComponentPlaygroundState| state.active_section == "date-color-range",
    );
    let overlaysFeedbackActive = cx.use_state(
        "overlaysFeedbackActive",
        |state: &ComponentPlaygroundState| state.active_section == "overlays-feedback",
    );
    let overlayOpen = cx.use_state("overlayOpen", |state: &ComponentPlaygroundState| {
        state.overlay_open
    });

    let record = cx.use_reducer("record", |state: &mut ComponentPlaygroundState, _| {
        state.record("Interaction recorded");
        Ok(())
    });
    let setValue = cx
        .use_value_reducer("setValue", |state: &mut ComponentPlaygroundState, value| {
            state.set_value(value)
        });
    let setSection = cx.use_reducer(
        "setSection",
        |state: &mut ComponentPlaygroundState, invocation| {
            if let Some(section) = invocation.value() {
                state.set_section(section);
            }
            Ok(())
        },
    );
    let closeOverlay = cx.use_reducer("closeOverlay", |state: &mut ComponentPlaygroundState, _| {
        state.close_overlay();
        Ok(())
    });
    let openOverlay = cx.use_reducer("openOverlay", |state: &mut ComponentPlaygroundState, _| {
        state.open_overlay();
        Ok(())
    });

    a3s_gui::rsx!(
        <PlaygroundShell
            key="playground"
            interactionCount={interactionCount}
            lastEvent={lastEvent}
            query={query}
            selectedValue={selectedValue}
            activeSection={activeSection}
            foundationActive={foundationActive}
            controlsActive={controlsActive}
            collectionsActive={collectionsActive}
            dataActive={dataActive}
            dateColorRangeActive={dateColorRangeActive}
            overlaysFeedbackActive={overlaysFeedbackActive}
            overlayOpen={overlayOpen}
            record={record}
            setValue={setValue}
            setSection={setSection}
            openOverlay={openOverlay}
            closeOverlay={closeOverlay}
        />
    )
}
