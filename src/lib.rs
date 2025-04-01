use mames_pp::any::DifficultyAttributes;
use mames_pp::{
    Difficulty, Performance, Beatmap, any::PerformanceAttributes
};

use interoptopus::{
    extra_type, ffi_function, ffi_type, function, patterns::option::FFIOption, Inventory,
    InventoryBuilder,
};
use std::ffi::CStr;
use std::os::raw::c_char;

#[ffi_type]
#[repr(C)]
#[derive(Clone, Default, PartialEq)]
pub struct CalculatePerformanceResult {
    pub pp: f64,
    pub stars: f64,
}

impl std::fmt::Display for CalculatePerformanceResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("CalculateResult");
        s.field("pp", &self.pp).field("stars", &self.stars);

        s.finish()
    }
}

impl CalculatePerformanceResult {
    fn from_attributes(perf: PerformanceAttributes, diff: DifficultyAttributes) -> Self {
        Self {
            pp: perf.pp(),
            stars: diff.stars(),
        }
    }
}

#[ffi_function]
#[no_mangle]
pub unsafe extern "C" fn calculate_score(
    beatmap_path: *const c_char,
    mode: u32,
    mods: u32,
    max_combo: u32,
    accuracy: f64,
    miss_count: u32,
    passed_objects: FFIOption<u32>,
) -> CalculatePerformanceResult {
    let beatmap_path = CStr::from_ptr(beatmap_path).to_str().unwrap();
    let beatmap = Beatmap::from_path(beatmap_path).unwrap();

    let difficulty = Difficulty::new().mods(mods).calculate(&beatmap);

    let mut performance = Performance::new(&beatmap);
    performance = performance
        .mods(mods)
        .combo(max_combo)
        .misses(miss_count);

    if let Some(passed_objects) = passed_objects.into_option() {
        performance = performance.passed_objects(passed_objects as u32);
    }
    
    performance = performance.accuracy(accuracy);
    let performance_result = performance.calculate();

    CalculatePerformanceResult::from_attributes(performance_result, difficulty)
}

#[ffi_function]
#[no_mangle]
pub unsafe extern "C" fn calculate_score_bytes(
    beatmap_bytes: *const u8, 
    len: u32,
    mode: u32,
    mods: u32,
    max_combo: u32,
    accuracy: f64,
    miss_count: u32,
    passed_objects: FFIOption<u32>,
) -> CalculatePerformanceResult {
    let bytes = std::slice::from_raw_parts(beatmap_bytes, len as usize);
    let beatmap = Beatmap::from_bytes(bytes).unwrap();

    let difficulty = Difficulty::new().mods(mods).calculate(&beatmap);

    let mut performance = Performance::new(&beatmap);
    performance = performance
        .mods(mods)
        .combo(max_combo)
        .misses(miss_count);

    if let Some(passed_objects) = passed_objects.into_option() {
        performance = performance.passed_objects(passed_objects as u32);
    }
    
    performance = performance.accuracy(accuracy);
    let performance_result = performance.calculate();

    CalculatePerformanceResult::from_attributes(performance_result, difficulty)
}

pub fn my_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(extra_type!(CalculatePerformanceResult))
        .register(function!(calculate_score))
        .inventory()
}
