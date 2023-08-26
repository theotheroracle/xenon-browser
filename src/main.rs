use adw::prelude::*;

use adw::{ToolbarView, Application, ApplicationWindow, HeaderBar, WindowTitle};
use gtk::{Box, ListBox, Orientation};

fn main() {
    let xenon_app = Application::builder()
        .application_id("os.saturn.xenon")
        .build();

    xenon_app.connect_activate(|app| {
        let xenon_url = WindowTitle::builder()
            .title("xenon")
            .subtitle("https://example.com")
            .focusable(true)
            .focus_on_click(true)
            .build();
            
        let xenon_headerbar = HeaderBar::builder()
            .decoration_layout("icons:close")
            .title_widget(&xenon_url)
            .build();
            
        let xenon_view = ToolbarView ::builder()
            .build();
        xenon_view.add_top_bar(&xenon_headerbar);
            
        let window = ApplicationWindow::builder()
            .application(app)
            .title("xenon")
            .default_width(350)
            // add content to window
            .content(&xenon_view)
            .build();
        window.present();
        });
    xenon_app.run();
}
