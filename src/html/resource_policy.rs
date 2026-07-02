use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlResourcePolicyProps {
    pub target: Option<String>,
    pub download: Option<String>,
    pub ping: Option<String>,
    pub rel: Option<String>,
    pub href_lang: Option<String>,
    pub link_as: Option<String>,
    pub integrity: Option<String>,
    pub blocking: Option<String>,
    pub nonce: Option<String>,
    pub image_srcset: Option<String>,
    pub image_sizes: Option<String>,
    pub resource_disabled: bool,
    pub async_script: bool,
    pub defer_script: bool,
    pub no_module: bool,
    pub frame_name: Option<String>,
    pub frame_allow: Option<String>,
    pub frame_allow_fullscreen: bool,
    pub frame_sandbox: Option<String>,
    pub frame_srcdoc: Option<String>,
}

impl HtmlResourcePolicyProps {
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    pub fn download(mut self, download: impl Into<String>) -> Self {
        self.download = Some(download.into());
        self
    }

    pub fn ping(mut self, ping: impl Into<String>) -> Self {
        self.ping = Some(ping.into());
        self
    }

    pub fn rel(mut self, rel: impl Into<String>) -> Self {
        self.rel = Some(rel.into());
        self
    }

    pub fn href_lang(mut self, href_lang: impl Into<String>) -> Self {
        self.href_lang = Some(href_lang.into());
        self
    }

    pub fn link_as(mut self, link_as: impl Into<String>) -> Self {
        self.link_as = Some(link_as.into());
        self
    }

    pub fn integrity(mut self, integrity: impl Into<String>) -> Self {
        self.integrity = Some(integrity.into());
        self
    }

    pub fn blocking(mut self, blocking: impl Into<String>) -> Self {
        self.blocking = Some(blocking.into());
        self
    }

    pub fn nonce(mut self, nonce: impl Into<String>) -> Self {
        self.nonce = Some(nonce.into());
        self
    }

    pub fn image_srcset(mut self, image_srcset: impl Into<String>) -> Self {
        self.image_srcset = Some(image_srcset.into());
        self
    }

    pub fn image_sizes(mut self, image_sizes: impl Into<String>) -> Self {
        self.image_sizes = Some(image_sizes.into());
        self
    }

    pub fn resource_disabled(mut self, resource_disabled: bool) -> Self {
        self.resource_disabled = resource_disabled;
        self
    }

    pub fn async_script(mut self, async_script: bool) -> Self {
        self.async_script = async_script;
        self
    }

    pub fn defer_script(mut self, defer_script: bool) -> Self {
        self.defer_script = defer_script;
        self
    }

    pub fn no_module(mut self, no_module: bool) -> Self {
        self.no_module = no_module;
        self
    }

    pub fn frame_name(mut self, frame_name: impl Into<String>) -> Self {
        self.frame_name = Some(frame_name.into());
        self
    }

    pub fn frame_allow(mut self, frame_allow: impl Into<String>) -> Self {
        self.frame_allow = Some(frame_allow.into());
        self
    }

    pub fn frame_allow_fullscreen(mut self, frame_allow_fullscreen: bool) -> Self {
        self.frame_allow_fullscreen = frame_allow_fullscreen;
        self
    }

    pub fn frame_sandbox(mut self, frame_sandbox: impl Into<String>) -> Self {
        self.frame_sandbox = Some(frame_sandbox.into());
        self
    }

    pub fn frame_srcdoc(mut self, frame_srcdoc: impl Into<String>) -> Self {
        self.frame_srcdoc = Some(frame_srcdoc.into());
        self
    }
}
