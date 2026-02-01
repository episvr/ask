use gtk4 as gtk;
use gtk::{prelude::*, Application, ApplicationWindow, Box as GtkBox, Entry, Label, Orientation, ScrolledWindow, CssProvider, gdk};
use glib;
use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use ask_core::config::{AppConfig, DEFAULT_SYSTEM_PROMPT};
use ask_core::api::query_gpt_stream;
use crate::markdown::markdown_to_pango;

const STYLE: &str = r#"
    window {
        background-color: transparent;
    }
    .main-window-box {
        background-color: @theme_bg_color;
        border-radius: 12px;
        box-shadow: 0 5px 15px rgba(0, 0, 0, 0.3);
        margin: 10px;
    }
    entry {
        background: transparent;
        border: none;
        box-shadow: none;
        outline: none;
        font-size: 20px;
        padding: 10px;
        color: @theme_fg_color;
    }
    entry:focus {
        background: transparent;
        box-shadow: none;
        outline: none;
    }
    label {
        font-size: 16px;
        color: @theme_fg_color;
        margin-bottom: 5px;
    }
    scrolledwindow {
        min-height: 0px;
    }
"#;

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
        // 失去焦点时自动隐藏 (Spotlight 风格)
        if !win.is_active() && win.is_visible() {
            win.set_visible(false);
        }
    });

    // 主容器，包含圆角背景
    let main_box = GtkBox::new(Orientation::Vertical, 0);
    main_box.add_css_class("main-window-box");
    
    // 搜索框容器
    let input_box = GtkBox::new(Orientation::Horizontal, 10);
    input_box.set_margin_top(10);
    input_box.set_margin_bottom(10);
    input_box.set_margin_start(15);
    input_box.set_margin_end(15);

    let entry = Entry::new();
    entry.set_placeholder_text(Some("Ask anything..."));
    entry.set_hexpand(true);
    input_box.append(&entry);
    
    main_box.append(&input_box);

    // 分割线
    let separator = gtk::Separator::new(Orientation::Horizontal);
    separator.set_visible(false); // 初始隐藏
    main_box.append(&separator);

    // 结果区域
    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    scrolled.set_max_content_height(500); // increase max height
    scrolled.set_propagate_natural_height(true);
    scrolled.set_visible(false); // 初始隐藏
    
    // 内容容器
    let content_box = GtkBox::new(Orientation::Vertical, 10);
    content_box.set_margin_top(15);
    content_box.set_margin_bottom(15);
    content_box.set_margin_start(20);
    content_box.set_margin_end(20);

    let label = Label::new(None);
    label.set_wrap(true);
    label.set_xalign(0.0);
    label.set_yalign(0.0);
    label.set_use_markup(true);
    label.set_selectable(true);
    
    content_box.append(&label);
    scrolled.set_child(Some(&content_box));
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
    
    // 监听 Entry 键盘事件 (历史记录导航)
    let entry_controller = gtk::EventControllerKey::new();
    
    entry_controller.connect_key_pressed(glib::clone!(
        @weak entry, 
        @strong history, 
        @strong history_index 
        => @default-return glib::Propagation::Proceed, 
        move |_, key, _, _| {
            // Explicitly trigger activate for Return/Enter keys
            if key == gdk::Key::Return || key == gdk::Key::KP_Enter {
                entry.emit_activate();
                return glib::Propagation::Stop;
            }
            
            let history_vec = history.borrow();
            if history_vec.is_empty() { return glib::Propagation::Proceed; }
            
            let mut idx = history_index.borrow_mut();
            
            if key == gdk::Key::Up {
                if *idx > 0 {
                    *idx -= 1;
                    entry.set_text(&history_vec[*idx]);
                    entry.set_position(-1);
                }
                return glib::Propagation::Stop;
            } else if key == gdk::Key::Down {
                if *idx < history_vec.len() {
                    *idx += 1;
                    if *idx < history_vec.len() {
                        entry.set_text(&history_vec[*idx]);
                    } else {
                        entry.set_text("");
                    }
                    entry.set_position(-1);
                }
                return glib::Propagation::Stop;
            }
            
            glib::Propagation::Proceed
    }));
    entry.add_controller(entry_controller);

    // 连接激活事件 (Enter 键提交)
    entry.connect_activate(glib::clone!(@weak label_clone, @weak scrolled_clone, @weak separator_clone, @strong history, @strong history_index, @strong cache, @strong config => move |entry| {
        let question = entry.text().to_string();
        if !question.trim().is_empty() {
            let cache_key = question.trim().to_string();
            
            if let Some(cached_response) = cache.borrow().get(&cache_key) {
                // 记录历史
                history.borrow_mut().push(question.clone());
                *history_index.borrow_mut() = history.borrow().len();

                // 显示结果区域
                scrolled_clone.set_visible(true);
                separator_clone.set_visible(true);
                
                let markup = markdown_to_pango(cached_response);
                label_clone.set_markup(&markup);
                
                entry.set_text("");
                return;
            }

            // 记录历史
            history.borrow_mut().push(question.clone());
            *history_index.borrow_mut() = history.borrow().len(); // 重置索引到最后

            // 显示结果区域
            scrolled_clone.set_visible(true);
            separator_clone.set_visible(true);
            
            // 显示加载状态
            label_clone.set_markup("<span alpha='50%'><i>Thinking...</i></span>");

            let system_prompt = DEFAULT_SYSTEM_PROMPT.to_string();
            let config_clone = Arc::clone(&config);
            let label = label_clone.clone();
            
            // 用于累积流式文本
            let accumulated_text = Rc::new(RefCell::new(String::new()));
            let acc_text_clone = accumulated_text.clone();
            let cache_clone = cache.clone();
            let cache_key_clone = cache_key.clone();

            glib::spawn_future_local(async move {
                let label_for_callback = label.clone();
                let acc_text_for_callback = acc_text_clone.clone();
                let callback = move |chunk: String| {
                    acc_text_for_callback.borrow_mut().push_str(&chunk);
                    let markup = markdown_to_pango(&acc_text_for_callback.borrow());
                    label_for_callback.set_markup(&markup);
                };

                // 使用流式API
                match query_gpt_stream(&config_clone, system_prompt, question, callback).await {
                    Ok(_) => {
                        let full_text = acc_text_clone.borrow().to_string();
                        cache_clone.borrow_mut().insert(cache_key_clone, full_text);
                    }
                    Err(e) => {
                         label.set_text(&format!("Error: {}", e));
                    }
                }
            });
            
            entry.set_text("");
        }
    }));
    
    window.present();
    entry.grab_focus();
}