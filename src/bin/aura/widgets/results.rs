use gtk4 as gtk;
use gtk::{ScrolledWindow, Box as GtkBox, Label, Orientation, PolicyType, prelude::*};

pub fn create_results() -> (ScrolledWindow, GtkBox, Label) {
    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(PolicyType::Never, PolicyType::Automatic);
    scrolled.set_max_content_height(500);
    scrolled.set_propagate_natural_height(true);
    scrolled.set_visible(false);

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

    (scrolled, content_box, label)
}
