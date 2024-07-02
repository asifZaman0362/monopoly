use std::sync::Arc;
use actix::{Actor, Context};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/*use std::io::Write;

use CellKind::*;*/

/*const CELLS: [Cell; 40] = [
    Cell {
        kind: Go,
        base_cost: None,
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Old Kent Road",
            color: Color::Brown,
            building: 0,
        }),
        base_cost: Some(60),
        owner: None,
    },
    Cell {
        kind: CommunityChest,
        base_cost: None,
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "WhiteChapel Road",
            color: Color::Brown,
            building: 0,
        }),
        base_cost: Some(60),
        owner: None,
    },
    Cell {
        kind: IncomeTax,
        base_cost: Some(200),
        owner: None,
    },
    Cell {
        kind: TrainStation("Kings Cross"),
        base_cost: Some(200),
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "The Angel IslingTon",
            color: Color::Cyan,
            building: 0,
        }),
        base_cost: Some(100),
        owner: None,
    },
    Cell {
        kind: Chance,
        base_cost: None,
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Euston Road",
            color: Color::Cyan,
            building: 0,
        }),
        base_cost: Some(100),
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "PentonVille Road",
            color: Color::Cyan,
            building: 0,
        }),
        base_cost: Some(100),
        owner: None,
    },
    Cell {
        kind: Jail,
        base_cost: None,
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Pall Mall",
            color: Color::Purple,
            building: 0,
        }),
        base_cost: Some(140),
        owner: None,
    },
    Cell {
        kind: Utility("Electric"),
        base_cost: Some(150),
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "WhiteHall",
            color: Color::Purple,
            building: 0,
        }),
        base_cost: Some(140),
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Northumrl'd Avenue",
            color: Color::Purple,
            building: 0,
        }),
        base_cost: Some(160),
        owner: None,
    },
    Cell {
        kind: TrainStation("Marylebone"),
        base_cost: Some(200),
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Bow Street",
            color: Color::Orange,
            building: 0,
        }),
        base_cost: Some(180),
        owner: None,
    },
    Cell {
        kind: CommunityChest,
        base_cost: None,
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Marlborough Street",
            color: Color::Orange,
            building: 0,
        }),
        base_cost: Some(180),
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Vine Street",
            color: Color::Orange,
            building: 0,
        }),
        base_cost: Some(200),
        owner: None,
    },
    Cell {
        kind: Parking,
        base_cost: None,
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Strand",
            color: Color::Red,
            building: 0,
        }),
        base_cost: Some(220),
        owner: None,
    },
    Cell {
        kind: Chance,
        base_cost: None,
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Fleet Street",
            color: Color::Red,
            building: 0,
        }),
        base_cost: Some(220),
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Trafalgar Square",
            color: Color::Red,
            building: 0,
        }),
        base_cost: Some(240),
        owner: None,
    },
    Cell {
        kind: TrainStation("Fenchurch St."),
        base_cost: Some(200),
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Leicester Square",
            color: Color::Yellow,
            building: 0,
        }),
        base_cost: Some(260),
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Coventry Street",
            color: Color::Yellow,
            building: 0,
        }),
        base_cost: Some(260),
        owner: None,
    },
    Cell {
        kind: Utility("Waterworks"),
        base_cost: Some(150),
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Piccadilly",
            color: Color::Yellow,
            building: 0,
        }),
        base_cost: Some(280),
        owner: None,
    },
    Cell {
        kind: GotoJail,
        base_cost: None,
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Regent Street",
            color: Color::Green,
            building: 0,
        }),
        base_cost: Some(300),
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Oxford Street",
            color: Color::Green,
            building: 0,
        }),
        base_cost: Some(300),
        owner: None,
    },
    Cell {
        kind: CommunityChest,
        base_cost: None,
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Bond Street",
            color: Color::Green,
            building: 0,
        }),
        base_cost: Some(320),
        owner: None,
    },
    Cell {
        kind: TrainStation("Liverpool St."),
        base_cost: Some(200),
        owner: None,
    },
    Cell {
        kind: Chance,
        base_cost: None,
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Park Lane",
            color: Color::Blue,
            building: 0,
        }),
        base_cost: Some(350),
        owner: None,
    },
    Cell {
        kind: SuperTax,
        base_cost: Some(100),
        owner: None,
    },
    Cell {
        kind: City(City {
            name: "Mayfair",
            color: Color::Blue,
            building: 0,
        }),
        base_cost: Some(400),
        owner: None,
    },
];*/

type Player = i32;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Color {
    Green,
    Red,
    Brown,
    Blue,
    Cyan,
    Orange,
    Yellow,
    Purple,
}

pub enum Building {
    First,
    Second,
    Third,
    Hotels,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct City {
    pub name: Arc<str>,
    pub color: Color,
    pub building: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CellKind {
    City(City),
    GotoJail,
    Jail,
    Go,
    CommunityChest,
    TrainStation(Arc<str>),
    IncomeTax,
    Utility(Arc<str>),
    SuperTax,
    Chance,
    Parking,
}

#[derive(Debug, Clone, Serialize)]
pub struct Cell {
    pub kind: CellKind,
    pub owner: Option<Player>,
    pub base_cost: Option<u16>,
    pub pos: usize,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            kind: CellKind::Jail,
            owner: None,
            base_cost: None,
            pos: 0,
        }
    }
}

