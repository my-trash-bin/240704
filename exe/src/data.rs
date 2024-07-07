use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    rc::{Rc, Weak},
};

use my_trash_bin_240704_lib::graph::{Graph, GraphDistanceF32};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationRaw {
    id: String,                          /* 역 식별자 */
    name: String,                        /* 역명 */
    line: String,                        /* 노선 */
    next_station_id: Option<String>,     /* 하행 방향 다음 역 */
    previous_station_id: Option<String>, /* 상행 방향 다음 역 */
    transfer_station_ids: Vec<String>,   /* 환승역 목록, 식별자랑 노선이 다른 동명의 식별자 목록 */
    latitude: f32,                       /* 위도 */
    longitude: f32,                      /* 경도 */
}

pub struct StationLine {
    next_station: Option<Weak<RefCell<StationInternal>>>,
    previous_station: Option<Weak<RefCell<StationInternal>>>,
    line: Line,
}

struct StationInternal {
    ids: Vec<String>,
    name: String,
    lines: HashMap<String, StationLine>,
    latitude: f32,
    longitude: f32,
}

#[derive(Clone)]
pub struct Station {
    internal: Rc<RefCell<StationInternal>>,
}

struct LineInternal {
    name: String,
    stations: Vec<Station>,
}

#[derive(Clone)]
pub struct Line {
    internal: Rc<RefCell<LineInternal>>,
}

pub struct Data {
    pub raw: Vec<StationRaw>,
    pub lines: HashMap<String, Line>,
    pub stations: HashMap<String, Station>,
    pub graph: Graph<Station, GraphDistanceF32>,
}

