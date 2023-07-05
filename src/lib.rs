#![feature(c_variadic)]
#![cfg(windows)]

pub mod dll;
pub mod hook;

use std::ffi::c_void;

use crate::hook::api::set_hook;
use minhook_sys::{MH_Initialize, MH_Uninitialize, MH_OK};
use windows::{
	core::PCSTR,
	Win32::{Foundation::HMODULE, System::LibraryLoader::GetModuleHandleA},
};

use simplelog::*;

pub fn init(module: HMODULE) {
	let cfg = ConfigBuilder::new()
		.set_time_offset_to_local()
		.unwrap()
		.build();

	CombinedLogger::init(vec![
		TermLogger::new(
			LevelFilter::Trace,
			cfg,
			TerminalMode::Mixed,
			ColorChoice::Auto,
		),
		WriteLogger::new(
			LevelFilter::Trace,
			Config::default(),
			std::fs::File::create(".\\amax\\log\\amax_logger_hooks.log")
				.expect("Couldn't create log file: .\\amax\\log\\amax_logger_hooks.log"),
		),
	])
	.unwrap();
	log_panics::init();
	log::info!("amax_logger_hooks: init @ {module:X?}");

	let ptr_base: *mut c_void = unsafe { GetModuleHandleA(PCSTR::null()) }.unwrap().0 as _;
	let r = unsafe { MH_Initialize() };
	if r != MH_OK {
		log::error!("minhook_sys::MH_Initialize() returns {r}");
	}
	set_hook(ptr_base);
	log::info!("amax_logger_hooks: init -- done!");
}

pub fn free(module: HMODULE) {
	let r = unsafe { MH_Uninitialize() };
	if r != MH_OK {
		log::error!("minhook_sys::MH_Uninitialize() returns {r}");
	}
	log::info!("amax_logger_hooks: free @ {module:X?}");
}
