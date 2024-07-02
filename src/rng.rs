use fastrand;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;

#[derive(Debug, Serialize)]
pub enum GoPassAction {
    Collect,
    Skip,
}

#[derive(Debug, Serialize)]
pub enum Target {
    Cell(usize),
    Go,
    Back(usize),
    Ahead(usize),
}

#[derive(Eq, PartialEq, Hash, Deserialize)]
enum ChanceType {
    Move,
    Gain,
    Loss,
    GotoJail,
    GetOutOfJailFree,
    Repairs,
}

#[derive(Debug, Serialize)]
pub enum Payee {
    Bank,
    Players,
}

#[derive(Debug, Serialize)]
pub enum Amount {
    Original,
    Roll,
}

#[derive(Debug, Serialize)]
pub enum RandomCard {
    Move {
        target: Target,
        go_pass_action: GoPassAction,
        money: Amount,
        multipler: u16,
    },
    Gain {
        amount: u16,
        text: String,
    },
    Loss {
        amount: u16,
        text: String,
        pay_to: Payee,
    },
    GotoJail,
    GetOutOfJailFree,
    Repairs {
        hotel: u16,
        house: u16,
        text: String,
    },
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
enum LabelFor {
    Loss,
    PayPlayers,
    Gain,
    Repairs,
}

type Labels = HashMap<LabelFor, Vec<String>>;

pub fn load_labels() -> std::io::Result<Labels> {
    let mut labels = Labels::new();
    let mut file = std::fs::File::open("res/labels.json")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let data: Vec<(LabelFor, Vec<String>)> = serde_json::from_str(&buf).unwrap();
    for (label_for, list) in data {
        labels.insert(label_for, list);
    }
    Ok(labels)
}

type ChanceTable = Vec<(ChanceType, f32)>;

pub fn load_chance_table() -> std::io::Result<ChanceTable> {
    let mut table = vec![];
    let mut file = std::fs::File::open("res/chance_table.json")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let data: Vec<(ChanceType, f32)> = serde_json::from_str(&buf).unwrap();
    let mut acc = 0.0;
    for (chance_type, chance) in data {
        table.push((chance_type, acc + chance));
        acc += chance;
    }
    Ok(table)
}

pub fn get_random_community_chest_card(
    labels: &Labels,
    chance_table: &ChanceTable,
    config: &super::Config,
    data: &super::Data,
) -> RandomCard {
    let probability = fastrand::f32() * 100.0;
    let mut i = 0;
    let chance_type = loop {
        match chance_table.get(i) {
            Some((t, chance)) => {
                if probability <= *chance {
                    break t;
                } else {
                    i += 1;
                }
            }
            None => break &chance_table.last().expect("Table is empty!").0,
        }
    };
    use ChanceType::*;
    match chance_type {
        GotoJail => RandomCard::GotoJail,
        GetOutOfJailFree => RandomCard::GetOutOfJailFree,
        Move => get_random_move_card(config, data),
        Gain => get_random_gain_card(labels),
        Loss => get_random_loss_card(labels),
        Repairs => get_random_repairs_card(labels),
    }
}

fn get_random_loss_card(labels: &Labels) -> RandomCard {
    if fastrand::f32() > 0.8 {
        let l = labels
            .get(&LabelFor::PayPlayers)
            .expect("Table doesnt contain entry for this type");
        let text = l[fastrand::usize(0..l.len())].clone();
        RandomCard::Loss {
            pay_to: Payee::Players,
            amount: fastrand::u16(100..250),
            text,
        }
    } else {
        let l = labels
            .get(&LabelFor::Loss)
            .expect("Table doesnt contain entry for this type");
        let text = l[fastrand::usize(0..l.len())].clone();
        RandomCard::Loss {
            pay_to: Payee::Bank,
            amount: fastrand::u16(25..100),
            text,
        }
    }
}

fn get_random_gain_card(labels: &Labels) -> RandomCard {
    let l = labels
        .get(&LabelFor::Gain)
        .expect("Table doesnt contain entry for this type");
    let text = l[fastrand::usize(0..l.len())].clone();
    RandomCard::Gain {
        amount: fastrand::u16(25..100),
        text,
    }
}

fn get_random_move_card(config: &super::Config, data: &super::Data) -> RandomCard {
    let p = fastrand::f32() * 100.0;
    if p < config.prob_goto_city {
        let c = fastrand::usize(0..data.cities.len());
        RandomCard::Move {
            target: Target::Cell(data.cities[c]),
            go_pass_action: GoPassAction::Collect,
            money: Amount::Original,
            multipler: 1,
        }
    } else if p < config.prob_goto_train {
        let t = fastrand::usize(0..data.stations.len());
        RandomCard::Move {
            target: Target::Cell(data.stations[t]),
            go_pass_action: GoPassAction::Collect,
            money: Amount::Original,
            multipler: 2,
        }
    } else {
        let u = fastrand::usize(0..data.utilities.len());
        RandomCard::Move {
            target: Target::Cell(data.utilities[u]),
            go_pass_action: GoPassAction::Collect,
            money: Amount::Roll,
            multipler: 10,
        }
    }
}

fn get_random_repairs_card(labels: &Labels) -> RandomCard {
    let l = labels
        .get(&LabelFor::Repairs)
        .expect("Label doesnt exist for this type");
    let text = l[fastrand::usize(0..l.len())].clone();
    let hotel = fastrand::u16(50..150);
    let house = fastrand::u16(25..75);
    RandomCard::Repairs { hotel, house, text }
}
