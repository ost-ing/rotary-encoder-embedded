use super::{DIR_CCW, DIR_CW};

const R_START: u8 = 0x00;
const H_CCW_BEGIN: u8 = 0x1;
const H_CW_BEGIN: u8 = 0x2;
const H_START_M: u8 = 0x3;
const H_CW_BEGIN_M: u8 = 0x4;
const H_CCW_BEGIN_M: u8 = 0x5;

pub const STATE_TABLE_HALF_STEPS: [[u8; 4]; 6] = [
    // 00        01         10           11 // BA
    [H_START_M, H_CW_BEGIN, H_CCW_BEGIN, R_START], // R_START (00)
    [H_START_M | DIR_CCW, R_START, H_CCW_BEGIN, R_START], // H_CCW_BEGIN
    [H_START_M | DIR_CW, H_CW_BEGIN, R_START, R_START], // H_CW_BEGIN
    [H_START_M, H_CCW_BEGIN_M, H_CW_BEGIN_M, R_START], // H_START_M (11)
    [H_START_M, H_START_M, H_CW_BEGIN_M, R_START | DIR_CW], // H_CW_BEGIN_M
    [H_START_M, H_CCW_BEGIN_M, H_START_M, R_START | DIR_CCW], // H_CCW_BEGIN_M
];
