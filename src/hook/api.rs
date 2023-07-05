use std::ffi::{c_char, c_void};

type FnAmaxLogger = unsafe extern "C" fn(*const *const c_void, *const c_char, ...);
static mut FN_ORG_AMAX_LOGGER: Option<FnAmaxLogger> = None; // stores the orignal function pointer, after hook

unsafe extern "C" fn hook_amax_logger(
	param_1: *const *const c_void, // pointer to some sort of logger struct...
	fmt: *const c_char,            // format string, like for example "Hello, %s\n"
	mut fmt_args: ...              // cool rusty unstable stuffs: #![feature(c_variadic)]
) {
	// *const c_char -> &str
	if let Ok(fmt_str) = std::ffi::CStr::from_ptr(fmt).to_str() {
		let mut fmt_str: String = fmt_str.into();
		//TODO: fmt_str.split_terminator()
		while fmt_str.contains("%s") {
			let s_arg: *const c_char = fmt_args.arg();
			let s_arg = std::ffi::CStr::from_ptr(s_arg).to_str().unwrap();
			fmt_str = fmt_str.replacen("%s", s_arg, 1);
		}
		let logger_ptr_val = param_1 as usize;
		log::info!("amax_logger[0x{logger_ptr_val:08X}]: {fmt_str}\n");
	}

	/*
	if let Some(f) = FN_ORG_AMAX_LOGGER {
		// this crashes; probably due to unexpected garbage on the stack:
		f(param_1, fmt);
	};
	*/
}

pub fn set_hook(ptr_module_base: *mut c_void) {
	/// Blur.exe+0x184500: Address of the original amax_logger(...) function
	const ADDY_FN_AMAX_LOGGER: isize = 0x184500;
	let ptr_src = ptr_module_base.wrapping_offset(ADDY_FN_AMAX_LOGGER);

	let fn_ptr: *mut c_void = ptr_src as *mut _;
	let fn_hook_ptr: *mut c_void = hook_amax_logger as *mut _;
	let fn_saved: *mut *mut c_void = &mut std::ptr::null_mut();
	let v = unsafe { minhook_sys::MH_CreateHook(fn_ptr, fn_hook_ptr, fn_saved) };
	if v != minhook_sys::MH_OK {
		let v = v.to_string();
		panic!("MH_CreateHook(hook_amax_logger returned: {v}!");
	}
	unsafe {
		FN_ORG_AMAX_LOGGER = Some(*(fn_saved as *const FnAmaxLogger)); // in case we want to call the orignal
	}
	let v = unsafe { minhook_sys::MH_EnableHook(fn_ptr) };
	if v != minhook_sys::MH_OK {
		panic!("MH_EnableHook(hook_amax_logger) returned: {v}!");
	}
}
