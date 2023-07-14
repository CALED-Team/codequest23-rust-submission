use crate::comms;
use crate::post_message;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::collections::HashMap;

pub struct Game {
    tank_id: String,
    objects: HashMap<String, Value>,
    width: f64,
    height: f64,
    current_turn_message: Value,
}

impl Game {
    /// Create a new `Game`, using information from the world init stage.
    pub fn new() -> Self {
        // The first message we receive will contain our tank id.
        let tank_id = comms::read_message()["message"]["your-tank-id"]
            .as_str()
            .unwrap()
            .to_owned();

        let mut objects: HashMap<String, Value> = HashMap::new();

        loop {
            // Receive the rest of the map objects...
            match comms::read_message() {
                Value::String(s) if s == comms::END_INIT_SIGNAL => {
                    break;
                }
                msg => {
                    for (key, obj) in msg["message"]["updated_objects"].as_object().unwrap() {
                        objects.insert(key.clone(), obj.clone());
                    }
                }
            }
        }

        let (width, height) = objects
            .values()
            .find_map(|obj| {
                // To allow us to safely retrieve the positions.
                #[derive(Serialize, Deserialize)]
                pub struct Boundary {
                    pub r#type: i32,
                    pub position: [[f64; 2]; 4],
                    pub velocity: [[f32; 2]; 4],
                }

                match from_value(obj.clone()) {
                    // Positions are always stored in the same order.
                    Ok(Boundary { position, .. }) => Some((position[3][0], position[3][1])),
                    _ => None,
                }
            })
            .unwrap();

        Self {
            tank_id,
            objects,
            width,
            height,
            current_turn_message: Value::Null,
        }
    }

    /// Called during the main game loop stage, here we receive information
    /// about the game's current state. Returns `false` if the game is over
    /// and true otherwise.
    pub fn read_next_turn(&mut self) -> bool {
        self.current_turn_message = comms::read_message();

        // Is the game over?
        if self.current_turn_message == Value::String(comms::END_SIGNAL.to_string()) {
            return false;
        }

        // Update our internal state to reflect game changes.
        for key in self.current_turn_message["message"]["deleted_objects"]
            .as_array()
            .unwrap()
        {
            self.objects.remove(key.as_str().unwrap());
        }

        for (key, obj) in self.current_turn_message["message"]["updated_objects"]
            .as_object()
            .unwrap()
        {
            self.objects.insert(key.clone(), obj.clone());
        }

        return true;
    }

    /// Respond to our turn with some choice of actions. In this example, we
    /// fire randomly.
    pub fn respond_to_turn(&self) {
        let mut rng = thread_rng();

        post_message!({"shoot": rng.gen_range(0.0..360.0)});
    }
}
