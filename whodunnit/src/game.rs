#[allow(dead_code)]

use std::io;
use std::io::Write;
use std::fs;
use std::fs::{File, OpenOptions};
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;
use std::error::Error;

use rand::prelude::IndexedRandom;

mod utils;
mod plhandler;
use plhandler::PLHandler;


#[derive(Clone, Debug, Copy)]
pub enum GameModes {
    Menu,
    Loop,
}

#[derive(Debug)]
pub struct Game {
    mode: GameModes,
    pl_handler: PLHandler,
    is_running: bool,
    last_user_input: String,
    knowledge_base: File,
    notes: Vec<String>,
    suspects: Vec<String>,
    weapons: Vec<String>,
    locations: Vec<String>
}

impl Game {
    pub fn new() -> Result<Self, std::io::Error> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let kb_path = format!("tmp/game-{}.pl", time);
        println!("{}", &kb_path);
        let mut kb_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&kb_path).expect("someshit");

        let suspects = fs::read_to_string(Path::new("game/suspects.txt"))
            .expect("there should be suspects")
            .lines()
            .map(|x| {
                let str = format!("suspect({}).\n", x);
                kb_file.write_all(str.as_bytes()).expect("something fishy");
                x
            })
            .map(String::from).collect::<Vec<String>>();

        let weapons = fs::read_to_string(Path::new("game/weapons.txt"))
            .expect("there should be weapons")
            .lines()
            .map(|x| {
                let str = format!("weapon({}).\n", x);
                kb_file.write_all(str.as_bytes()).expect("something fishy");
                x
            })
            .map(String::from).collect::<Vec<String>>();

        let locations = fs::read_to_string(Path::new("game/locations.txt"))
            .expect("there should be locations")
            .lines()
            .map(|x| {
                let str = format!("location({}).\n", x);
                kb_file.write_all(str.as_bytes()).expect("something fishy");
                x
            })
            .map(String::from).collect::<Vec<String>>();

        let _preds = fs::read_to_string(Path::new("game/predicates.txt"))
            .expect("there should be predicates")
            .lines()
            .map(|x| {
                let str = format!("{}\n", x);
                kb_file.write_all(str.as_bytes()).expect("something fishy");
                x
            })
            .map(String::from).collect::<Vec<String>>();

        let murderer = suspects.choose(&mut rand::rng()).unwrap();
        let m_weapon = weapons.choose(&mut rand::rng()).unwrap();
        let m_location = locations.choose(&mut rand::rng()).unwrap();

        kb_file.write_all(format!("murderer({}).\n", murderer).as_bytes())?;
        kb_file.write_all(format!("murder_weapon({}).\n", m_weapon).as_bytes())?;
        kb_file.write_all(format!("murder_location({}).\n", m_location).as_bytes())?;


        let debug_path = "debug.txt";

        let pl_handler = PLHandler::new(debug_path, &kb_path)?;

        Ok(Self {
            mode: GameModes::Menu,
            pl_handler,
            is_running: true,
            last_user_input: String::new(),
            knowledge_base: kb_file,
            notes: vec!(),
            suspects,
            weapons,
            locations
        })
    }
}

impl Game {

    pub fn start(&mut self) -> Result<(), Box<dyn Error>>{
        while self.is_running() {
            match self.get_mode() {
                GameModes::Menu => {
                    println!("This is the menu");
                    self.prompt()?;
                    self.check_input()?;
                },
                GameModes::Loop => {
                    self.pl_handler.prompt_goal()?;
                    self.check_game()?;
                    self.pl_handler.query_prolog()?;
                    let outcome = self.pl_handler.read_output()?;

                    println!("the outcome is: {:?}", outcome);
                }
            }
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.is_running
    }

    fn get_mode(&self) -> GameModes {
        self.mode
    }

    fn check_game(&mut self) -> Result<(), std::io::Error> {
        match self.pl_handler.get_current_goal() {
            "stop" | "menu" => self.set_mode(GameModes::Menu),
            _ => Ok(())
        }
    }

    fn get_last_input(&self) -> String {
        self.last_user_input.clone()
    }

    fn check_input(&mut self) -> Result<(), std::io::Error> {
        match self.get_last_input().as_str() {
            "loop" => self.set_mode(GameModes::Loop),
            "exit" => {
                self.is_running = false;
                Ok(())
            },
            "menu" => self.set_mode(GameModes::Menu),
            _ => Ok(())
        }
    }
    fn set_mode(&mut self, mode: GameModes) -> Result<(), std::io::Error> {
        self.mode = mode;
        Ok(())
    }

    fn prompt(&mut self) -> Result<(), std::io::Error> {
        let mut buffer = Default::default();
        println!("What do you want to do?");
        println!("'loop' for starting a game");
        println!("'exit' for closing this program");
        let _prompt = io::stdin().read_line(&mut buffer)?;
        self.last_user_input = buffer.trim().to_string();
        Ok(())
    }

}
