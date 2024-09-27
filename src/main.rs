use gpui::*;

struct Counter {
    count: usize,
    text: SharedString,
}

struct Change {
    increment: usize,
}

impl EventEmitter<Change> for Counter {}

actions!(calculator, [Quit]);

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let counter: Model<Counter> = cx.new_model(|_cx| Counter {
            count: 0,
            text: "Dog".into(),
        });
        let subscriber = cx.new_model(|cx: &mut ModelContext<Counter>| {
            cx.subscribe(&counter, |subscriber, _emitter, event, _cx| {
                subscriber.count += event.increment * 2;
            })
            .detach();

            Counter {
                count: counter.read(cx).count * 2,
                text: "Puppy".into(),
            }
        });

        counter.update(cx, |counter, cx| {
            counter.count += 2;
            cx.notify();
            cx.emit(Change { increment: 2 });
        });

        let new_count = subscriber.read(cx).count;
        let new_text = subscriber.read(cx).text.clone();
        assert_eq!(subscriber.read(cx).count, 4);

        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "Calculator".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);

        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|_cx| Counter {
                    count: new_count,
                    text: new_text,
                })
            },
        )
        .unwrap();
    });
}

impl Render for Counter {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        std::dbg!("Rendering counter view");

        let increment_button = div()
            .bg(rgb(0x4caf50))
            .text_color(rgb(0xffffff))
            .child("Increment")
            .on_mouse_down(MouseButton::Left, move |_event, _cx| {
                std::dbg!("Incrementing counter");
                /*
                CounterModel::update(
                    |model, cx| {
                        model.inner.update(cx, |model, cx| {
                            model.count += 1;
                            cx.notify();
                        });
                    },
                    cx,
                )
                */
            });

        let nushell = div()
            .bg(rgb(0x4caf50))
            .text_color(rgb(0xffffff))
            .child("Rock and Roll");

        let hello_world = div()
            .bg(rgb(0x4caf50))
            .text_color(rgb(0xffffff))
            .child(format!("Hello {}", &self.text));

        let count = div()
            .bg(rgb(0x4caf50))
            .text_color(rgb(0xffffff))
            .child(format!("The number is {}", &self.count.to_string()));

        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size(Length::Definite(Pixels(300.0).into()))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .child(div().flex().flex_col().children(vec![
                increment_button,
                //decrement_button,
                nushell,
                hello_world,
                count,
            ]))
    }
}
