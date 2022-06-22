use yew::prelude::*;

enum Msg {
    AddOne,
    AddThree,
    AddFive,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            },
            Msg::AddThree => {
                self.value += 3;
                true
            },
            Msg::AddFive => {
                self.value += 5;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
            <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
            <button onclick={link.callback(|_| Msg::AddThree)}>{ "+3" }</button>
            <button onclick={link.callback(|_| Msg::AddFive)}>{ "+5" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}