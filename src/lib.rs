#![feature(c_variadic)]
#![cfg(windows)]

pub mod hook;

use std::ffi::c_void;

use crate::hook::api::set_hook;
use minhook_sys::{MH_Initialize, MH_Uninitialize, MH_OK};
use windows::{core::PCSTR, Win32::System::LibraryLoader::GetModuleHandleA};

use simplelog::*;

use blur_plugins_core::{BlurAPI, BlurEvent, BlurPlugin};

#[repr(C)]
struct MyAmaxDebugHookPlugin {}

impl BlurPlugin for MyAmaxDebugHookPlugin {
	fn name(&self) -> &'static str {
		"MyAmaxDebugHookPlugin"
	}

	fn on_event(&self, _event: &BlurEvent) {}

	fn free(&self) {
		let r = unsafe { MH_Uninitialize() };
		if r != MH_OK {
			log::error!("minhook_sys::MH_Uninitialize() returns {r}");
		}
	}
}

#[no_mangle]
fn plugin_init(_api: &mut dyn BlurAPI) -> Box<dyn BlurPlugin> {
	let plugin = MyAmaxDebugHookPlugin {};

	let cfg = ConfigBuilder::new()
		.set_time_offset_to_local()
		.unwrap()
		.build();
	let log_file = blur_plugins_core::create_log_file("amax_logger_hook.log").unwrap();
	CombinedLogger::init(vec![
		TermLogger::new(
			LevelFilter::Trace,
			cfg,
			TerminalMode::Mixed,
			ColorChoice::Auto,
		),
		WriteLogger::new(LevelFilter::Trace, Config::default(), log_file),
	])
	.unwrap();
	log_panics::init();

	let ptr_base: *mut c_void = unsafe { GetModuleHandleA(PCSTR::null()) }.unwrap().0 as _;
	let r = unsafe { MH_Initialize() };
	if r != MH_OK {
		log::error!("minhook_sys::MH_Initialize() returns {r}");
	}
	set_hook(ptr_base);

	Box::new(plugin)
}
