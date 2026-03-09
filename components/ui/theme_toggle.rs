use maud::{html, Markup, PreEscaped};

/// Botón para cambiar entre Dark y Light mode.
pub fn theme_toggle() -> Markup {
    html! {
        button type="button" 
            x-data="{ isDark: document.documentElement.classList.contains('dark') }"
            x-on:click="
                isDark = !isDark;
                if (isDark) {
                    document.documentElement.classList.add('dark');
                    localStorage.theme = 'dark';
                } else {
                    document.documentElement.classList.remove('dark');
                    localStorage.theme = 'light';
                }
            "
            aria-label="Toggle theme"
            // Clases consistentes con los íconos de interacción
            class="inline-flex items-center justify-center w-10 h-10 rounded-md text-foreground/80 hover:text-foreground hover:bg-accent/50 transition-colors"
        {
            // Moon (Aparece en dark mode)
            svg x-show="isDark" class="w-5 h-5" style="display: none;" fill="currentColor" viewBox="0 0 256 256" {
                (PreEscaped(r#"<path d="M216.7,152.6A91.9,91.9,0,0,1,103.4,39.3h0A92,92,0,1,0,216.7,152.6Z"></path>"#))
            }
            
            // Sun (Aparece en light mode)
            svg x-show="!isDark" class="w-5 h-5" fill="currentColor" viewBox="0 0 256 256" {
                (PreEscaped(r#"<path d="M120,40V16a8,8,0,0,1,16,0V40a8,8,0,0,1-16,0Zm72,88a64,64,0,1,1-64-64A64.1,64.1,0,0,1,192,128Zm-16,0a48,48,0,1,0-48,48A48.1,48.1,0,0,0,176,128ZM40,116H16a8,8,0,0,0,0,16H40a8,8,0,0,0,0-16Zm216,0H232a8,8,0,0,0,0,16h24a8,8,0,0,0,0-16ZM69.7,75.3a8.1,8.1,0,0,0,11.3,0l17-17a8.1,8.1,0,0,0-11.3-11.3l-17,17A8.1,8.1,0,0,0,69.7,75.3Zm133.6,105a8.1,8.1,0,0,0-11.3,11.3l17,17a8.1,8.1,0,0,0,11.3-11.3ZM191,81A8.2,8.2,0,0,0,196.7,78.6l17-17a8.1,8.1,0,0,0-11.3-11.3l-17,17A8.1,8.1,0,0,0,191,81ZM75.3,186.3a8.1,8.1,0,0,0-11.3,0l-17,17a8.1,8.1,0,1,0,11.3,11.3l17-17A8.1,8.1,0,0,0,75.3,186.3Zm56,29.7V240a8,8,0,0,1-16,0V216a8,8,0,0,1,16,0Z"></path>"#))
            }
        }
    }
}
