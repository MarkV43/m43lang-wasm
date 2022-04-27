mod utils;

use wasm_bindgen::prelude::*;
use m43lang_visual::logic::{structure::*, interpretation::*};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
	fn alert(s: &str);

    fn prompt(s: &str) -> String;

    fn print_to_console(s: String);
}

const PROGRAM: ConstGrid<Block, 15> = ConstGrid {
    cells: [
		Some(Block::Start(Direction::Down)),
		None,
		Some(Block::Redirect(Direction::Right)),
		Some(Block::Redirect(Direction::Down)),
		Some(Block::End),
		Some(Block::Set(43)),
		None,
		Some(Block::Store),
		Some(Block::OpAdd),
		Some(Block::Print),
		Some(Block::Redirect(Direction::Right)),
		Some(Block::Display),
		Some(Block::Redirect(Direction::Up)),
		Some(Block::Redirect(Direction::Right)),
		Some(Block::Redirect(Direction::Up)),
	],
	width: 5,
	height: 3,
};

#[wasm_bindgen]
pub fn debug_mode() {
	utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn run() {
    PROGRAM.interpret(prompt, print_to_console);
}

#[wasm_bindgen]
pub fn execute_code(code: String) {
	let program = DynGrid::<Block>::from(code);
	program.interpret(prompt, print_to_console);
}

#[wasm_bindgen]
pub fn get_code_str(code: String) -> String {
	DynGrid::<Block>::from(code).as_code()
}

#[wasm_bindgen]
pub struct M43Debugger {
	debugger: GridDebugger<
		DynGrid<Block>,
		fn(&str) -> String,
		fn(String),
	>,
}

#[wasm_bindgen]
pub struct M43State {
	pub dir: char,
	pub pos: Index,
	pub val: Value,
	coords: (usize, usize),
	storage: [Value; STORAGE_SIZE],
}

#[wasm_bindgen]
impl M43State {
	pub fn get_storage(&self) -> *const Value {
		self.storage.as_ptr()
	}

	pub fn get_coords_x(&self) -> usize {
		self.coords.0
	}

	pub fn get_coords_y(&self) -> usize {
		self.coords.1
	}

	pub fn get_storage_size(&self) -> usize {
		STORAGE_SIZE
	}
}

#[wasm_bindgen]
impl M43Debugger {
	pub fn new(code: String, break_points: Vec<usize>) -> Self {
		let program = DynGrid::<Block>::from(code);
		assert_eq!(break_points.len() % 2, 0, "Breakpoints must be in pairs");
		let breaks = break_points.chunks_exact(2).map(|pair| (pair[0], pair[1])).collect();
		let debugger = GridDebugger::new(
			program,
			prompt as fn(&str) -> String,
			print_to_console as fn(String),
			breaks,
		);

		Self { debugger }
	}
	
	pub fn step(&mut self) -> Result<(), u8> {
		self.debugger.step()
	}

	pub fn run(&mut self) -> Result<(), u8> {
		self.debugger.run()
	}

	pub fn get_state(&self) -> M43State {
		M43State {
			dir: match self.debugger.state.dir {
				Direction::Up => 'U',
				Direction::Down => 'D',
				Direction::Left => 'L',
				Direction::Right => 'R',
			},
			pos: self.debugger.state.pos,
			val: self.debugger.state.val,
			storage: self.debugger.state.storage.clone(),
			coords: self.debugger.state.coords.clone(),
		}
	}
}