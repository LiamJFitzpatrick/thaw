mod theme;

use crate::{theme::use_theme, utils::mount_style::mount_style, Theme};
use leptos::*;
pub use theme::TableTheme;

#[component]
pub fn Table(
    #[prop(optional, into)] style: MaybeSignal<String>,
    #[prop(default=true.into(), into)] single_row: MaybeSignal<bool>,
    #[prop(optional, into)] single_column: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    mount_style("table", include_str!("./table.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--background-color: {};",
                theme.table.background_color.clone()
            ));
            css_vars.push_str(&format!(
                "--background-color-striped: {};",
                theme.table.background_color_striped.clone()
            ));
            css_vars.push_str(&format!(
                "--border-color: {};",
                theme.table.border_color.clone()
            ));
            css_vars.push_str(&format!(
                "--border-radius: {};",
                theme.common.border_radius.clone()
            ));
        });

        css_vars
    });
    view! {
        <table
            class="melt-table"
            class=("melt-table--single-row", move || single_row.get())
            class=("melt-table--single-column", move || single_column.get())
            style=move || format!("{}{}", css_vars.get(), style.get())
        >
            {children()}
        </table>
    }
}
