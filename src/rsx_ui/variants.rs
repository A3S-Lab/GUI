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
                    "border border-hairline-strong bg-surface-card text-ink active:bg-surface-strong",
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
                    "error",
                    "border border-semantic-error bg-semantic-error text-canvas active:bg-semantic-error/90 \
                     focus-visible:ring-semantic-error/20",
                ),
            ],
        )?
        .axis(
            "size",
            "default",
            [
                ("default", "h-9 px-3 py-1.5 has-[>svg]:px-3"),
                ("sm", "h-8 gap-1.5 px-2.5 py-1.5 has-[>svg]:px-2.5"),
                ("lg", "h-10 rounded-md px-3.5 has-[>svg]:px-3"),
                ("icon", "size-9 px-0"),
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
                "border-hairline-strong bg-surface-card text-ink",
            ),
            ("secondary", "border-transparent bg-surface-strong text-body"),
            (
                "error",
                "border-semantic-error bg-semantic-error text-canvas focus-visible:ring-semantic-error/20",
            ),
            ("outline", "border-hairline-strong bg-surface-card text-ink"),
        ],
    )
}
