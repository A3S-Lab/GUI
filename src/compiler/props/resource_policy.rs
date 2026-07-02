use crate::html::{canonical_html_tag, HtmlResourcePolicyProps};
use crate::web::WebProps;

use super::attributes::{bool_attribute, html_present_string_attribute, html_string_attribute};

pub(super) fn html_resource_policy_props_from_tag(
    tag: &str,
    web: &WebProps,
) -> HtmlResourcePolicyProps {
    let Some(tag) = canonical_html_tag(tag) else {
        return HtmlResourcePolicyProps::default();
    };

    let attributes = &web.attributes;
    let mut policy = HtmlResourcePolicyProps::default();

    match tag {
        "a" | "area" => {
            policy.target = html_string_attribute(attributes, &["target"]);
            policy.download = html_present_string_attribute(attributes, &["download"]);
            policy.ping = html_string_attribute(attributes, &["ping"]);
            policy.rel = html_string_attribute(attributes, &["rel"]);
            policy.href_lang = html_string_attribute(attributes, &["hreflang", "hrefLang"]);
        }
        "base" => {
            policy.target = html_string_attribute(attributes, &["target"]);
        }
        "link" => {
            policy.rel = html_string_attribute(attributes, &["rel"]);
            policy.href_lang = html_string_attribute(attributes, &["hreflang", "hrefLang"]);
            policy.link_as = html_string_attribute(attributes, &["as"]);
            policy.integrity = html_string_attribute(attributes, &["integrity"]);
            policy.blocking = html_string_attribute(attributes, &["blocking"]);
            policy.image_srcset =
                html_string_attribute(attributes, &["imagesrcset", "imageSrcSet"]);
            policy.image_sizes = html_string_attribute(attributes, &["imagesizes", "imageSizes"]);
            policy.resource_disabled = bool_attribute(attributes, &["disabled"]).unwrap_or(false);
        }
        "script" => {
            policy.integrity = html_string_attribute(attributes, &["integrity"]);
            policy.blocking = html_string_attribute(attributes, &["blocking"]);
            policy.nonce = html_string_attribute(attributes, &["nonce"]);
            policy.async_script = bool_attribute(attributes, &["async"]).unwrap_or(false);
            policy.defer_script = bool_attribute(attributes, &["defer"]).unwrap_or(false);
            policy.no_module =
                bool_attribute(attributes, &["nomodule", "noModule"]).unwrap_or(false);
        }
        "style" => {
            policy.blocking = html_string_attribute(attributes, &["blocking"]);
            policy.nonce = html_string_attribute(attributes, &["nonce"]);
        }
        "iframe" => {
            policy.frame_name = html_string_attribute(attributes, &["name"]);
            policy.frame_allow = html_string_attribute(attributes, &["allow"]);
            policy.frame_allow_fullscreen =
                bool_attribute(attributes, &["allowfullscreen", "allowFullScreen"])
                    .unwrap_or(false);
            policy.frame_sandbox = html_present_string_attribute(attributes, &["sandbox"]);
            policy.frame_srcdoc = html_string_attribute(attributes, &["srcdoc", "srcDoc"]);
        }
        _ => {}
    }

    policy
}