fn distance(a_latitude: f32, a_longitude: f32, b_latitude: f32, b_longitude: f32) -> f32 {
    fn to_radians(degrees: f32) -> f32 {
        degrees * std::f32::consts::PI / 180.0
    }

    let a_lat_rad = to_radians(a_latitude);
    let a_lon_rad = to_radians(a_longitude);
    let b_lat_rad = to_radians(b_latitude);
    let b_lon_rad = to_radians(b_longitude);

    // Haversine formula
    let delta_lat = b_lat_rad - a_lat_rad;
    let delta_lon = b_lon_rad - a_lon_rad;

    let a = (delta_lat / 2.0).sin().powi(2)
        + a_lat_rad.cos() * b_lat_rad.cos() * (delta_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    // Earth's circumference is 40000 km
    const EARTH_CIRCUMFERENCE: f32 = 40000.0;
    const EARTH_RADIUS: f32 = EARTH_CIRCUMFERENCE / (2.0 * std::f32::consts::PI);

    EARTH_RADIUS * c
}

pub fn parse_data(data: &[u8]) -> Result<Data, Box<dyn Error>> {
    let raw: Vec<StationRaw> = serde_json::from_slice(data)?;

    let mut station_map = HashMap::<String, Station>::new();
    let mut line_map = HashMap::<String, Line>::new();

    // input ids, name, latitude, longitude
    for StationRaw {
        id,
        name,
        transfer_station_ids,
        latitude,
        longitude,
        ..
    } in raw.iter()
    {
        if !station_map.contains_key(id) {
            let mut ids = vec![id.clone()];
            for transfer_station_id in transfer_station_ids {
                ids.push(transfer_station_id.clone());
            }
            let result = Station {
                internal: Rc::new(RefCell::new(StationInternal {
                    ids: ids.clone(),
                    name: name.clone(),
                    lines: HashMap::new(),
                    latitude: *latitude,
                    longitude: *longitude,
                })),
            };
            for id in ids {
                station_map.insert(id, result.clone());
            }
        }
    }

    // input StationInternal::lines, LineInternal::stations
    for StationRaw {
        id,
        line: line_name,
        next_station_id,
        previous_station_id,
        ..
    } in raw.iter()
    {
        let station = station_map.get(id).unwrap().clone();
        let next_station = next_station_id
            .clone()
            .map(|id| {
                station_map
                    .get(&id)
                    .map(|station| Rc::downgrade(&station.internal))
                    .ok_or("Invalid data: no matching next_station")
            })
            .transpose()?;
        let previous_station = previous_station_id
            .clone()
            .map(|id| {
                station_map
                    .get(&id)
                    .map(|station| Rc::downgrade(&station.internal))
                    .ok_or("Invalid data: no matching previous_station")
            })
            .transpose()?;
        let line = if let Some(line) = line_map.get(line_name) {
            line.internal.borrow_mut().stations.push(station.clone());
            line.clone()
        } else {
            let result = Line {
                internal: Rc::new(RefCell::new(LineInternal {
                    name: line_name.clone(),
                    stations: vec![station.clone()],
                })),
            };
            line_map.insert(line_name.clone(), result.clone());
            result
        };
        let station_line = StationLine {
            next_station,
            previous_station,
            line,
        };
        station
            .internal
            .borrow_mut()
            .lines
            .insert(line_name.clone(), station_line);
    }

    // fill adjacent matrix
    let values = station_map
        .iter()
        .map(|(_, station)| station.clone())
        .collect::<Vec<_>>();
    let index_map = values
        .iter()
        .enumerate()
        .flat_map(|(idx, station)| {
            station
                .internal
                .borrow()
                .ids
                .iter()
                .map(|id| (id.clone(), idx))
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<String, usize>>();
    let mut adjacent_matrix: Vec<Vec<Option<GraphDistanceF32>>> =
        vec![vec![None; values.len()]; values.len()];
    for StationRaw { id, .. } in raw.iter() {
        let from_index = *index_map.get(id).unwrap();
        let from = station_map.get(id).unwrap();
        for (
            line_name,
            StationLine {
                next_station,
                previous_station,
                ..
            },
        ) in from.internal.borrow().lines.iter()
        {
            let mut previous = from.internal.clone();
            let mut next_station = next_station.clone();
            let mut next_station_distance_sum = 0f32;
            while let Some(next) = next_station.clone() {
                let current = next.upgrade().unwrap();
                let (&current_index, next) = {
                    let current_borrow = current.borrow();
                    let previous_borrow = previous.borrow();
                    next_station_distance_sum += distance(
                        previous_borrow.latitude,
                        previous_borrow.longitude,
                        current_borrow.latitude,
                        current_borrow.longitude,
                    );
                    (
                        index_map.get(&current.borrow().ids[0]).unwrap(),
                        current
                            .borrow()
                            .lines
                            .get(line_name)
                            .unwrap()
                            .next_station
                            .clone(),
                    )
                };
                adjacent_matrix[from_index][current_index] =
                    Some(GraphDistanceF32::new(next_station_distance_sum));

                previous = current.clone();
                next_station = next;
            }

            let mut previous = from.internal.clone();
            let mut next_station = previous_station.clone();
            let mut next_station_distance_sum = 0f32;
            while let Some(next) = next_station.clone() {
                let current = next.upgrade().unwrap();
                let (&current_index, next) = {
                    let current_borrow = current.borrow();
                    let previous_borrow = previous.borrow();
                    next_station_distance_sum += distance(
                        previous_borrow.latitude,
                        previous_borrow.longitude,
                        current_borrow.latitude,
                        current_borrow.longitude,
                    );
                    (
                        index_map.get(&current.borrow().ids[0]).unwrap(),
                        current
                            .borrow()
                            .lines
                            .get(line_name)
                            .unwrap()
                            .previous_station
                            .clone(),
                    )
                };
                adjacent_matrix[from_index][current_index] =
                    Some(GraphDistanceF32::new(next_station_distance_sum));

                previous = current.clone();
                next_station = next;
            }
        }
    }

    // result
    let graph = Graph::new(values, adjacent_matrix)?;
    Ok(Data {
        raw,
        lines: line_map,
        stations: station_map,
        graph,
    })
}