impl Cell {
    pub fn calc_cost(&self, game: &Game) -> u16 {
        match self.base_cost {
            Some(cost) => {
                match &self.kind {
                    CellKind::City(city) => {
                        // complete set results in building status to be > 2, hence cost is
                        // multiplied
                        city.building as u16 * cost
                    }
                    CellKind::Utility(_) | CellKind::IncomeTax | CellKind::SuperTax => cost,
                    CellKind::TrainStation(_) => game.station_set_multiplier(&self.owner) * cost,
                    _ => cost,
                }
            }
            None => 0,
        }
    }
    pub fn update_owner(&mut self, player: Player) {
        self.owner = Some(player);
    }
}

pub struct Board {
    cells: Vec<Cell>,
    cities: Vec<usize>,
    utilities: Vec<usize>,
    stations: Vec<usize>
}

struct PlayerInfo {
    money: i32,
    owned: HashSet<usize>,
    bankrupt: bool,
}

pub struct Game {
    board: Board,
    players: HashMap<Player, PlayerInfo>,
    ids: Vec<Player>,
    set_info: HashMap<Color, Vec<usize>>,
    turn: usize,
}

impl Board {
    fn new(data: &super::Data) -> Board {
        let cells = data.cells.clone();
        let cities = data.cities.clone();
        let stations = data.stations.clone();
        let utilities = data.utilities.clone();
        Self { cells, stations, cities, utilities }
    }
}

impl Actor for Game {
    type Context = Context<Game>;
}

impl Game {
    pub fn new(data: &super::Data) -> Game {
        let board = Board::new(&data);
        let players = HashMap::new();
        let mut set_info: HashMap<Color, Vec<usize>> = HashMap::new();
        for (i, cell) in board.cells.iter().enumerate() {
            match &cell.kind {
                CellKind::City(c) => {
                    if let Some(x) = set_info.get_mut(&c.color) {
                        x.push(i)
                    } else {
                        set_info.insert(c.color, vec![i]);
                    }
                }
                _ => {}
            }
        }
        let ids = vec![];
        //serialize_cities(&board);
        //serialize_utility(&board);
        //serialize_stations(&board);
        //serialize_others(&board);
        Self {
            board,
            players,
            set_info,
            ids,
            turn: 0,
        }
    }

    fn start() {}

    fn next_turn(&self, current: usize) -> usize {
        (current + 1) % self.ids.len()
    }

    fn on_progress_turn(&mut self) -> bool {
        // make sure there are more than a single active player
        if self.players.values().filter(|x| !x.bankrupt).count() <= 1 {
            return false;
        }
        let mut turn = self.turn;
        loop {
            turn = self.next_turn(turn);
            if !self
                .players
                .get(&self.ids[turn])
                .expect("Player doesnt exist!")
                .bankrupt
            {
                break self.turn = turn;
            }
        }
        true
    }

    fn station_set_multiplier(&self, player: &Option<Player>) -> u16 {
        if let Some(player) = player {
            self.players[&player]
                .owned
                .iter()
                .filter(|x| match self.board.cells[**x].kind {
                    CellKind::TrainStation(_) => true,
                    _ => false,
                })
                .count() as u16
        } else {
            1
        }
    }

    fn on_complete_set(&mut self, c: Color) {
        for cell in self.set_info.get(&c).expect("Color set doesnt exist!") {
            match &mut self.board.cells[*cell] {
                Cell{kind: CellKind::City(City{building, ..}), ..} => {
                    *building = 2;
                }
                other => panic!("
                    Board and set info mismatch! Expected the cell at {cell} to be a city of color {c:?}, 
                    but found {other:?} instead!
                ")
            }
        }
    }

    fn is_complete_set(&self, c: Color, owner: &Player) -> bool {
        for cell in self.set_info.get(&c).unwrap() {
            if self.board.cells[*cell].owner != Some(*owner) {
                return false;
            }
        }
        true
    }
}

/*fn serialize_cities(board: &Board) {
    let cities = board
        .cells
        .iter()
        .enumerate()
        .filter_map(|(idx, cell)| match &cell.kind {
            CellKind::City(c) => Some((c.name, cell.base_cost.unwrap(), c.color, idx)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let ser = serde_json::to_string_pretty(&cities).unwrap();
    let mut f = std::fs::File::create("res/cities.json").unwrap();
    f.write(ser.as_str().as_bytes()).unwrap();
}

fn serialize_stations(board: &Board) {
    let cities = board
        .cells
        .iter()
        .enumerate()
        .filter_map(|(idx, cell)| match &cell.kind {
            CellKind::TrainStation(name) => Some((name, cell.base_cost.unwrap(), idx)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let ser = serde_json::to_string_pretty(&cities).unwrap();
    let mut f = std::fs::File::create("res/stations.json").unwrap();
    f.write(ser.as_str().as_bytes()).unwrap();
}

fn serialize_utility(board: &Board) {
    let cities = board
        .cells
        .iter()
        .enumerate()
        .filter_map(|(idx, cell)| match &cell.kind {
            CellKind::Utility(name) => Some((name, cell.base_cost.unwrap(), idx)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let ser = serde_json::to_string_pretty(&cities).unwrap();
    let mut f = std::fs::File::create("res/utilities.json").unwrap();
    f.write(ser.as_str().as_bytes()).unwrap();
}

fn serialize_others(board: &Board) {
    let cities = board
        .cells
        .iter()
        .enumerate()
        .filter_map(|(idx, cell)| match &cell.kind {
            CellKind::Utility(_) | CellKind::City(_) | CellKind::TrainStation(_) => None,
            other => Some((idx, other)),
        })
        .collect::<Vec<_>>();
    let ser = serde_json::to_string_pretty(&cities).unwrap();
    let mut f = std::fs::File::create("res/other_cells.json").unwrap();
    f.write(ser.as_str().as_bytes()).unwrap();
}*/
