use lazy_static::lazy_static;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::os::raw;
use std::thread::sleep;
use std::time::{Duration, Instant};

const DOOMGENERIC_RESX: usize = 640;
const DOOMGENERIC_RESY: usize = 480;

extern "C" {
    fn D_DoomMain(); // doomgeneric.h
    fn M_FindResponseFile(); // used in main of i_main.c
}

#[no_mangle]
static mut DG_ScreenBuffer: *const u32 = std::ptr::null();
//static DG_ScreenBuffer: &[u32] = &[0u32; DOOMGENERIC_RESX * DOOMGENERIC_RESY];

lazy_static! {
    static ref START_TIME: Instant = Instant::now();
    static ref SCREEN_BUFFER: std::sync::Mutex<Option<Box<[u32]>>> = std::sync::Mutex::new(None);
}

#[no_mangle]
extern "C" fn DG_Init() {
    println!("DG_Init()");
    println!("Passed millis: {}", DG_GetTicksMs());

    *SCREEN_BUFFER.lock().unwrap() = Some(Box::new([0u32; DOOMGENERIC_RESX * DOOMGENERIC_RESY]));
    unsafe {
        // Setting DG_ScreenBuffer to where the new buffer is
        DG_ScreenBuffer = SCREEN_BUFFER.lock().unwrap().as_ref().unwrap().as_ptr();
    }
}

#[no_mangle]
extern "C" fn DG_GetKey(pressed: *mut raw::c_int, char: raw::c_uchar) -> raw::c_int {
    println!("DG_GetKey({:?}, {:?})", pressed, char);
    0
}

#[no_mangle]
extern "C" fn DG_GetTicksMs() -> u32 {
    let tick_ms = u32::try_from(START_TIME.elapsed().as_millis())
        .expect("Can't fit passed milliseconds into u32!");
    println!("DG_GetTicksMs() -> {:?}", tick_ms);
    tick_ms
}

#[no_mangle]
extern "C" fn DG_SleepMs(ms: u32) {
    println!("DG_SleepMs({:?})", ms);
    sleep(Duration::from_millis(ms as u64));
}

#[no_mangle]
extern "C" fn DG_DrawFrame() {
    println!("DG_DrawFrame()");
}

#[no_mangle]
extern "C" fn DG_SetWindowTitle(title: *const raw::c_char) {
    let title = unsafe { CStr::from_ptr(title) }
        .to_str()
        .expect("Can't convert title c string to rust string");
    println!("DG_SetWindowTitle({:?})", title);
}

pub fn init() {
    unsafe {
        M_FindResponseFile();
        DG_Init();
        D_DoomMain();
    }
}
