//use gpui::{prelude::*, App, AppContext, EventEmitter, Model, ModelContext};
use gpui::*;

struct Counter {
    count: usize,
    text: SharedString,
}

struct Change {
    increment: usize,
}

impl EventEmitter<Change> for Counter {}

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
                text: "Hello World".into(),
            }
        });

        counter.update(cx, |counter, cx| {
            counter.count += 2;
            cx.notify();
            cx.emit(Change { increment: 2 });
        });

        assert_eq!(subscriber.read(cx).count, 4);
    });
}

/*
impl Render for Counter {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        std::dbg!("Rendering counter view");
    }
}
*/

impl Render for Counter {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        std::dbg!("Rendering counter view");
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
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}
