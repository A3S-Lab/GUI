pub const UI_BUTTON_BASE_CLASS: &str = "\
inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm \
font-medium transition-[color,box-shadow] disabled:pointer-events-none disabled:opacity-50 \
[&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 \
[&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 \
focus-visible:ring-[3px] aria-invalid:ring-destructive/20 \
dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive";

pub const UI_BUTTON_DEFAULT_VARIANT_CLASS: &str =
    "bg-primary text-primary-foreground shadow-xs hover:bg-primary/90";
pub const UI_BUTTON_DEFAULT_SIZE_CLASS: &str = "h-9 px-4 py-2 has-[>svg]:px-3";
pub const UI_BUTTON_CLASS: &str = "\
inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm \
font-medium transition-[color,box-shadow] disabled:pointer-events-none disabled:opacity-50 \
[&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 \
[&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 \
focus-visible:ring-[3px] aria-invalid:ring-destructive/20 \
dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive bg-primary \
text-primary-foreground shadow-xs hover:bg-primary/90 h-9 px-4 py-2 has-[>svg]:px-3";

pub const UI_INPUT_CLASS: &str = "\
h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 \
text-base shadow-xs transition-[color,box-shadow] outline-none selection:bg-primary \
selection:text-primary-foreground file:inline-flex file:h-7 file:border-0 \
file:bg-transparent file:text-sm file:font-medium file:text-foreground \
placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed \
disabled:opacity-50 md:text-sm dark:bg-input/30 focus-visible:border-ring \
focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive \
aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40";

pub const UI_TEXTAREA_CLASS: &str = "\
border-input placeholder:text-muted-foreground focus-visible:border-ring \
focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 \
dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive \
dark:bg-input/30 flex field-sizing-content min-h-16 w-full rounded-md border \
bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] \
outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed \
disabled:opacity-50 md:text-sm";

pub const UI_CARD_CLASS: &str =
    "bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm";
pub const UI_CARD_HEADER_CLASS: &str = "\
@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-2 px-6 \
has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6";
pub const UI_CARD_TITLE_CLASS: &str = "leading-none font-semibold";
pub const UI_CARD_DESCRIPTION_CLASS: &str = "text-muted-foreground text-sm";
pub const UI_CARD_CONTENT_CLASS: &str = "px-6";
pub const UI_CARD_FOOTER_CLASS: &str = "flex items-center px-6 [.border-t]:pt-6";

pub const UI_BADGE_BASE_CLASS: &str = "\
inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs \
font-medium w-fit whitespace-nowrap shrink-0 gap-1 transition-[color,box-shadow] overflow-hidden";
pub const UI_BADGE_CLASS: &str = "\
inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs \
font-medium w-fit whitespace-nowrap shrink-0 gap-1 transition-[color,box-shadow] \
overflow-hidden bg-secondary text-secondary-foreground";

pub const UI_SEPARATOR_CLASS: &str = "\
bg-border shrink-0 data-[orientation=horizontal]:h-px \
data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full \
data-[orientation=vertical]:w-px";

pub const UI_TABS_CLASS: &str = "flex flex-col gap-2";
pub const UI_TABS_LIST_CLASS: &str = "\
bg-muted text-muted-foreground inline-flex h-9 w-fit items-center justify-center \
rounded-lg p-[3px]";
pub const UI_TABS_TRIGGER_CLASS: &str = "\
data-[selected=true]:bg-background data-[selected=true]:text-foreground \
focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:outline-ring \
inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-1.5 \
rounded-md border border-transparent px-2 py-1 text-sm font-medium whitespace-nowrap \
transition-[color,box-shadow] focus-visible:ring-[3px] disabled:pointer-events-none \
disabled:opacity-50 data-[selected=true]:shadow-sm";
pub const UI_TABS_CONTENT_CLASS: &str = "flex-1 outline-none";
