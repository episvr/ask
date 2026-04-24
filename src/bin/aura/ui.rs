use gtk4 as gtk;
use gtk::{prelude::*, Application, ApplicationWindow, Box as GtkBox, Orientation, CssProvider, gdk};
use glib;
use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use ask_core::config::AppConfig;
use crate::constants::STYLE;
use crate::widgets::{search, results};

pub fn build_ui(app: &Application, config: Arc<AppConfig>) {
    // 加载 CSS
    let provider = CssProvider::new();
    provider.load_from_data(STYLE);
    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title(format!("Aura v{}", env!("CARGO_PKG_VERSION"))) 
        .default_width(750)
        .decorated(false) // 去除窗口装饰
        .resizable(false) // 禁止调整大小（类似搜索框）
        .build();

    // 监听键盘事件 (ESC 退出)
    let controller = gtk::EventControllerKey::new();
    let app_clone = app.clone();
    controller.connect_key_pressed(move |_, key, _, _| {
        if key == gdk::Key::Escape {
            app_clone.quit(); // 正式退出
            return glib::Propagation::Stop;
        }
        glib::Propagation::Proceed
    });
    window.add_controller(controller);

    // 禁用默认的关闭行为，改为隐藏
    window.connect_close_request(move |win| {
        win.set_visible(false);
        glib::Propagation::Stop // 阻止默认的销毁行为
    });

    window.connect_is_active_notify(move |win| {
        // 失去焦点时延迟隐藏
        if !win.is_active() && win.is_visible() {
            let win_clone = win.clone();
            glib::timeout_add_local_once(std::time::Duration::from_millis(200), move || {
                if !win_clone.is_active() && win_clone.is_visible() {
                    win_clone.set_visible(false);
                }
            });
        }
    });

    // 主容器，包含圆角背景
    let main_box = GtkBox::new(Orientation::Vertical, 0);
    main_box.add_css_class("main-window-box");
    
    // 搜索框容器（使用 widget）
    let input_box = GtkBox::new(Orientation::Horizontal, 10);
    input_box.set_margin_top(10);
    input_box.set_margin_bottom(10);
    input_box.set_margin_start(15);
    input_box.set_margin_end(15);

    let entry = search::create_entry();
    input_box.append(&entry);
    main_box.append(&input_box);

    // 分割线
    let separator = gtk::Separator::new(Orientation::Horizontal);
    separator.set_visible(false); // 初始隐藏
    main_box.append(&separator);

    // 结果区域（使用 widget）
    let (scrolled, _content_box, label) = results::create_results();
    main_box.append(&scrolled);

    window.set_child(Some(&main_box));

    let label_clone = label.clone();
    let scrolled_clone = scrolled.clone();
    let separator_clone = separator.clone();

    // 历史记录
    let history: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let history_index = Rc::new(RefCell::new(0usize));
    // 缓存
    let cache: Rc<RefCell<HashMap<String, String>>> = Rc::new(RefCell::new(HashMap::new()));

    // 将事件处理委托给 search widget
    search::attach_handlers(&entry, &label_clone, &scrolled_clone, &separator_clone, history.clone(), history_index.clone(), cache.clone(), Arc::clone(&config));
    
    window.present();
    entry.grab_focus();
}