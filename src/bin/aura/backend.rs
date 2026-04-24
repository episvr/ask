use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use glib;
use ask_core::api::query_gpt_stream;
use ask_core::config::AppConfig;

pub fn stream_query_and_cache(
    config: Arc<AppConfig>,
    system_prompt: String,
    question: String,
    cache: Rc<RefCell<HashMap<String, String>>>,
    cache_key: String,
    chunk_callback: Box<dyn Fn(String) + 'static>,
    error_callback: Box<dyn Fn(String) + 'static>,
) {
    glib::spawn_future_local(async move {
        let accumulated = Rc::new(RefCell::new(String::new()));
        let acc_for_cb = accumulated.clone();

        let cb = move |chunk: String| {
            acc_for_cb.borrow_mut().push_str(&chunk);
            (chunk_callback)(chunk);
        };

        match query_gpt_stream(&config, system_prompt, question, cb).await {
            Ok(_) => {
                // move accumulated String out without cloning
                let mut acc = accumulated.borrow_mut();
                let full = std::mem::take(&mut *acc);
                cache.borrow_mut().insert(cache_key, full);
            }
            Err(e) => {
                (error_callback)(format!("{}", e));
            }
        }
    });
}
