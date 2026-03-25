use serde::de::value;
use shared::input::ElementsFlags;

use crate::model::{
    events::{self, GameEvent},
    player::Player,
    spells::elements::{self, Element, ElementKind, flag_to_element},
    world::{self, World},
};

pub fn run(world: &mut World) {
    for player in world.players.values_mut() {
        set_elements(player);
        try_cast(player);
    }
}

fn try_cast(player: &mut Player) {

    if !player.can_cast() {
        return;
    }

    if player.intent.self_cast{

        

        return;
    }


    if player.intent.normal_cast{


        return;
    }


}

pub fn set_elements(player: &mut Player) {
    let mut incoming_elements_flags = player.intent.element_inputs.clone();

    if incoming_elements_flags.is_empty() {
        return;
    }

    player.cast.elements.reverse();

    for element in &mut player.cast.elements {
        if element.kind == ElementKind::Zero {
            continue;
        }

        if remove_conter(&element, &mut incoming_elements_flags) {
            *element = Element::new(ElementKind::Zero);
        }
    }

    player.cast.elements.reverse();

    player.cast.elements = reorder_elements(player.cast.elements);

    let mut canditate_elements = get_candidate_elements(incoming_elements_flags);

    for element in &mut player.cast.elements {
        if element.kind != ElementKind::Zero {
            continue;
        }

        if let Some(candidate_element) = canditate_elements.pop() {
            *element = candidate_element
        } else {
            break;
        }
    }

    println!(
        "{},{},{}",
        player.cast.elements[0].data.element_bits,
        player.cast.elements[1].data.element_bits,
        player.cast.elements[2].data.element_bits
    )
}

fn remove_conter(element: &Element, flags_elements: &mut ElementsFlags) -> bool {
    if flags_elements.intersects(element.data.conters_flags.clone()) {
        if element.kind == ElementKind::Lightning {
            if flags_elements.contains(ElementsFlags::WATER) {
                flags_elements.remove(ElementsFlags::WATER);
            } else if flags_elements.contains(ElementsFlags::EARTH) {
                flags_elements.remove(ElementsFlags::EARTH);
            }
        } else {
            flags_elements.remove(element.data.conters_flags.clone());
        }

        return true;
    }

    false
}

fn get_candidate_elements(flags_elements: ElementsFlags) -> Vec<Element> {
    let mut result = Vec::new();

    let mut aux_flags_elements = flags_elements.clone();

    for (_name, flag) in flags_elements.iter_names() {
        if let Some(element) = flag_to_element(&flag) {
            if element.kind == ElementKind::Shield {
                result.push(element);
                continue;
            }

            if remove_conter(&element, &mut aux_flags_elements) {
                aux_flags_elements.remove(flag);
                continue;
            }

            if !aux_flags_elements.intersects(flag) {
                continue;
            }

            result.push(element);
        }
    }

    result
}

fn reorder_elements(elements: [Element; 3]) -> [Element; 3] {
    let mut result_index = 0;

    let mut result_elements = [Element::new(ElementKind::Zero); 3];

    for element in &elements {
        if element.kind != ElementKind::Zero {
            result_elements[result_index] = *element;
            result_index += 1;
        }
    }

    result_elements
}


fn resolve_cast(  ) {
    
}