use maud::{html, Markup};

/// Contenedor de toasts — se renderiza una vez en el layout raíz.
pub fn toaster() -> Markup {
    html! {
        div class="fixed top-4 left-1/2 -translate-x-1/2 z-[9999] flex flex-col items-center gap-2 pointer-events-none"
            x-data="{ 
                show: false, 
                message: '', 
                type: 'success', 
                timeout: null,
                init() {
                    const cookie = document.cookie.split('; ').find(row => row.startsWith('nears_toast='));
                    if (cookie) {
                        const val = decodeURIComponent(cookie.split('=')[1]);
                        const [msg, t] = val.split('|');
                        this.message = msg;
                        this.type = t || 'success';
                        this.show = true;
                        document.cookie = 'nears_toast=; Max-Age=0; path=/';
                        this.timeout = setTimeout(() => this.show = false, 4000);
                    }
                }
            }"
            "x-on:toast.window"="
                message = $event.detail.message; 
                type = $event.detail.type || 'success'; 
                show = true; 
                clearTimeout(timeout); 
                timeout = setTimeout(() => show = false, 4000);
            " 
        {
            div x-show="show"
                x-cloak=""
                class="flex items-center gap-3 px-4 py-3 rounded-lg shadow-lg border text-sm font-medium w-max max-w-sm pointer-events-auto"
                "x-bind:class"="type === 'error' ? 'bg-destructive border-transparent text-white' : 'bg-card border-border text-foreground'"
            {
                div class="flex-shrink-0" {
                    template x-if="type === 'success'" {
                        svg class="w-5 h-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" {
                            path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" {}
                        }
                    }
                    template x-if="type === 'error'" {
                        svg class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" {
                            path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" {}
                        }
                    }
                }
                span x-text="message" {}
            }
        }
    }
}
