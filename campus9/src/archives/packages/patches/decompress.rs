use dynlib::{VoidPtr,LoadWinDynLib,DynLibWin};
use std::alloc::Layout;
use crate::archives::packages::patches::{block::MAX_BLOCK_LENGTH};

type OodleLZDecompress = fn(
    input: *mut u8,    // buffer
    input_size: u64,    // buffer_size
    output: *mut u8,    // result
    output_size: u64,    // result_buffer_size
    u32,    // a
    u32,    // b
    u32,    // c
    u64,    // d
    u64,    // e
    u64,    // f
    u64,    // g
    u64,    // h
    u64,    // i
    u32     // ThreadModule
) -> u64;


struct OodleGuts {
    pub lib: DynLibWin,
    pub func_ptr: VoidPtr,
    pub callable: OodleLZDecompress
}

struct OodleGutsHolder {
    pub initialized: bool,
    pub guts: Option<OodleGuts>
}

const HOLDER: OodleGutsHolder = OodleGutsHolder {
    initialized: false,
    guts: None
};


// private static extern long OodleLZ_Decompress(byte[] buffer, long bufferSize, byte[] result, long outputBufferSize, int a, int b, int c, long d, long e, long f, long g, long h, long i, int ThreadModule);
pub struct Oodle {}
impl Oodle {
    pub fn init(path: &str) {
        if HOLDER.initialized {
            return;
        }
        let lib = LoadWinDynLib::new().load(path).unwrap();
        let func_ptr = lib.load_function("OodleLZ_Decompress").unwrap();
        let callable: extern "Rust" fn(
            *mut u8,    // buffer
            u64,    // buffer_size
            *mut u8,    // result
            u64,    // result_buffer_size
            u32,    // a
            u32,    // b
            u32,    // c
            u64,    // d
            u64,    // e
            u64,    // f
            u64,    // g
            u64,    // h
            u64,    // i
            u32     // ThreadModule
        ) -> u64 = unsafe { std::mem::transmute(func_ptr) };
        HOLDER.guts = Some(OodleGuts {
            lib,
            func_ptr,
            callable
        });
        HOLDER.initialized = true;
    }

    pub fn lz_decompress(mut data: Vec<u8>) -> Vec<u8> {
        let target: *mut u8;
        let bytes_decompressed: u64;
        unsafe {
            target = std::alloc::alloc(Layout::array::<u8>(MAX_BLOCK_LENGTH).unwrap());
            let data_ptr = data.as_mut_ptr();
            bytes_decompressed = (HOLDER.guts.unwrap().callable)(
                data_ptr,
                data.len() as u64,
                target,
                MAX_BLOCK_LENGTH as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0
            );
            Vec::<u8>::from_raw_parts(target, bytes_decompressed as usize, MAX_BLOCK_LENGTH)
        }
    }
}