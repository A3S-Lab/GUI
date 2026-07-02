use crate::html::canonical_html_tag;
use crate::web::WebProps;

use super::attributes::{
    bool_attribute, html_present_string_attribute, html_string_attribute, u32_attribute,
};

#[derive(Debug, Default)]
pub(super) struct HtmlResourceAliases {
    pub(super) alt: Option<String>,
    pub(super) href: Option<String>,
    pub(super) src: Option<String>,
    pub(super) srcset: Option<String>,
    pub(super) sizes: Option<String>,
    pub(super) media: Option<String>,
    pub(super) resource_type: Option<String>,
    pub(super) intrinsic_width: Option<u32>,
    pub(super) intrinsic_height: Option<u32>,
    pub(super) loading: Option<String>,
    pub(super) decoding: Option<String>,
    pub(super) fetch_priority: Option<String>,
    pub(super) cross_origin: Option<String>,
    pub(super) referrer_policy: Option<String>,
    pub(super) poster: Option<String>,
    pub(super) controls: bool,
    pub(super) autoplay: bool,
    pub(super) loop_playback: bool,
    pub(super) muted: bool,
    pub(super) plays_inline: bool,
    pub(super) preload: Option<String>,
    pub(super) track_kind: Option<String>,
    pub(super) srclang: Option<String>,
    pub(super) track_label: Option<String>,
    pub(super) default_track: bool,
}

