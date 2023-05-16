use gtk::glib::clone;
use gtk::prelude::*;
use gtk::Window;
use relm4::{prelude::*, view};
use relm4::{ComponentSender, SimpleComponent};

#[derive(Debug)]
enum AppInput {
    Increment,
    Decrement,
}

struct AppModel {
    counter: u8,
}

struct AppWidgets {
    label: gtk::Label,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = AppInput;

    type Output = ();

    type Init = u8;

    view! {
        gtk::Window {
            set_title:Some("Simple app"),
            set_default_width:300,
            set_default_height:100,
            gtk::Box {
                set_orientation:gtk::Orientation::Vertical,
                set_spacing:5,
                set_margin_all:5,

                gtk::Button {
                    set_label: "加1",
                    connect_clicked[sender]=> move |_|{
                        sender.input(AppInput::Increment);

                    }
                },

                gtk::Button::with_label("减1") {
                    connect_clicked[sender] => move |_| {
                        sender.input(AppInput::Decrement);
                    }

                },
                gtk::Label {
                    #[watch]
                    set_label: &format!("累计： {}",model.counter),
                    set_margin_all:5,
                },

            }
        }
    }

    // Initialize the UI.
    fn init(
        counter: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel { counter };

        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    // type Root = gtk::Window;

    // type Widgets = AppWidgets;
    // fn init_root() -> Self::Root {
    //     gtk::Window::builder()
    //         .title("Simple app")
    //         .default_width(300)
    //         .default_height(100)
    //         .build()
    // }

    // /// Initialize the UI and model.
    // fn init(
    //     counter: Self::Init,
    //     window: &Self::Root,
    //     sender: ComponentSender<Self>,
    // ) -> relm4::ComponentParts<Self> {
    //     let model = AppModel { counter };

    //     let vbox = gtk::Box::builder()
    //         .orientation(gtk::Orientation::Vertical)
    //         .spacing(5)
    //         .build();

    //     let inc_button = gtk::Button::with_label("加1");
    //     let dec_button = gtk::Button::with_label("减1");

    //     let label = gtk::Label::new(Some(&format!("累计: {}", model.counter)));
    //     label.set_margin_all(5);

    //     window.set_child(Some(&vbox));
    //     vbox.set_margin_all(5);
    //     vbox.append(&inc_button);
    //     vbox.append(&dec_button);
    //     vbox.append(&label);

    //     inc_button.connect_clicked(clone!(@strong sender => move |_| {
    //         sender.input(AppInput::Increment);
    //     }));

    //     dec_button.connect_clicked(clone!(@strong sender => move |_| {
    //         sender.input(AppInput::Decrement);
    //     }));

    //     let widgets = AppWidgets { label };

    //     ComponentParts { model, widgets }
    // }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppInput::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
    // Update the view to represent the updated model.
    // fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
    //     widgets.label.set_label(&format!("累计: {}", self.counter));
    // }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple_manual");
    app.run::<AppModel>(0);
}
