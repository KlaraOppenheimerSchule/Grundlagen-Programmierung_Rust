use colored::Colorize;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    ship: bool,
    hit: Option<bool>,
}

#[derive(Debug, Clone)]
struct Player {
    grid: Vec<Vec<Point>>,
    ships: Vec<Point>,
    moves_done: i32,
    ships_left: i32,
}


#[derive(Debug, Clone)]
pub struct Game {
    ship_positions: Vec<Point>,
    players: [Player; 2],
}

impl Game {
    pub fn new(grid_size: (i32, i32)) -> Self {
        let mut player_1 = Player::new(grid_size);
        let mut player_2 = Player::new(grid_size);

        player_1.generate_ships(2);
        player_2.generate_ships(2);

        Self {
            ship_positions: Vec::new(),
            players: [player_1, player_2],
        }
    }

    pub fn start(&mut self) {
        self.game_loop();
    }

    fn check_hit(opponent_point: &mut Point) -> bool {
        if opponent_point.hit.unwrap_or_default() {
            if opponent_point.ship {
                println!("{}", "You already guessed this point, there was a ship there".red());
            } else {
                println!("{}", "You already guessed this point, it was empty".red());
            }
            return false;
        } else {
            if opponent_point.ship {
                println!("{}", "You hit a ship!".green());
                opponent_point.hit = Some(true);
                return true;
            } else {
                println!("{}", "You missed!".red());
                opponent_point.hit = Some(true);
                return false;
            }
        }
    }


    fn game_loop(&mut self) {
        use std::thread;
        use std::time::Duration;

        let mut rng = rand::thread_rng();

        loop {
            print!("{}[2J", 27 as char);

            if self.players[0].ships_left < 1 {
                println!("{}", "Player 2 WON!".green().bold().underline());

                // Print the grid a last time
                println!("\n{}", "Player 1 Grid".underline().red().bold());
                self.players[0].print_grid();
                println!("\n{}", "Player 2 Grid".underline().red().bold());
                self.players[1].print_grid();

                break;
            } else if self.players[1].ships_left < 1 {
                println!("{}", "Player 1 WON!".green().bold().underline());

                // Print the grid a last time
                println!("\n{}", "Player 1 Grid".underline().red().bold());
                self.players[0].print_grid();
                println!("\n{}", "Player 2 Grid".underline().red().bold());
                self.players[1].print_grid();

                break;
            }

            println!("{}", "Player 1's turn".underline());
            let mut guess_x = rng.gen_range(0..self.players[1].grid.len());
            let mut guess_y = rng.gen_range(0..self.players[1].grid[0].len());

            let mut opponent_point = &mut self.players[1].grid[guess_x][guess_y];
            if Self::check_hit(opponent_point) {
                self.players[1].ships_left -= 1;
            }
            self.players[0].moves_done += 1;


            println!("{}", "Player 2's turn".underline());
            guess_x = rng.gen_range(0..self.players[0].grid.len());
            guess_y = rng.gen_range(0..self.players[0].grid[0].len());

            opponent_point = &mut self.players[0].grid[guess_x][guess_y];
            if Self::check_hit(opponent_point) {
                self.players[0].ships_left -= 1;
            }
            self.players[1].moves_done += 1;



            println!("\n{}", "Player 1 Grid".underline().red().bold());
            self.players[0].print_grid();
            println!("\n{}", "Player 2 Grid".underline().red().bold());
            self.players[1].print_grid();

            thread::sleep(Duration::from_secs(2));
        }
    }
}

impl Player {
    fn generate_ships(&mut self, num_ships: i32) {
        let mut rng = rand::thread_rng();

        for _ in 0..num_ships {
            let guess_x = rng.gen_range(0..self.grid.len());
            let guess_y = rng.gen_range(0..self.grid[0].len());

            let ship: Point = Point {
                x: guess_x as i32,
                y: guess_y as i32,
                ship: true,
                hit: Some(false),
            };
            self.grid[guess_x][guess_y] = ship;
            self.ships.push(ship);
            self.ships_left += 1;
        }
    }


    pub fn print_grid(&self) {
        for line in &self.grid {
            for place in line {
                print!("|{}", {
                    if place.ship {
                        // If a point contains a ship then the hit variable can't be "None"
                        if place.hit.unwrap() {"X"} else {"S"}
                    } else if place.hit.unwrap_or_default() {"X"} else {"O"}
                });
            }
            print!("|\n");
        }
    }

    fn new(grid_size: (i32, i32)) -> Self {
        let mut tmp_grid: Vec<Vec<Point>> = Vec::new();

        for x in 0..grid_size.0 {
            let mut curr_row: Vec<Point> = Vec::new();

            for y in 0..grid_size.1 {
                curr_row.push(Point {
                    x, y,
                    ship: false,
                    hit: None,
                });
            }
            tmp_grid.push(curr_row);
        }

        Self {
            grid: tmp_grid,
            ships: vec![],
            moves_done: 0,
            ships_left: 0
        }
    }
}