impl HtmlResourceAliases {
    pub(super) fn from_tag(tag: &str, web: &WebProps) -> Self {
        let Some(tag) = canonical_html_tag(tag) else {
            return Self::default();
        };

        let attributes = &web.attributes;
        let mut aliases = Self::default();

        match tag {
            "a" | "area" | "base" => {
                aliases.href = html_string_attribute(attributes, &["href"]);
            }
            "link" => {
                aliases.href = html_string_attribute(attributes, &["href"]);
                aliases.media = html_string_attribute(attributes, &["media"]);
                aliases.resource_type = html_string_attribute(attributes, &["type"]);
                aliases.fetch_priority =
                    html_string_attribute(attributes, &["fetchpriority", "fetchPriority"]);
                aliases.cross_origin =
                    html_present_string_attribute(attributes, &["crossorigin", "crossOrigin"]);
                aliases.referrer_policy =
                    html_string_attribute(attributes, &["referrerpolicy", "referrerPolicy"]);
            }
            "img" => {
                aliases.alt = html_string_attribute(attributes, &["alt"]);
                aliases.src = html_string_attribute(attributes, &["src"]);
                aliases.srcset = html_string_attribute(attributes, &["srcset", "srcSet"]);
                aliases.sizes = html_string_attribute(attributes, &["sizes"]);
                aliases.intrinsic_width = u32_attribute(attributes, &["width"]);
                aliases.intrinsic_height = u32_attribute(attributes, &["height"]);
                aliases.loading = html_string_attribute(attributes, &["loading"]);
                aliases.decoding = html_string_attribute(attributes, &["decoding"]);
                aliases.fetch_priority =
                    html_string_attribute(attributes, &["fetchpriority", "fetchPriority"]);
                aliases.cross_origin =
                    html_present_string_attribute(attributes, &["crossorigin", "crossOrigin"]);
                aliases.referrer_policy =
                    html_string_attribute(attributes, &["referrerpolicy", "referrerPolicy"]);
            }
            "audio" => {
                aliases.src = html_string_attribute(attributes, &["src"]);
                aliases.cross_origin =
                    html_present_string_attribute(attributes, &["crossorigin", "crossOrigin"]);
                aliases.controls = bool_attribute(attributes, &["controls"]).unwrap_or(false);
                aliases.autoplay =
                    bool_attribute(attributes, &["autoplay", "autoPlay"]).unwrap_or(false);
                aliases.loop_playback = bool_attribute(attributes, &["loop"]).unwrap_or(false);
                aliases.muted = bool_attribute(attributes, &["muted"]).unwrap_or(false);
                aliases.preload = html_string_attribute(attributes, &["preload"]);
            }
            "video" => {
                aliases.src = html_string_attribute(attributes, &["src"]);
                aliases.cross_origin =
                    html_present_string_attribute(attributes, &["crossorigin", "crossOrigin"]);
                aliases.poster = html_string_attribute(attributes, &["poster"]);
                aliases.intrinsic_width = u32_attribute(attributes, &["width"]);
                aliases.intrinsic_height = u32_attribute(attributes, &["height"]);
                aliases.controls = bool_attribute(attributes, &["controls"]).unwrap_or(false);
                aliases.autoplay =
                    bool_attribute(attributes, &["autoplay", "autoPlay"]).unwrap_or(false);
                aliases.loop_playback = bool_attribute(attributes, &["loop"]).unwrap_or(false);
                aliases.muted = bool_attribute(attributes, &["muted"]).unwrap_or(false);
                aliases.plays_inline =
                    bool_attribute(attributes, &["playsinline", "playsInline"]).unwrap_or(false);
                aliases.preload = html_string_attribute(attributes, &["preload"]);
            }
            "source" => {
                aliases.src = html_string_attribute(attributes, &["src"]);
                aliases.srcset = html_string_attribute(attributes, &["srcset", "srcSet"]);
                aliases.sizes = html_string_attribute(attributes, &["sizes"]);
                aliases.media = html_string_attribute(attributes, &["media"]);
                aliases.resource_type = html_string_attribute(attributes, &["type"]);
                aliases.intrinsic_width = u32_attribute(attributes, &["width"]);
                aliases.intrinsic_height = u32_attribute(attributes, &["height"]);
            }
            "track" => {
                aliases.src = html_string_attribute(attributes, &["src"]);
                aliases.track_kind = html_string_attribute(attributes, &["kind"]);
                aliases.srclang = html_string_attribute(attributes, &["srclang", "srcLang"]);
                aliases.track_label = html_string_attribute(attributes, &["label"]);
                aliases.default_track = bool_attribute(attributes, &["default"]).unwrap_or(false);
            }
            "embed" => {
                aliases.src = html_string_attribute(attributes, &["src"]);
                aliases.resource_type = html_string_attribute(attributes, &["type"]);
                aliases.intrinsic_width = u32_attribute(attributes, &["width"]);
                aliases.intrinsic_height = u32_attribute(attributes, &["height"]);
            }
            "iframe" => {
                aliases.src = html_string_attribute(attributes, &["src"]);
                aliases.loading = html_string_attribute(attributes, &["loading"]);
                aliases.referrer_policy =
                    html_string_attribute(attributes, &["referrerpolicy", "referrerPolicy"]);
                aliases.intrinsic_width = u32_attribute(attributes, &["width"]);
                aliases.intrinsic_height = u32_attribute(attributes, &["height"]);
            }
            "object" => {
                aliases.src = html_string_attribute(attributes, &["data"]);
                aliases.resource_type = html_string_attribute(attributes, &["type"]);
                aliases.intrinsic_width = u32_attribute(attributes, &["width"]);
                aliases.intrinsic_height = u32_attribute(attributes, &["height"]);
            }
            "script" => {
                aliases.src = html_string_attribute(attributes, &["src"]);
                aliases.resource_type = html_string_attribute(attributes, &["type"]);
                aliases.fetch_priority =
                    html_string_attribute(attributes, &["fetchpriority", "fetchPriority"]);
                aliases.cross_origin =
                    html_present_string_attribute(attributes, &["crossorigin", "crossOrigin"]);
                aliases.referrer_policy =
                    html_string_attribute(attributes, &["referrerpolicy", "referrerPolicy"]);
            }
            "picture" => {
                aliases.intrinsic_width = u32_attribute(attributes, &["width"]);
                aliases.intrinsic_height = u32_attribute(attributes, &["height"]);
            }
            _ => {}
        }

        aliases
    }
}
