use yew::prelude::*;
use std::fmt;

use super::console_log;
use super::*;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Copy)]
pub struct u4 {
    v: u8
}

impl u4 {
    pub fn add(&mut self, v: u4) -> bool {
        self.v += v.v;
        if self.v >= 16 {
            self.v -= 16;
            true
        } else {
            false
        }
    }
}

impl fmt::Display for u4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04b}", self.v)
    }
}

impl fmt::Binary for u4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04b}", self.v)
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
    GotInput(usize, String),
}

impl Model {
    fn clock(&mut self) {
        self.operate(self.rom[self.pc.v as usize]);
        self.pc.add(u4 {v: 1});
    }

    fn reset(&mut self) {
        self.register_a= u4 {v: 0};
        self.register_b= u4 {v: 0};
        self.carry= false;
        self.pc= u4 {v: 0};
        self.o_port= u4 {v: 0};
        self.i_port= u4 {v: 0};
        self.rom= [0; 16];
    }

    fn operate(&mut self, r: u8) {
        let opcode  = r >> 4;
        let im = r & 0b00001111u8;
        match opcode {
            0b0011 => { // MOV A, Im
                self.register_a.v = im;
                self.carry = false;
            }
            0b0111 => { // MOV B, Im
                self.register_b.v = im;
                self.carry = false;
            }
            0b0001 => { // MOV A, B
                self.register_a.v = self.register_b.v & 0b00001111u8;
                self.carry = false;
            }
            0b0100 => { // MOV B, A
                self.register_b.v = self.register_a.v & 0b00001111u8;
                self.carry = false;
            }
            0b0000 => { // ADD A, Im
                self.carry = self.register_a.add(u4 {v: im});
            },
            0b0101 => { // ADD B, Im
                self.carry = self.register_b.add(u4 {v: im});
            },
            0b0010 => { // IN A
                self.register_a.v = self.i_port.v & 0b00001111u8;
                self.carry = false;
            },
            0b0110 => { // IN B
                self.register_b.v = self.i_port.v & 0b00001111u8;
                self.carry = false;
            },
            0b1011 => { // OUT Im
                self.o_port.v = im;
                self.carry = false;
            },
            0b1001 => { // OUT B
                self.o_port.v = self.register_b.v & 0b00001111u8;
                self.carry = false;
            },
            0b1111 => { // JMP Im
                self.pc.v = im ;
                self.carry = false;
            },
            0b1110 => { // JNC Im
                if !self.carry {
                    self.pc.v = im;
                } 
                self.carry = false;
            },
            _ => panic!("Unknown opcode")
        };
    }
}

impl Model {
    fn view_rom_item(&self, index: usize, rom: u8) -> Html {
        html! {
            <div class="td4-rom-item">
                { format!("{:04b}", index) }{": "}
                <input type="text" 
                    value=format!("{:08b}", rom)
                    oninput=self.link.callback(move |e: InputData| Msg::GotInput(index, e.value))
                    placeholder="0" />
            </div>
        }
    }

    fn view_registers(&self) -> Html {
        html! {
            <div class="td4-registers">
                <div class="td4-registers-header">
                    <p> { "Registers" } </p>
                </div>
                <div class="td4-registers-items">
                    <div class="td4-registers-item">
                      <span class="td4-registers-item-left">{ "Register A:" }</span>
                      <span class="td4-registers-item-right">{ self.register_a }</span>
                    </div>
                    <div class="td4-registers-item">
                      <span class="td4-registers-item-left">{ "Register B:" }</span>
                      <span class="td4-registers-item-right">{ self.register_b }</span>
                    </div>
                    <div class="td4-registers-item">
                      <span class="td4-registers-item-left">{ "Carry:" }</span>
                      <span class="td4-registers-item-right">{ self.carry }</span>
                    </div>
                    <div class="td4-registers-item">
                      <span class="td4-registers-item-left">{ "PC:" }</span>
                      <span class="td4-registers-item-right">{ self.pc }</span>
                    </div>
                    <div class="td4-registers-item">
                      <span class="td4-registers-item-left">{ "o_port:" }</span>
                      <span class="td4-registers-item-right">{ self.o_port }</span>
                    </div>
                    <div class="td4-registers-item">
                      <span class="td4-registers-item-left">{ "i_port:" }</span>
                      <span class="td4-registers-item-right">{ self.i_port }</span>
                    </div>
                </div>
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
            rom: [4; 16],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clock => {
                self.clock();
            }
            Msg::Reset => {
                self.reset();
            }
            Msg::GotInput(index, rom) => {
                match isize::from_str_radix(&rom.to_string(), 2) {
                    Ok(rom) => {
                        self.rom[index] = rom as u8;
                    }
                    Err(e) => {
                        console_log!("{}: {}", e, "string is allowed to 0-15");
                        self.rom[index] = 0;
                    }
                }
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
            <div class="td4">
                { self.view_registers() }
                <div class="td4-rom">
                    <div class="td4-rom-header">
                        <p>{ "ROM" }</p>
                    </div>
                    <div class="td4-rom-items">
                        { self.rom.iter().enumerate().map(|(i, rom)| self.view_rom_item(i, *rom)).collect::<Html>() }
                    </div>
                </div>
                <div class="td4-timing">
                    <div class="td4-timing-header">
                        <p>{ "Timing" }</p>
                    </div>
                    <div class="td4-timing-buttons">
                        <button class="td4-timing-button-clock" onclick=self.link.callback(|_| Msg::Clock)>{ "Clock" }</button>
                        <button class="td4-timing-button-reset" onclick=self.link.callback(|_| Msg::Reset)>{ "Reset" }</button>
                    </div>
                </div>
            </div>
        }
    }
}
