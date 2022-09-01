use super::{DIR_CCW, DIR_CW};

const R_START: u8 = 0x00;
const F_CW_FINAL: u8 = 0x01;
const F_CW_BEGIN: u8 = 0x02;
const F_CW_NEXT: u8 = 0x03;
const F_CCW_BEGIN: u8 = 0x04;
const F_CCW_FINAL: u8 = 0x05;
const F_CCW_NEXT: u8 = 0x06;

pub const STATE_TABLE_FULL_STEPS: [[u8; 4]; 7] = [
    // 00     01          10           11 // BA
    [R_START, F_CW_BEGIN, F_CCW_BEGIN, R_START], // R_START
    [F_CW_NEXT, R_START, F_CW_FINAL, R_START | DIR_CW], // F_CW_FINAL
    [F_CW_NEXT, F_CW_BEGIN, R_START, R_START],   // F_CW_BEGIN
    [F_CW_NEXT, F_CW_BEGIN, F_CW_FINAL, R_START], // F_CW_NEXT
    [F_CCW_NEXT, R_START, F_CCW_BEGIN, R_START], // F_CCW_BEGIN
    [F_CCW_NEXT, F_CCW_FINAL, R_START, R_START | DIR_CCW], // F_CCW_FINAL
    [F_CCW_NEXT, F_CCW_FINAL, F_CCW_BEGIN, R_START], // F_CCW_NEXT
];
