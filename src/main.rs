extern crate core;

use cursive::{Cursive, CursiveRunnable};
use cursive::theme::{Color, ColorStyle};
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, ListView, NamedView, SelectView, TextView};
use cursive::traits::*;
use crate::ui::config::ConfigApi;

pub mod api;
pub mod ui;

fn main() {
    let mut siv = cursive::default();
    siv.set_user_data(ConfigApi::new());

    let status_bar = LinearLayout::horizontal()
        .child(TextView::new("[s] Start "))
        .child(TextView::new("[i] Insert dev "))
        .child(TextView::new("[d] Delete dev").style(ColorStyle::new(Color::Rgb(200, 10, 10), Color::TerminalDefault)))
        .child(TextView::new(" [q] Quit "))
        .with_name("status_bar")
        .full_width();

    let main_layout = LinearLayout::horizontal()
        .child(add_developers_view(&mut siv))
        .child(ListView::new().child("t1", DummyView))
        .full_height();

    siv.add_global_callback('q', Cursive::quit);
    siv.add_global_callback('i', show_add_modal);
    siv.add_global_callback('s', start_daily);
    siv.add_global_callback('d', delete_developer);
    siv.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(main_layout)
                .child(status_bar)
        )
            .title("stand-up")
            .full_screen()
    );
    siv.run();
}


fn show_add_modal(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("developers_list", |view: &mut SelectView| {
            view.add_item_str(name)
        });
        s.with_user_data(|data: &mut ConfigApi| {
            data.add_dev(name);
        });
        s.pop_layer();
    }

    // let data = s.user_data::<ConfigApi>().expect("data should be here!");
    // if !data.is_started() {
    s.add_layer(Dialog::around(EditView::new()
        .on_submit(ok)
        .with_name("name")
        .fixed_width(10))
        .title("Enter the name")
        .button("Ok", |s| {
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
    // }
}
//
// fn do_nothing(s: &mut Cursive) {}
//
// fn if_not_started_then(f: fn(&mut Cursive) -> ()) -> fn(s: &mut Cursive) -> () {
//     return f;
// }

fn start_daily(s: &mut Cursive) {
    s.clear_global_callbacks('i');
    s.clear_global_callbacks('d');
    s.with_user_data(|data: &mut ConfigApi| {
        data.start()
    });
    s.call_on_name("status_bar", |view: &mut LinearLayout| {
        view.clear();
        view.add_child(TextView::new("[n] Next "));
        view.add_child(TextView::new("[N] Skip "));
        view.add_child(TextView::new("[m] Out "));
        view.add_child(TextView::new("[q] Quit "));
    });

    return;
}

fn add_developers_view(s: &mut CursiveRunnable) -> NamedView<SelectView> {
    let mut developers_list = SelectView::new();
    s.with_user_data(|data: &mut ConfigApi| {
        developers_list.add_all_str(data.get_devs())
    });
    return developers_list.with_name("developers_list");
}

fn delete_developer(s: &mut Cursive) {
    let mut select = s.find_name::<SelectView<String>>("developers_list").unwrap();
    match select.selected_id() {
        None => (),
        Some(focus) => {
            select.remove_item(focus);
            s.with_user_data(|data: &mut ConfigApi| {
                data.delete_dev(focus)
            });
        }
    }
}
