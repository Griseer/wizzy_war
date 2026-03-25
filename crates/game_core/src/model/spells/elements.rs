use std::collections::HashMap;

use bitflags::Flags;
use shared::input::ElementsFlags;

#[repr(u8)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
pub enum ElementKind {
    Zero,
    Water,
    Life,
    Shield,
    Cold,
    Lightning,
    Arcane,
    Earth,
    Fire,
}

#[derive(Copy,Clone)]
pub struct Element{
    pub kind: ElementKind,
    pub data: ElementData

}


impl Element {
    pub fn new(kind:ElementKind) -> Element{
        let data = element_data(&kind);
        Element{
            kind,
            data
        }
    }



}

#[derive(Copy, Clone)]
pub struct ElementData {
    pub conters_flags: ElementsFlags,
    pub element_bits: u16,

}

pub fn element_data(e: &ElementKind) -> ElementData {
    match e {
        &ElementKind::Zero => ElementData {
            conters_flags: ElementsFlags::empty(),
            element_bits: ElementsFlags::empty().bits()
        },
        &ElementKind::Fire => ElementData {
            conters_flags: ElementsFlags::COLD,
            element_bits: ElementsFlags::FIRE.bits()
        },
        &ElementKind::Water => ElementData {
            conters_flags: ElementsFlags::LIGHTNING,
            element_bits: ElementsFlags::WATER.bits()
        },
        &ElementKind::Life => ElementData {
            conters_flags: ElementsFlags::ARCANE,
            element_bits: ElementsFlags::LIFE.bits()
        },
        &ElementKind::Arcane => ElementData {
            conters_flags: ElementsFlags::LIFE,
            element_bits: ElementsFlags::ARCANE.bits()
        },
        &ElementKind::Shield => ElementData {
            conters_flags: ElementsFlags::SHIELD,
            element_bits: ElementsFlags::SHIELD.bits()
        },
        &ElementKind::Cold => ElementData {
            conters_flags: ElementsFlags::FIRE,
            element_bits: ElementsFlags::COLD.bits()
        },
        &ElementKind::Lightning => ElementData {
            conters_flags: ElementsFlags::WATER.union(ElementsFlags::EARTH),
            element_bits: ElementsFlags::LIGHTNING.bits()
        },
        &ElementKind::Earth => ElementData {
            conters_flags: ElementsFlags::LIGHTNING,
            element_bits: ElementsFlags::EARTH.bits()
        },
    }
}

pub fn flag_to_element(e: &ElementsFlags) -> Option<Element> {
    match e {
        &ElementsFlags::FIRE => Some(Element::new(ElementKind::Fire)),
        &ElementsFlags::WATER => Some(Element::new(ElementKind::Water)),
        &ElementsFlags::LIFE => Some(Element::new(ElementKind::Life)),
        &ElementsFlags::ARCANE => Some(Element::new(ElementKind::Arcane)),
        &ElementsFlags::SHIELD => Some(Element::new(ElementKind::Shield)),
        &ElementsFlags::COLD => Some(Element::new(ElementKind::Cold)),
        &ElementsFlags::LIGHTNING => Some(Element::new(ElementKind::Lightning)),
        &ElementsFlags::EARTH => Some(Element::new(ElementKind::Earth)),
        _ =>  None,
    }
}

