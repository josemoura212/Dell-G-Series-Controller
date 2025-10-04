// ELC (LED Controller) Constants
// Translated from elc_constants.py

#![allow(unused)]

// Major Commands
pub const ELC_QUERY: u8 = 0x20;
pub const USER_ANIMATION: u8 = 0x21;
pub const POWER_ANIMATION: u8 = 0x22;
pub const START_SERIES: u8 = 0x23;
pub const ADD_ACTION: u8 = 0x24;
pub const SET_EVENT: u8 = 0x25;
pub const DIMMING: u8 = 0x26;
pub const SET_COLOR: u8 = 0x27;
pub const RESET: u8 = 0x28;
pub const SPI_FLASH: u8 = 0xFF;

// ELC_QUERY Subcommands
pub const GET_VERSION: u8 = 0x00;
pub const GET_STATUS: u8 = 0x01;
pub const GET_PLATFORM: u8 = 0x02;
pub const GET_ANIMATION_COUNT: u8 = 0x03;
pub const GET_ANIMATION_BY_ID: u8 = 0x04;
pub const READ_SERIES: u8 = 0x05;

// Animation Subcommands
pub const START_NEW: u16 = 0x01;
pub const FINISH_SAVE: u16 = 0x02;
pub const FINISH_PLAY: u16 = 0x03;
pub const REMOVE: u16 = 0x04;
pub const PLAY: u16 = 0x05;
pub const SET_DEFAULT: u16 = 0x06;
pub const SET_STARTUP: u16 = 0x07;

// Action Effect codes
pub const COLOR: u8 = 0x00;
pub const PULSE: u8 = 0x01;
pub const MORPH: u8 = 0x02;

// Animation ID's
pub const AC_SLEEP: u16 = 0x5b;
pub const AC_CHARGED: u16 = 0x5c;
pub const AC_CHARGING: u16 = 0x5d;
pub const DC_SLEEP: u16 = 0x5e;
pub const DC_ON: u16 = 0x5f;
pub const DC_LOW: u16 = 0x60;
pub const DEFAULT_POST_BOOT: u16 = 0x61;
pub const RUNNING_START: u16 = 0xFFFF;
pub const RUNNING_FINISH: u16 = 0x00FF;

// Duration and tempo constants
pub const DURATION_MAX: u16 = 0xffff;
pub const DURATION_BATTERY_LOW: u16 = 0xff;
pub const DURATION_MIN: u16 = 0x00;
pub const TEMPO_MAX: u16 = 0xff;
pub const TEMPO_MIN: u16 = 0x01;

// Zones
pub const ZONES_ALL: [u8; 4] = [0, 1, 2, 3];
pub const ZONES_KB: [u8; 3] = [0, 1, 2];
pub const ZONES_NP: [u8; 1] = [3];
