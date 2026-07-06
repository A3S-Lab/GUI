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
                    "bg-primary text-primary-foreground shadow-xs hover:bg-primary/90",
                ),
                (
                    "secondary",
                    "bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80",
                ),
                (
                    "outline",
                    "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground",
                ),
                ("ghost", "hover:bg-accent hover:text-accent-foreground"),
                ("link", "text-primary underline-offset-4 hover:underline"),
                (
                    "destructive",
                    "bg-destructive text-white shadow-xs hover:bg-destructive/90 \
                     focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 \
                     dark:bg-destructive/60",
                ),
            ],
        )?
        .axis(
            "size",
            "default",
            [
                ("default", "h-9 px-4 py-2 has-[>svg]:px-3"),
                ("sm", "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5"),
                ("lg", "h-10 rounded-md px-6 has-[>svg]:px-4"),
                ("icon", "size-9"),
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
                "bg-primary text-primary-foreground [a&]:hover:bg-primary/90",
            ),
            (
                "secondary",
                "bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90",
            ),
            (
                "destructive",
                "bg-destructive text-white [a&]:hover:bg-destructive/90 \
                 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 \
                 dark:bg-destructive/60",
            ),
            (
                "outline",
                "text-foreground [a&]:hover:bg-accent [a&]:hover:text-accent-foreground",
            ),
        ],
    )
}
