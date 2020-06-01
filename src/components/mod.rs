use yew::prelude::*;
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Copy)]
pub struct u4 {
    v: u8
}

impl u4 {
    pub fn add(&mut self, v: u4) -> (bool, u4) {
        self.v += v.v;
        if self.v >= 16 {
            self.v -= 15;
            (true, *self)
        } else {
            (false, *self)
        }
    }
}

impl fmt::Display for u4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}

pub struct Model {
    link: ComponentLink<Self>,
    register_a: u4,
    register_b: u4,
    carry: bool,
    pc: u4,
}

pub enum Msg {
    Clock,
    Reset,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            register_a: u4 {v: 0},
            register_b: u4 {v: 0},
            carry: false,
            pc: u4 {v: 0},
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clock => {
                self.register_a.add(u4 { v: 2 });
                self.pc.add(u4 {v: 1});
            }
            _ => {
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>{ "Register A" } { self.register_a }</div>
                <div>{ "Register B" } { self.register_b }</div>

                <button onclick=self.link.callback(|_| Msg::Clock)>{ "Clock" }</button>
            </div>
        }
    }
}
