use yew::prelude::*;

enum Event {
    Add(i64)
}
struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Event;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Event) -> bool {
        match msg {
            Event::Add(c) => {
                self.value += c;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
            <button onclick={link.callback(|_| Event::Add(1))}>{ "+1" }</button>
            <button onclick={link.callback(|_| Event::Add(3))}>{ "+3" }</button>
            <button onclick={link.callback(|_| Event::Add(5))}>{ "+5" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}