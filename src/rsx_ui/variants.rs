use crate::compiler::ComponentClassVariants;
use crate::error::GuiResult;

pub fn ui_button_variants() -> GuiResult<ComponentClassVariants> {
    ComponentClassVariants::new()
        .axis(
            "variant",
            "default",
            [
                (
                    "default",
                    "border border-primary bg-primary text-on-primary active:bg-primary-active",
                ),
                (
                    "secondary",
                    "border border-hairline-strong bg-canvas text-ink active:bg-surface-strong",
                ),
                (
                    "outline",
                    "border border-hairline-strong bg-transparent text-ink active:bg-surface-strong",
                ),
                (
                    "ghost",
                    "border border-transparent bg-transparent text-ink active:bg-surface-strong",
                ),
                (
                    "link",
                    "h-auto rounded-none border border-transparent bg-transparent px-0 py-0 text-link underline",
                ),
                (
                    "destructive",
                    "border border-semantic-error bg-semantic-error text-on-primary active:bg-semantic-error/90 \
                     focus-visible:ring-semantic-error/20",
                ),
            ],
        )?
        .axis(
            "size",
            "default",
            [
                ("default", "h-10 px-[18px] py-2 has-[>svg]:px-4"),
                ("sm", "h-9 gap-1.5 px-3 py-2 has-[>svg]:px-3"),
                ("lg", "h-11 rounded-md px-5 has-[>svg]:px-4"),
                ("icon", "size-10 px-0"),
            ],
        )
}

pub fn ui_badge_variants() -> GuiResult<ComponentClassVariants> {
    ComponentClassVariants::new().axis(
        "variant",
        "secondary",
        [
            (
                "default",
                "border-primary bg-primary text-on-primary",
            ),
            ("secondary", "border-transparent bg-surface-strong text-ink"),
            (
                "destructive",
                "border-semantic-error bg-semantic-error text-on-primary focus-visible:ring-semantic-error/20",
            ),
            ("outline", "border-hairline-strong bg-canvas text-ink"),
        ],
    )
}
