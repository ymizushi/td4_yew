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
            self.v -= 16;
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
    o_port: u4,
    i_port: u4,
    rom: [u8;16]
}

pub enum Msg {
    Clock,
    Reset,
    ROMOperation(u4),
    GotInput(usize, String),
}

impl Model {
    fn clock(&mut self) {
        let (carry, pc) = self.pc.add(u4 {v: 1});
        self.carry = carry; // あとでキャリーフラグのクリア処理を書く
        self.operate(self.rom[pc.v as usize]);
    }

    fn operate(&mut self, r: u8) {
        let opcode  = r >> 4;
        let operand = r & 0b00001111u8;
        match opcode {
            0b0000 => self.register_a.add(u4 {v: opcode}),
            0b0101 => self.register_b.add(u4 {v: opcode}),
            _ => {(true, u4 {v: 0})}
        };
    }
}

impl Model {
    fn view_rom(&self, index: usize, rom: u8) -> Html {
        html! {
            <div>
                <p>{"index: "}{ index }</p>
                <p>{"rom: "}{ rom }</p>
                <textarea rows=5
                    value=rom
                    oninput=self.link.callback(move |e: InputData| Msg::GotInput(index, e.value))
                    placeholder="placeholder" />
            </div>
        }
    }
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
            o_port: u4 {v: 0},
            i_port: u4 {v: 0},
            rom: [0; 16],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clock => {
                self.clock();
            }
            Msg::Reset => {
                self.pc = u4 {v: 0};
            }
            Msg::ROMOperation(_) => {
            }
            Msg::GotInput(index, rom) => {
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
                <div>{ "Register A: " } { self.register_a }</div>
                <div>{ "Register B: " } { self.register_b }</div>
                <div>{ "PC: " } { self.pc }</div>
                { self.rom.into_iter().enumerate().map(|(i, rom)| self.view_rom(i, *rom)).collect::<Html>() }
                <button onclick=self.link.callback(|_| Msg::Clock)>{ "Clock" }</button>
            </div>
        }
    }
}
