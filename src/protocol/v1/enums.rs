use crate::event::NativeEventKind;
use crate::platform::{
    NativeBackendKind, NativeContainerKind, NativeTextInputKind, NativeWidgetKind,
};

use super::*;

macro_rules! define_protocol_leaf_enum {
    ($protocol:ident => $internal:ty { $( $variant:ident ),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub enum $protocol {
            $($variant,)+
        }

        impl From<$internal> for $protocol {
            fn from(value: $internal) -> Self {
                match value {
                    $(<$internal>::$variant => Self::$variant,)+
                }
            }
        }

        impl From<$protocol> for $internal {
            fn from(value: $protocol) -> Self {
                match value {
                    $($protocol::$variant => Self::$variant,)+
                }
            }
        }
    };
}

define_protocol_leaf_enum! {
    ProtocolNativeBackendKindV1 => NativeBackendKind {
        AppKit, WinUI, Gtk4, Headless,
    }
}

define_protocol_leaf_enum! {
    ProtocolNativeContainerKindV1 => NativeContainerKind {
        Linear, Grid, Canvas, Embedded,
    }
}

define_protocol_leaf_enum! {
    ProtocolNativeTextInputKindV1 => NativeTextInputKind {
        SingleLine, Search, Number, Password, Multiline,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProtocolNativeWidgetKindV1 {
    Window,
    Container(ProtocolNativeContainerKindV1),
    ScrollContainer,
    Label,
    Button,
    TextInput(ProtocolNativeTextInputKindV1),
    Checkbox,
    Switch,
    RadioGroup,
    Radio,
    ComboBox,
    List,
    ListItem,
    Tree,
    TreeItem,
    Table,
    Dialog,
    Popover,
    Tabs,
    Tab,
    Menu,
    MenuItem,
    Separator,
    Slider,
    Progress,
    Toolbar,
    Image,
    Media,
}

impl From<NativeWidgetKind> for ProtocolNativeWidgetKindV1 {
    fn from(value: NativeWidgetKind) -> Self {
        match value {
            NativeWidgetKind::Window => Self::Window,
            NativeWidgetKind::Container(value) => Self::Container(value.into()),
            NativeWidgetKind::ScrollContainer => Self::ScrollContainer,
            NativeWidgetKind::Label => Self::Label,
            NativeWidgetKind::Button => Self::Button,
            NativeWidgetKind::TextInput(value) => Self::TextInput(value.into()),
            NativeWidgetKind::Checkbox => Self::Checkbox,
            NativeWidgetKind::Switch => Self::Switch,
            NativeWidgetKind::RadioGroup => Self::RadioGroup,
            NativeWidgetKind::Radio => Self::Radio,
            NativeWidgetKind::ComboBox => Self::ComboBox,
            NativeWidgetKind::List => Self::List,
            NativeWidgetKind::ListItem => Self::ListItem,
            NativeWidgetKind::Tree => Self::Tree,
            NativeWidgetKind::TreeItem => Self::TreeItem,
            NativeWidgetKind::Table => Self::Table,
            NativeWidgetKind::Dialog => Self::Dialog,
            NativeWidgetKind::Popover => Self::Popover,
            NativeWidgetKind::Tabs => Self::Tabs,
            NativeWidgetKind::Tab => Self::Tab,
            NativeWidgetKind::Menu => Self::Menu,
            NativeWidgetKind::MenuItem => Self::MenuItem,
            NativeWidgetKind::Separator => Self::Separator,
            NativeWidgetKind::Slider => Self::Slider,
            NativeWidgetKind::Progress => Self::Progress,
            NativeWidgetKind::Toolbar => Self::Toolbar,
            NativeWidgetKind::Image => Self::Image,
            NativeWidgetKind::Media => Self::Media,
        }
    }
}

impl From<ProtocolNativeWidgetKindV1> for NativeWidgetKind {
    fn from(value: ProtocolNativeWidgetKindV1) -> Self {
        match value {
            ProtocolNativeWidgetKindV1::Window => Self::Window,
            ProtocolNativeWidgetKindV1::Container(value) => Self::Container(value.into()),
            ProtocolNativeWidgetKindV1::ScrollContainer => Self::ScrollContainer,
            ProtocolNativeWidgetKindV1::Label => Self::Label,
            ProtocolNativeWidgetKindV1::Button => Self::Button,
            ProtocolNativeWidgetKindV1::TextInput(value) => Self::TextInput(value.into()),
            ProtocolNativeWidgetKindV1::Checkbox => Self::Checkbox,
            ProtocolNativeWidgetKindV1::Switch => Self::Switch,
            ProtocolNativeWidgetKindV1::RadioGroup => Self::RadioGroup,
            ProtocolNativeWidgetKindV1::Radio => Self::Radio,
            ProtocolNativeWidgetKindV1::ComboBox => Self::ComboBox,
            ProtocolNativeWidgetKindV1::List => Self::List,
            ProtocolNativeWidgetKindV1::ListItem => Self::ListItem,
            ProtocolNativeWidgetKindV1::Tree => Self::Tree,
            ProtocolNativeWidgetKindV1::TreeItem => Self::TreeItem,
            ProtocolNativeWidgetKindV1::Table => Self::Table,
            ProtocolNativeWidgetKindV1::Dialog => Self::Dialog,
            ProtocolNativeWidgetKindV1::Popover => Self::Popover,
            ProtocolNativeWidgetKindV1::Tabs => Self::Tabs,
            ProtocolNativeWidgetKindV1::Tab => Self::Tab,
            ProtocolNativeWidgetKindV1::Menu => Self::Menu,
            ProtocolNativeWidgetKindV1::MenuItem => Self::MenuItem,
            ProtocolNativeWidgetKindV1::Separator => Self::Separator,
            ProtocolNativeWidgetKindV1::Slider => Self::Slider,
            ProtocolNativeWidgetKindV1::Progress => Self::Progress,
            ProtocolNativeWidgetKindV1::Toolbar => Self::Toolbar,
            ProtocolNativeWidgetKindV1::Image => Self::Image,
            ProtocolNativeWidgetKindV1::Media => Self::Media,
        }
    }
}

define_protocol_leaf_enum! {
    ProtocolNativeEventKindV1 => NativeEventKind {
        PressStart, PressEnd, PressUp, PressCancel, Press, LongPressStart, LongPressEnd,
        LongPress, MoveStart, Move, MoveEnd, Action, HoverStart, HoverEnd, Change,
        SelectionChange, Toggle, Focus, Blur, KeyDown, KeyUp, Copy, Cut, Paste, Close,
    }
}

define_protocol_leaf_enum! {
    ProtocolNativeRoleV1 => NativeRole {
        Window, View, Document, DocumentHead, DocumentBody, DocumentTitle, Metadata,
        ResourceLink, StyleSheet, Script, Template, Slot, Text, Abbreviation, Citation,
        Definition, DataValue, InsertedText, DeletedText, MarkedText, Time, Emphasis,
        StrongText, Code, KeyboardInput, SampleOutput, Variable, InlineQuote, Subscript,
        Superscript, SmallText, BoldText, ItalicText, StruckText, UnderlinedText,
        BidirectionalIsolate, BidirectionalOverride, Paragraph, PreformattedText, BlockQuote,
        ContactAddress, LineBreak, WordBreakOpportunity, NoBreakText, CenteredText, FontText,
        BigText, TeletypeText, Applet, BackgroundSound, Frame, FrameSet, NoEmbedFallback,
        NoFramesFallback, Marquee, Math, NextId, SelectedContent, Heading, HeadingGroup, Ruby,
        RubyBase, RubyText, RubyParenthesis, RubyTextContainer, Main, Navigation, Header,
        Footer, Article, Section, Aside, Search, Disclosure, DisclosureSummary, Figure,
        FigureCaption, DescriptionList, DescriptionTerm, DescriptionDetails, Image, Media,
        Canvas, EmbeddedContent, Button, Link, ImageMap, ImageMapArea, TextField, Checkbox,
        Switch, RadioGroup, Radio, Form, FieldSet, Legend, OptionGroup, Output, Meter, Select,
        ComboBox, ListBox, ListBoxItem, Tree, TreeItem, Dialog, Popover, Tabs, TabList, Tab,
        TabPanel, Menu, MenuItem, Separator, Slider, ProgressBar, Toolbar, Table, TableSection,
        TableRow, TableCell, TableColumn, TableCaption,
    }
}

define_protocol_leaf_enum! {
    ProtocolAccessibilityRoleV1 => AccessibilityRole {
        Window, Group, Document, DocumentHead, DocumentBody, DocumentTitle, Metadata,
        ResourceLink, StyleSheet, Script, Template, Slot, StaticText, Abbreviation, Citation,
        Definition, DataValue, InsertedText, DeletedText, MarkedText, Time, Emphasis,
        StrongText, Code, KeyboardInput, SampleOutput, Variable, InlineQuote, Subscript,
        Superscript, SmallText, BoldText, ItalicText, StruckText, UnderlinedText,
        BidirectionalIsolate, BidirectionalOverride, Paragraph, PreformattedText, BlockQuote,
        ContactAddress, LineBreak, WordBreakOpportunity, NoBreakText, CenteredText, FontText,
        BigText, TeletypeText, Applet, BackgroundSound, Frame, FrameSet, NoEmbedFallback,
        NoFramesFallback, Marquee, Math, NextId, SelectedContent, Heading, HeadingGroup, Ruby,
        RubyBase, RubyText, RubyParenthesis, RubyTextContainer, Main, Navigation, Header,
        Footer, Article, Section, Aside, Search, Disclosure, DisclosureSummary, Figure,
        FigureCaption, DescriptionList, DescriptionTerm, DescriptionDetails, Image, Media,
        Canvas, EmbeddedContent, Button, Link, ImageMap, ImageMapArea, TextField, Checkbox,
        Switch, RadioGroup, RadioButton, Form, FieldSet, Legend, OptionGroup, Output, Meter,
        ComboBox, ListBox, ListBoxOption, Tree, TreeItem, Dialog, Popover, TabGroup, TabList,
        Tab, TabPanel, Menu, MenuItem, Separator, Slider, ProgressIndicator, Toolbar, Table,
        TableSection, TableRow, TableCell, TableColumn, TableCaption,
    }
}
