pub const UI_BUTTON_BASE_CLASS: &str = "\
inline-flex h-9 items-center justify-center gap-2 whitespace-nowrap rounded-md px-3 py-1.5 \
text-sm font-medium leading-none disabled:pointer-events-none disabled:text-muted-soft \
[&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 \
[&_svg]:shrink-0 outline-none focus-visible:ring-[2px] focus-visible:ring-ink/40 \
aria-invalid:border-semantic-error";

pub const UI_BUTTON_DEFAULT_VARIANT_CLASS: &str =
    "border border-primary bg-primary text-on-primary active:bg-primary-active";
pub const UI_BUTTON_DEFAULT_SIZE_CLASS: &str = "h-9 px-3 py-1.5 has-[>svg]:px-3";
pub const UI_BUTTON_CLASS: &str = "\
inline-flex h-9 items-center justify-center gap-2 whitespace-nowrap rounded-md border \
border-primary bg-primary px-3 py-1.5 text-sm font-medium leading-none text-on-primary \
disabled:pointer-events-none disabled:text-muted-soft [&_svg]:pointer-events-none \
[&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none \
active:bg-primary-active focus-visible:ring-[2px] focus-visible:ring-ink/40 \
aria-invalid:border-semantic-error has-[>svg]:px-3";

pub const UI_INPUT_CLASS: &str = "\
h-9 w-full min-w-0 rounded-md border border-hairline-strong bg-surface-card px-3 py-1.5 \
text-sm text-ink outline-none selection:bg-ink \
selection:text-canvas file:inline-flex file:h-7 file:border-0 \
file:bg-transparent file:text-sm file:font-medium file:text-ink \
placeholder:text-muted disabled:pointer-events-none disabled:cursor-not-allowed \
disabled:bg-surface-strong disabled:text-muted-soft md:text-sm focus-visible:border-ink \
focus-visible:ring-[2px] focus-visible:ring-ink/40 aria-invalid:border-semantic-error";

pub const UI_FIELD_CONTROL_CLASS: &str = "\
flex h-9 w-full min-w-0 items-center rounded-md border border-hairline-strong bg-surface-card \
focus-within:border-ink focus-within:ring-[2px] focus-within:ring-ink/40";
pub const UI_FIELD_CONTROL_INPUT_CLASS: &str = "\
min-w-0 flex-1 bg-transparent px-3 py-1.5 text-sm text-ink outline-none \
placeholder:text-muted md:text-sm";
pub const UI_FIELD_CONTROL_TRIGGER_CLASS: &str =
    "inline-flex h-9 shrink-0 items-center justify-center px-3 text-body hover:text-ink";

pub const UI_TEXTAREA_CLASS: &str = "\
border-hairline-strong placeholder:text-muted focus-visible:border-ink \
focus-visible:ring-ink/40 aria-invalid:border-semantic-error flex field-sizing-content \
min-h-20 w-full rounded-md border bg-surface-card px-3 py-1.5 text-sm text-ink \
outline-none focus-visible:ring-[2px] disabled:cursor-not-allowed \
disabled:bg-surface-strong disabled:text-muted-soft md:text-sm";

pub const UI_CARD_CLASS: &str =
    "flex flex-col gap-3 rounded-md border border-hairline bg-canvas p-3 text-ink";
pub const UI_CARD_HEADER_CLASS: &str = "\
@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 \
has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-3";
pub const UI_CARD_TITLE_CLASS: &str = "text-lg font-semibold leading-7 text-ink";
pub const UI_CARD_DESCRIPTION_CLASS: &str = "text-sm leading-6 text-body";
pub const UI_CARD_CONTENT_CLASS: &str = "";
pub const UI_CARD_FOOTER_CLASS: &str = "flex items-center [.border-t]:pt-3";

pub const UI_BADGE_BASE_CLASS: &str = "\
inline-flex min-h-5 w-fit shrink-0 items-center justify-center gap-1 overflow-hidden \
whitespace-nowrap rounded-full border border-transparent bg-surface-strong px-2.5 py-0.5 \
text-xs font-medium text-body";
pub const UI_BADGE_CLASS: &str = "\
inline-flex min-h-5 w-fit shrink-0 items-center justify-center gap-1 overflow-hidden \
whitespace-nowrap rounded-full border border-transparent bg-surface-strong px-2.5 py-0.5 \
text-xs font-medium text-body";

pub const UI_SEPARATOR_CLASS: &str = "\
bg-hairline shrink-0 data-[orientation=horizontal]:h-px \
data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full \
data-[orientation=vertical]:w-px";

pub const UI_TABS_CLASS: &str = "flex flex-col gap-2";
pub const UI_TABS_LIST_CLASS: &str = "\
inline-flex h-9 w-fit items-center justify-center rounded-md border border-hairline \
bg-canvas-soft p-0.5 text-body";
pub const UI_TABS_TRIGGER_CLASS: &str = "\
data-[selected=true]:bg-canvas data-[selected=true]:text-ink focus-visible:ring-ink/40 \
focus-visible:outline-ink inline-flex h-8 flex-1 items-center justify-center gap-1.5 \
whitespace-nowrap rounded-sm border border-transparent px-3 py-1 text-sm font-medium \
focus-visible:ring-[2px] disabled:pointer-events-none disabled:text-muted-soft";
pub const UI_TABS_CONTENT_CLASS: &str = "flex-1 outline-none";
