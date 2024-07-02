use std::sync::Arc;
use game::{Cell, CellKind, City, Color};
use std::io::Read;

mod game;
mod rng;
mod http;

struct Data {
    cells: Vec<Cell>,
    cities: Vec<usize>,
    utilities: Vec<usize>,
    stations: Vec<usize>,
}

struct Config {
    prob_payall: f32,
    prob_goto_city: f32,
    prob_goto_train: f32,
}

type _City = (String, u16, Color, usize);
type _Utility = (String, u16, usize);
type _Station = (String, u16, usize);

#[actix::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let data = load_cell_data()?;
    http::start(data).await
}


fn load_cell_data() -> std::io::Result<Data> {
    let mut cells = vec![];
    let mut cities = vec![];
    let mut utilities = vec![];
    let mut stations = vec![];
    {
        let mut file = std::fs::File::open("res/cities.json")?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let _cities: Vec<_City> = serde_json::from_str(&buffer).unwrap();
        for (name, base_cost, color, pos) in _cities {
            cities.push(pos);
            cells.push(Cell {
                kind: CellKind::City(City {
                    name: Arc::from(name),
                    color,
                    building: 0,
                }),
                owner: None,
                base_cost: Some(base_cost),
                pos,
            });
        }
    }
    {
        let mut file = std::fs::File::open("res/utilities.json")?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let _utilities: Vec<_Utility> = serde_json::from_str(&buffer).unwrap();
        for (name, base_cost, pos) in _utilities {
            utilities.push(pos);
            cells.push(Cell {
                kind: CellKind::Utility(Arc::from(name)),
                owner: None,
                base_cost: Some(base_cost),
                pos,
            });
        }
    }
    {
        let mut file = std::fs::File::open("res/stations.json")?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let _stations: Vec<_Station> = serde_json::from_str(&buffer).unwrap();
        for (name, base_cost, pos) in _stations {
            stations.push(pos);
            cells.push(Cell {
                kind: CellKind::TrainStation(Arc::from(name)),
                owner: None,
                base_cost: Some(base_cost),
                pos,
            });
        }
    }
    {
        let mut file = std::fs::File::open("res/other_cells.json")?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let _others: Vec<(usize, CellKind)> = serde_json::from_str(&buffer).unwrap();
        for (pos, kind) in _others {
            cells.push(Cell {
                kind,
                owner: None,
                base_cost: None,
                pos,
            });
        }
    }
    cells.sort_by(|x, y| x.pos.cmp(&y.pos));
    Ok(Data {
        cells,
        cities,
        utilities,
        stations,
    })
}
