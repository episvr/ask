use gtk4 as gtk;
use gtk::{Entry, Label, ScrolledWindow, Separator, EventControllerKey, gdk, prelude::*};
use glib;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use ask_core::config::AppConfig;
use crate::markdown::markdown_to_pango;
use crate::constants::THINKING_MARKUP;

pub fn create_entry() -> Entry {
    let entry = Entry::new();
    entry.set_placeholder_text(Some("Ask anything..."));
    entry.set_hexpand(true);
    entry
}

pub fn attach_handlers(
    entry: &Entry,
    label: &Label,
    scrolled: &ScrolledWindow,
    separator: &Separator,
    history: Rc<RefCell<Vec<String>>>,
    history_index: Rc<RefCell<usize>>,
    cache: Rc<RefCell<HashMap<String, String>>>,
    config: Arc<AppConfig>,
) {
    // 历史导航键
    let entry_controller = EventControllerKey::new();
    entry_controller.connect_key_pressed(glib::clone!(
        @weak entry,
        @strong history,
        @strong history_index
        => @default-return glib::Propagation::Proceed,
        move |_, key, _, _| {
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

    // 激活事件（Enter 提交）
    entry.connect_activate(glib::clone!(
        @weak label, @weak scrolled, @weak separator, @strong history, @strong history_index, @strong cache, @strong config
        => move |entry| {
            let question = entry.text().to_string();
            if !question.trim().is_empty() {
                let cache_key = question.trim().to_string();

                if let Some(cached_response) = cache.borrow().get(&cache_key) {
                    history.borrow_mut().push(question.clone());
                    *history_index.borrow_mut() = history.borrow().len();

                    scrolled.set_visible(true);
                    separator.set_visible(true);

                    let markup = markdown_to_pango(cached_response);
                    label.set_markup(&markup);

                    entry.set_text("");
                    return;
                }

                history.borrow_mut().push(question.clone());
                *history_index.borrow_mut() = history.borrow().len();

                scrolled.set_visible(true);
                separator.set_visible(true);

                label.set_markup(THINKING_MARKUP);

                            let system_prompt = ask_core::config::DEFAULT_SYSTEM_PROMPT;
                            let config_clone = Arc::clone(&config);
                            let label_for_async = label.clone();
                            let label_for_chunks = label_for_async.clone();
                            let label_for_error = label_for_async.clone();

                            let cache_clone = cache.clone();
                            let cache_key_clone = cache_key.clone();

                            // 本地累计缓冲，避免每次都读取 widget 文本并减少 string 分配
                            let accumulated_text = Rc::new(RefCell::new(String::with_capacity(1024)));
                            let acc_for_chunks = accumulated_text.clone();

                            // 调用 backend 以处理流式查询并缓存结果
                            crate::backend::stream_query_and_cache(
                                config_clone,
                                system_prompt,
                                question,
                                cache_clone,
                                cache_key_clone,
                                Box::new(move |chunk: String| {
                                    // 将 chunk 累加到本地缓冲并立即渲染（保持流式体验）
                                    acc_for_chunks.borrow_mut().push_str(&chunk);
                                    let markup = markdown_to_pango(&acc_for_chunks.borrow());
                                    label_for_chunks.set_markup(&markup);
                                }),
                                Box::new(move |err: String| {
                                    label_for_error.set_text(&format!("Error: {}", err));
                                }),
                            );

                entry.set_text("");
            }
    }));
}
