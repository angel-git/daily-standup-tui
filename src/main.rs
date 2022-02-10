extern crate core;

use crate::ui::config::ConfigApi;
use cursive::traits::*;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, NamedView, SelectView, TextView};
use cursive::{Cursive, CursiveRunnable};

pub mod api;
pub mod ui;

fn main() {
    let mut siv = cursive::default();
    siv.set_user_data(ConfigApi::new());

    let status_bar = LinearLayout::horizontal()
        .child(TextView::new("[s] Start "))
        .child(TextView::new("[i] Insert dev "))
        .child(TextView::new("[d] Delete dev"))
        .child(TextView::new(" [q] Quit "))
        .with_name("status_bar")
        .full_width();

    let main_layout = LinearLayout::vertical()
        .child(add_developers_view(&mut siv))
        .with_name("main")
        .full_height();

    siv.add_global_callback('q', Cursive::quit);
    siv.add_global_callback('i', show_add_modal);
    siv.add_global_callback('s', start_daily);
    siv.add_global_callback('d', delete_developer);
    siv.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(main_layout)
                .child(status_bar),
        )
        .title("stand-up")
        .full_screen(),
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

    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(ok)
                .with_name("name")
                .fixed_width(10),
        )
        .title("Enter the name")
        .button("Ok", |s| {
            let name = s
                .call_on_name("name", |view: &mut EditView| view.get_content())
                .unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    );
}

fn start_daily(s: &mut Cursive) {
    s.clear_global_callbacks('i');
    s.clear_global_callbacks('d');
    s.clear_global_callbacks('s');
    s.add_global_callback('n', next);
    s.add_global_callback('N', skip);
    s.with_user_data(|data: &mut ConfigApi| {
        data.start();
    });
    s.call_on_name("status_bar", |view: &mut LinearLayout| {
        view.clear();
        view.add_child(TextView::new("[n] Next "));
        view.add_child(TextView::new("[N] Skip "));
        view.add_child(TextView::new("[q] Quit "));
    });
    print_next_dev(s);
}

fn print_next_dev(s: &mut Cursive) {
    let mut view = s.find_name::<LinearLayout>("main").unwrap();
    let dev_turns = s.user_data::<ConfigApi>().unwrap().get_turns();
    view.clear();

    if dev_turns.len() == 0 {
        s.clear_global_callbacks('n');
        s.clear_global_callbacks('N');
        view.add_child(TextView::new("DONE! Press [q] to quit"));
    } else {
        view.add_child(TextView::new(format!(
            "😎 SPEAKING: {}",
            dev_turns.get(0).unwrap()
        )));
        if dev_turns.len() > 1 {
            view.add_child(DummyView);
            view.add_child(TextView::new(format!(
                "😧 NEXT: {}",
                dev_turns.get(1).unwrap()
            )));
        }
    };
}

fn next(s: &mut Cursive) {
    s.with_user_data(|data: &mut ConfigApi| {
        data.next();
    });
    print_next_dev(s);
}

fn skip(s: &mut Cursive) {
    s.with_user_data(|data: &mut ConfigApi| {
        data.skip();
    });
    print_next_dev(s);
}

fn add_developers_view(s: &mut CursiveRunnable) -> NamedView<SelectView> {
    let mut developers_list = SelectView::new();
    s.with_user_data(|data: &mut ConfigApi| developers_list.add_all_str(data.get_devs()));
    return developers_list.with_name("developers_list");
}

fn delete_developer(s: &mut Cursive) {
    fn ok(s: &mut Cursive) {
        let mut select = s
            .find_name::<SelectView<String>>("developers_list")
            .unwrap();
        match select.selected_id() {
            None => (),
            Some(focus) => {
                select.remove_item(focus);
                s.with_user_data(|data: &mut ConfigApi| data.delete_dev(focus));
            }
        }
        s.pop_layer();
    }

    let select = s
        .find_name::<SelectView<String>>("developers_list")
        .unwrap();
    let selected_dev = select.get_item(select.selected_id().unwrap()).unwrap();

    s.add_layer(
        Dialog::around(TextView::new(format!("You will delete {}", selected_dev.0)))
            .title("Delete dev")
            .button("Remove", |s| {
                ok(s);
            })
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}
