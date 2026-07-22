use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProtocolCompiledOrientationV1 {
    Horizontal,
    Vertical,
}

impl From<CompiledOrientation> for ProtocolCompiledOrientationV1 {
    fn from(value: CompiledOrientation) -> Self {
        match value {
            CompiledOrientation::Horizontal => Self::Horizontal,
            CompiledOrientation::Vertical => Self::Vertical,
        }
    }
}

impl From<ProtocolCompiledOrientationV1> for CompiledOrientation {
    fn from(value: ProtocolCompiledOrientationV1) -> Self {
        match value {
            ProtocolCompiledOrientationV1::Horizontal => Self::Horizontal,
            ProtocolCompiledOrientationV1::Vertical => Self::Vertical,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProtocolCompiledStyleValueV1 {
    String(String),
    Number(f64),
    Bool(bool),
}

impl From<&CompiledStyleValue> for ProtocolCompiledStyleValueV1 {
    fn from(value: &CompiledStyleValue) -> Self {
        match value {
            CompiledStyleValue::String(value) => Self::String(value.clone()),
            CompiledStyleValue::Number(value) => Self::Number(*value),
            CompiledStyleValue::Bool(value) => Self::Bool(*value),
        }
    }
}

impl From<ProtocolCompiledStyleValueV1> for CompiledStyleValue {
    fn from(value: ProtocolCompiledStyleValueV1) -> Self {
        match value {
            ProtocolCompiledStyleValueV1::String(value) => Self::String(value),
            ProtocolCompiledStyleValueV1::Number(value) => Self::Number(value),
            ProtocolCompiledStyleValueV1::Bool(value) => Self::Bool(value),
        }
    }
}

/// Stable metadata shared by every version-1 protocol message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolMetadataV1 {
    pub protocol_version: u32,
    pub session_id: String,
    pub render_revision: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_sequence: Option<u64>,
}

impl ProtocolMetadataV1 {
    pub fn render(session_id: impl Into<String>, render_revision: u64) -> Self {
        Self {
            protocol_version: NATIVE_PROTOCOL_VERSION_V1,
            session_id: session_id.into(),
            render_revision,
            event_sequence: None,
        }
    }

    pub fn event(session_id: impl Into<String>, render_revision: u64, event_sequence: u64) -> Self {
        Self {
            event_sequence: Some(event_sequence),
            ..Self::render(session_id, render_revision)
        }
    }
}

/// Versioned transport envelope. Payloads contain protocol DTOs rather than
/// runtime or platform implementation objects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolEnvelopeV1<T> {
    pub metadata: ProtocolMetadataV1,
    pub payload: T,
}

impl<T> ProtocolEnvelopeV1<T> {
    pub fn new(metadata: ProtocolMetadataV1, payload: T) -> Self {
        Self { metadata, payload }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolUiActionV1 {
    pub id: String,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl From<&UiAction> for ProtocolUiActionV1 {
    fn from(action: &UiAction) -> Self {
        Self {
            id: action.id.clone(),
            disabled: action.disabled,
            label: action.label.clone(),
        }
    }
}

impl From<ProtocolUiActionV1> for UiAction {
    fn from(action: ProtocolUiActionV1) -> Self {
        Self {
            id: action.id,
            disabled: action.disabled,
            label: action.label,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolWindowOptionsV1 {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_close: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_height: Option<f64>,
    #[serde(default = "default_true")]
    pub resizable: bool,
}

impl From<&WindowOptions> for ProtocolWindowOptionsV1 {
    fn from(window: &WindowOptions) -> Self {
        Self {
            title: window.title.clone(),
            on_close: window.on_close.clone(),
            width: window.width,
            height: window.height,
            min_width: window.min_width,
            min_height: window.min_height,
            max_width: window.max_width,
            max_height: window.max_height,
            resizable: window.resizable,
        }
    }
}

impl From<ProtocolWindowOptionsV1> for WindowOptions {
    fn from(window: ProtocolWindowOptionsV1) -> Self {
        Self {
            title: window.title,
            on_close: window.on_close,
            width: window.width,
            height: window.height,
            min_width: window.min_width,
            min_height: window.min_height,
            max_width: window.max_width,
            max_height: window.max_height,
            resizable: window.resizable,
        }
    }
}

macro_rules! define_protocol_compiled_props_v1 {
    ($( $field:ident: $ty:ty ),+ $(,)?) => {
        /// Resolved authoring properties accepted by protocol v1.
        ///
        /// Binding and spread expressions are intentionally absent: a wire frame
        /// is an execution artifact and must be fully resolved before transport.
        #[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        pub struct ProtocolCompiledPropsV1 {
            $(#[serde(default)] pub $field: $ty,)+
            #[serde(default)]
            pub orientation: Option<ProtocolCompiledOrientationV1>,
            #[serde(default)]
            pub style: BTreeMap<String, ProtocolCompiledStyleValueV1>,
            #[serde(default)]
            pub attributes: BTreeMap<String, String>,
            #[serde(default)]
            pub events: BTreeMap<String, String>,
            #[serde(default)]
            pub action_labels: BTreeMap<String, String>,
            #[serde(default)]
            pub explicit_props: BTreeSet<String>,
        }

        impl From<&CompiledProps> for ProtocolCompiledPropsV1 {
            fn from(props: &CompiledProps) -> Self {
                Self {
                    $($field: props.$field.clone(),)+
                    orientation: props.orientation.map(Into::into),
                    style: props
                        .style
                        .iter()
                        .map(|(name, value)| (name.clone(), value.into()))
                        .collect(),
                    attributes: props.attributes.clone(),
                    events: props.events.clone(),
                    action_labels: props.action_labels.clone(),
                    explicit_props: props.explicit_props.clone(),
                }
            }
        }

        impl From<ProtocolCompiledPropsV1> for CompiledProps {
            fn from(props: ProtocolCompiledPropsV1) -> Self {
                Self {
                    $($field: props.$field,)+
                    orientation: props.orientation.map(Into::into),
                    style: props
                        .style
                        .into_iter()
                        .map(|(name, value)| (name, value.into()))
                        .collect(),
                    attributes: props.attributes,
                    events: props.events,
                    action_labels: props.action_labels,
                    bindings: BTreeMap::new(),
                    spreads: Vec::new(),
                    explicit_props: props.explicit_props,
                }
            }
        }
    };
}

define_protocol_compiled_props_v1! {
    label: Option<String>,
    text_value: Option<String>,
    value: Option<String>,
    placeholder: Option<String>,
    action: Option<String>,
    aria_label: Option<String>,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
    is_selected: bool,
    is_checked: Option<bool>,
    is_expanded: Option<bool>,
    min_value: Option<f64>,
    max_value: Option<f64>,
    value_number: Option<f64>,
    step_value: Option<f64>,
    name: Option<String>,
    form: Option<String>,
    input_type: Option<String>,
    accept: Option<String>,
    capture: Option<String>,
    alt: Option<String>,
    href: Option<String>,
    src: Option<String>,
    srcset: Option<String>,
    sizes: Option<String>,
    media: Option<String>,
    resource_type: Option<String>,
    intrinsic_width: Option<u32>,
    intrinsic_height: Option<u32>,
    loading: Option<String>,
    decoding: Option<String>,
    fetch_priority: Option<String>,
    cross_origin: Option<String>,
    referrer_policy: Option<String>,
    poster: Option<String>,
    controls: Option<bool>,
    autoplay: Option<bool>,
    loop_playback: Option<bool>,
    muted: Option<bool>,
    plays_inline: Option<bool>,
    preload: Option<String>,
    track_kind: Option<String>,
    srclang: Option<String>,
    track_label: Option<String>,
    default_track: Option<bool>,
    list: Option<String>,
    dirname: Option<String>,
    form_action: Option<String>,
    form_enctype: Option<String>,
    form_method: Option<String>,
    form_target: Option<String>,
    form_no_validate: Option<bool>,
    id: Option<String>,
    class_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum ProtocolCompiledNodeV1 {
    Element {
        key: String,
        tag: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        import_source: Option<String>,
        #[serde(default)]
        props: ProtocolCompiledPropsV1,
        #[serde(default)]
        children: Vec<ProtocolCompiledNodeV1>,
    },
    Text {
        key: String,
        value: String,
    },
}

impl From<&CompiledRsxNode> for ProtocolCompiledNodeV1 {
    fn from(node: &CompiledRsxNode) -> Self {
        match node {
            CompiledRsxNode::Element {
                key,
                tag,
                import_source,
                props,
                children,
            } => Self::Element {
                key: key.clone(),
                tag: tag.clone(),
                import_source: import_source.clone(),
                props: props.into(),
                children: children.iter().map(Self::from).collect(),
            },
            CompiledRsxNode::Text { key, value } => Self::Text {
                key: key.clone(),
                value: value.clone(),
            },
        }
    }
}

impl From<ProtocolCompiledNodeV1> for CompiledRsxNode {
    fn from(node: ProtocolCompiledNodeV1) -> Self {
        match node {
            ProtocolCompiledNodeV1::Element {
                key,
                tag,
                import_source,
                props,
                children,
            } => Self::Element {
                key,
                tag,
                import_source,
                props: props.into(),
                children: children.into_iter().map(Self::from).collect(),
            },
            ProtocolCompiledNodeV1::Text { key, value } => Self::Text { key, value },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolUiFrameV1 {
    pub frame_id: String,
    pub root: ProtocolCompiledNodeV1,
    #[serde(default)]
    pub actions: Vec<ProtocolUiActionV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<ProtocolWindowOptionsV1>,
}

impl TryFrom<&UiFrame> for ProtocolUiFrameV1 {
    type Error = GuiError;

    fn try_from(frame: &UiFrame) -> Result<Self, Self::Error> {
        frame.validate()?;
        Ok(Self {
            frame_id: frame.frame_id.clone(),
            root: (&frame.root).into(),
            actions: frame.actions.iter().map(Into::into).collect(),
            window: frame.window.as_ref().map(Into::into),
        })
    }
}

impl TryFrom<ProtocolUiFrameV1> for UiFrame {
    type Error = GuiError;

    fn try_from(frame: ProtocolUiFrameV1) -> Result<Self, Self::Error> {
        UiFrame::from_compiled_parts(
            frame.frame_id,
            frame.root.into(),
            Some(frame.actions.into_iter().map(Into::into).collect()),
            frame.window.map(Into::into),
        )
    }
}

/// Stable protocol projection of a planned widget.
///
/// `portable_style` is derived from the explicit class/style inputs on decode,
/// keeping the large parser-owned representation out of the wire contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolWidgetBlueprintV1 {
    pub backend: ProtocolNativeBackendKindV1,
    pub widget_kind: ProtocolNativeWidgetKindV1,
    pub widget_class: String,
    pub role: ProtocolNativeRoleV1,
    pub accessibility_role: ProtocolAccessibilityRoleV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    pub control_state: ProtocolNativeControlStateV1,
    #[serde(default)]
    pub style: BTreeMap<String, String>,
    #[serde(default)]
    pub events: BTreeMap<String, String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

impl From<&NativeWidgetBlueprint> for ProtocolWidgetBlueprintV1 {
    fn from(blueprint: &NativeWidgetBlueprint) -> Self {
        let value_sensitivity = effective_blueprint_value_sensitivity(blueprint);
        let mut control_state: ProtocolNativeControlStateV1 = (&blueprint.control_state).into();
        if value_sensitivity.is_sensitive() {
            control_state.accessibility_description.value_text = None;
        }
        let mut metadata = blueprint.metadata.clone();
        value_sensitivity.redact_metadata(&mut metadata);
        Self {
            backend: blueprint.backend.into(),
            widget_kind: blueprint.widget_kind.into(),
            widget_class: blueprint.widget_class.clone(),
            role: blueprint.role.into(),
            accessibility_role: blueprint.accessibility_role.into(),
            label: blueprint.label.clone(),
            value: value_sensitivity
                .redact(blueprint.value.as_deref())
                .map(ToOwned::to_owned),
            action: blueprint.action.clone(),
            class_name: blueprint.class_name.clone(),
            control_state,
            style: blueprint.style.clone(),
            events: blueprint.events.clone(),
            metadata,
        }
    }
}

impl From<ProtocolWidgetBlueprintV1> for NativeWidgetBlueprint {
    fn from(blueprint: ProtocolWidgetBlueprintV1) -> Self {
        let mut web = WebProps::new();
        web.class_name = blueprint.class_name.clone();
        web.style = blueprint.style.clone();
        let portable_style = PortableStyle::from_web(&web);
        let mut native = Self {
            backend: blueprint.backend.into(),
            widget_kind: blueprint.widget_kind.into(),
            widget_class: blueprint.widget_class,
            role: blueprint.role.into(),
            accessibility_role: blueprint.accessibility_role.into(),
            label: blueprint.label,
            value: blueprint.value,
            value_sensitivity: ValueSensitivity::Public,
            action: blueprint.action,
            class_name: blueprint.class_name,
            control_state: blueprint.control_state.into(),
            style: blueprint.style,
            portable_style,
            events: blueprint.events,
            metadata: blueprint.metadata,
        };
        native.value_sensitivity = effective_blueprint_value_sensitivity(&native);
        if native.value_sensitivity.is_sensitive() {
            native.value = None;
            native.control_state.accessibility_description.value_text = None;
        }
        native
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum ProtocolCommandV1 {
    Create {
        id: u64,
        blueprint: ProtocolWidgetBlueprintV1,
    },
    Update {
        id: u64,
        blueprint: ProtocolWidgetBlueprintV1,
    },
    InsertChild {
        parent: u64,
        child: u64,
        index: usize,
    },
    Remove {
        id: u64,
    },
    SetRoot {
        id: u64,
    },
    RequestFocus {
        id: u64,
    },
}

impl From<&PlatformCommand> for ProtocolCommandV1 {
    fn from(command: &PlatformCommand) -> Self {
        match command {
            PlatformCommand::Create { id, blueprint } => Self::Create {
                id: id.get(),
                blueprint: blueprint.into(),
            },
            PlatformCommand::Update { id, blueprint } => Self::Update {
                id: id.get(),
                blueprint: blueprint.into(),
            },
            PlatformCommand::InsertChild {
                parent,
                child,
                index,
            } => Self::InsertChild {
                parent: parent.get(),
                child: child.get(),
                index: *index,
            },
            PlatformCommand::Remove { id } => Self::Remove { id: id.get() },
            PlatformCommand::SetRoot { id } => Self::SetRoot { id: id.get() },
            PlatformCommand::RequestFocus { id } => Self::RequestFocus { id: id.get() },
        }
    }
}

impl TryFrom<ProtocolCommandV1> for PlatformCommand {
    type Error = GuiError;

    fn try_from(command: ProtocolCommandV1) -> Result<Self, Self::Error> {
        fn node(id: u64) -> GuiResult<HostNodeId> {
            if id == 0 {
                Err(GuiError::host(
                    "version-1 protocol commands require non-zero node ids",
                ))
            } else {
                Ok(HostNodeId::new(id))
            }
        }

        match command {
            ProtocolCommandV1::Create { id, blueprint } => Ok(Self::Create {
                id: node(id)?,
                blueprint: blueprint.into(),
            }),
            ProtocolCommandV1::Update { id, blueprint } => Ok(Self::Update {
                id: node(id)?,
                blueprint: blueprint.into(),
            }),
            ProtocolCommandV1::InsertChild {
                parent,
                child,
                index,
            } => Ok(Self::InsertChild {
                parent: node(parent)?,
                child: node(child)?,
                index,
            }),
            ProtocolCommandV1::Remove { id } => Ok(Self::Remove { id: node(id)? }),
            ProtocolCommandV1::SetRoot { id } => Ok(Self::SetRoot { id: node(id)? }),
            ProtocolCommandV1::RequestFocus { id } => Ok(Self::RequestFocus { id: node(id)? }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolAccessibilityNodeV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<u64>,
    pub role: ProtocolAccessibilityRoleV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub relationships: ProtocolAccessibilityRelationshipPropsV1,
    pub description: ProtocolAccessibilityDescriptionPropsV1,
    pub structure: ProtocolAccessibilityStructurePropsV1,
    pub state: ProtocolAccessibilityStatePropsV1,
    pub disabled: bool,
    pub required: bool,
    pub invalid: bool,
    pub read_only: bool,
    pub multiple: bool,
    pub focused: bool,
    pub selected: bool,
    pub checked: Option<bool>,
    pub expanded: Option<bool>,
    #[serde(default)]
    pub children: Vec<ProtocolAccessibilityNodeV1>,
}

impl From<&AccessibilityNode> for ProtocolAccessibilityNodeV1 {
    fn from(node: &AccessibilityNode) -> Self {
        let mut description = node.description.clone();
        if node.value_sensitivity.is_sensitive() {
            description.value_text = None;
        }
        Self {
            node: node.node.map(HostNodeId::get),
            role: node.role.into(),
            label: node.label.clone(),
            value: node
                .value_sensitivity
                .redact(node.value.as_deref())
                .map(ToOwned::to_owned),
            relationships: (&node.relationships).into(),
            description: (&description).into(),
            structure: (&node.structure).into(),
            state: (&node.state).into(),
            disabled: node.disabled,
            required: node.required,
            invalid: node.invalid,
            read_only: node.read_only,
            multiple: node.multiple,
            focused: node.focused,
            selected: node.selected,
            checked: node.checked,
            expanded: node.expanded,
            children: node.children.iter().map(Self::from).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolRenderPayloadV1 {
    pub frame_id: String,
    pub root: u64,
    pub command_sequence: u64,
    pub commands: Vec<ProtocolCommandV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessibility_tree: Option<ProtocolAccessibilityNodeV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolCommandAckV1 {
    pub command_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolActionInvocationV1 {
    pub node: u64,
    pub action: String,
    pub event: ProtocolNativeEventKindV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolInteractionStateV1 {
    pub focused: bool,
    pub pressed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub selected: bool,
    pub checked: Option<bool>,
    pub expanded: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolInteractionChangeV1 {
    pub node: u64,
    pub before: ProtocolInteractionStateV1,
    pub after: ProtocolInteractionStateV1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolEventPayloadV1 {
    pub frame_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invocation: Option<ProtocolActionInvocationV1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interaction_changes: Vec<ProtocolInteractionChangeV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessibility_tree: Option<ProtocolAccessibilityNodeV1>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolNativeEventV1 {
    pub node: u64,
    pub kind: ProtocolNativeEventKindV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtocolHostEventPayloadV1 {
    pub frame_id: String,
    pub event: ProtocolNativeEventV1,
}

impl TryFrom<ProtocolHostEventPayloadV1> for HostEvent {
    type Error = GuiError;

    fn try_from(event: ProtocolHostEventPayloadV1) -> Result<Self, Self::Error> {
        if event.event.node == 0 {
            return Err(GuiError::host(
                "version-1 host events require a non-zero node id",
            ));
        }
        Ok(Self {
            frame_id: event.frame_id,
            event: NativeEvent {
                node: HostNodeId::new(event.event.node),
                kind: event.event.kind.into(),
                value: event.event.value,
                context: Default::default(),
            },
        })
    }
}

impl From<&HostEvent> for ProtocolHostEventPayloadV1 {
    fn from(event: &HostEvent) -> Self {
        Self {
            frame_id: event.frame_id.clone(),
            event: ProtocolNativeEventV1 {
                node: event.event.node.get(),
                kind: event.event.kind.into(),
                value: event.event.value.clone(),
            },
        }
    }
}

pub type ProtocolRenderRequestV1 = ProtocolEnvelopeV1<ProtocolUiFrameV1>;
pub type ProtocolRenderResponseV1 = ProtocolEnvelopeV1<ProtocolRenderPayloadV1>;
pub type ProtocolHostEventV1 = ProtocolEnvelopeV1<ProtocolHostEventPayloadV1>;
pub type ProtocolHostEventResponseV1 = ProtocolEnvelopeV1<ProtocolEventPayloadV1>;
pub type ProtocolRenderAckV1 = ProtocolEnvelopeV1<ProtocolCommandAckV1>;
