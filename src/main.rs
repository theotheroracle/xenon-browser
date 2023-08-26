use adw::{Avatar, ToolbarView, Application, ApplicationWindow, HeaderBar, WindowTitle};
use gtk::{Box, Orientation, Button, ScrolledWindow};

use adw::prelude::*;

fn main() {
    let xenon_app = Application::builder()
        .application_id("os.saturn.xenon")
        .build();

    xenon_app.connect_activate(|app| {
        let xenon_url_title = WindowTitle::builder()
            .title("xenon")
            .subtitle("https://example.com")
            .focusable(true)
            .focus_on_click(true)
            .build();
            

        let xenon_headerbar = HeaderBar::builder()
            .decoration_layout("icons:close")
            .title_widget(&xenon_url_title)
            .build();
        let screenshot_button = Button::from_icon_name("camera-photo-symbolic");
        let gallery_button = Button::from_icon_name("image-x-generic-symbolic");
        let downloads_button = Button::from_icon_name("folder-download-symbolic");
        let media_button = Button::from_icon_name("media-playback-start-symbolic");
        let home_button = Button::from_icon_name("go-home-symbolic");
        let menu_button = Button::from_icon_name("open-menu-symbolic");
        xenon_headerbar.pack_start(&screenshot_button);
        xenon_headerbar.pack_start(&gallery_button);
        xenon_headerbar.pack_start(&downloads_button);
        xenon_headerbar.pack_start(&media_button);
        xenon_headerbar.pack_start(&home_button);
        xenon_headerbar.pack_end(&menu_button);
        
        let xenon_content = Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(gtk::Align::End)
            .build();
        let tab_balls = Box::new(Orientation::Vertical,0);
        let scroll_balls = ScrolledWindow::builder()
            .child(&tab_balls)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .build();
        let mut tabs_vec: Vec<Avatar> = Vec::new();
        for _x in 1..10 {
            let ball = Avatar::builder()
                .size(60)
                .icon_name("web-browser-symbolic")
                .build();
            tab_balls.append(&ball);
            tabs_vec.push(ball);
        }
        xenon_content.append(&scroll_balls);
            
        let xenon_view = ToolbarView ::builder()
            .content(&xenon_content)
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
