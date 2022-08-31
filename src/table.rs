/// Direction Clockwise
pub const DIR_CW: u8 = 0x10;
/// Direction Counter Clockwise
pub const DIR_CCW: u8 = 0x20;

const R_START: u8 = 0x00;
const F_CW_FINAL: u8 = 0x01;
const F_CW_BEGIN: u8 = 0x02;
const F_CW_NEXT: u8 = 0x03;
const F_CCW_BEGIN: u8 = 0x04;
const F_CCW_FINAL: u8 = 0x05;
const F_CCW_NEXT: u8 = 0x06;
const H_CCW_BEGIN: u8 = 0x1;
const H_CW_BEGIN: u8 = 0x2;
const H_START_M: u8 = 0x3;
const H_CW_BEGIN_M: u8 = 0x4;
const H_CCW_BEGIN_M: u8 = 0x5;

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

pub const STATE_TABLE_HALF_STEPS: [[u8; 4]; 6] = [
    // 00        01         10           11 // BA
    [H_START_M, H_CW_BEGIN, H_CCW_BEGIN, R_START], // R_START (00)
    [H_START_M | DIR_CCW, R_START, H_CCW_BEGIN, R_START], // H_CCW_BEGIN
    [H_START_M | DIR_CW, H_CW_BEGIN, R_START, R_START], // H_CW_BEGIN
    [H_START_M, H_CCW_BEGIN_M, H_CW_BEGIN_M, R_START], // H_START_M (11)
    [H_START_M, H_START_M, H_CW_BEGIN_M, R_START | DIR_CW], // H_CW_BEGIN_M
    [H_START_M, H_CCW_BEGIN_M, H_START_M, R_START | DIR_CCW], // H_CCW_BEGIN_M
];
