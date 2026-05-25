pub mod ad {
    pub const AMP_TYPE: [u8; 3] = [0x00, 0x00, 0x21];
    pub const AMP_GAIN: [u8; 3] = [0x00, 0x00, 0x22];
    pub const AMP_BASS: [u8; 3] = [0x00, 0x00, 0x24];
    pub const AMP_MID: [u8; 3] = [0x00, 0x00, 0x25];
    pub const AMP_TREB: [u8; 3] = [0x00, 0x00, 0x26];
    pub const AMP_PRES: [u8; 3] = [0x00, 0x00, 0x27];
    pub const AMP_VOL: [u8; 3] = [0x00, 0x00, 0x28];
    pub const AMP_SAG: [u8; 3] = [0x00, 0x00, 0x2A];
    pub const AMP_RES: [u8; 3] = [0x00, 0x00, 0x2B];

    pub const BST_ON: [u8; 3] = [0x00, 0x00, 0x10];
    pub const BST_TYPE: [u8; 3] = [0x00, 0x00, 0x11];
    pub const BST_DRIVE: [u8; 3] = [0x00, 0x00, 0x12];
    pub const BST_BOTTOM: [u8; 3] = [0x00, 0x00, 0x13];
    pub const BST_TONE: [u8; 3] = [0x00, 0x00, 0x14];
    pub const BST_LEVEL: [u8; 3] = [0x00, 0x00, 0x17];

    pub const MOD_ON: [u8; 3] = [0x00, 0x01, 0x00];
    pub const MOD_TYPE: [u8; 3] = [0x00, 0x01, 0x01];

    pub const FX_ON: [u8; 3] = [0x00, 0x03, 0x00];
    pub const FX_TYPE: [u8; 3] = [0x00, 0x03, 0x01];

    pub const DLY1_ON: [u8; 3] = [0x00, 0x05, 0x00];
    pub const DLY1_TYPE: [u8; 3] = [0x00, 0x05, 0x01];
    pub const DLY1_TIME: [u8; 3] = [0x00, 0x05, 0x02];
    pub const DLY1_FB: [u8; 3] = [0x00, 0x05, 0x04];
    pub const DLY1_HICUT: [u8; 3] = [0x00, 0x05, 0x05];
    pub const DLY1_LVL: [u8; 3] = [0x00, 0x05, 0x06];

    pub const REV_ON: [u8; 3] = [0x00, 0x05, 0x40];
    pub const REV_TYPE: [u8; 3] = [0x00, 0x05, 0x41];
    pub const REV_TIME: [u8; 3] = [0x00, 0x05, 0x42];
    pub const REV_PRE: [u8; 3] = [0x00, 0x05, 0x43];
    pub const REV_DENS: [u8; 3] = [0x00, 0x05, 0x47];
    pub const REV_LVL: [u8; 3] = [0x00, 0x05, 0x48];

    pub const NS_ON: [u8; 3] = [0x00, 0x05, 0x66];
    pub const NS_THR: [u8; 3] = [0x00, 0x05, 0x67];
    pub const NS_REL: [u8; 3] = [0x00, 0x05, 0x68];
}

pub const AMP_TYPES: &[&str] = &[
    "NATURAL CLEAN", "[ACOUSTIC]", "COMBO CRUNCH", "STACK CRUNCH",
    "HIGAIN STACK", "POWER DRIVE", "EXTREME LEAD", "CORE METAL",
    "[CLEAN]", "CLEAN TWIN", "PRO CRUNCH", "[CRUNCH]",
    "DELUXE CRUNCH", "VO DRIVE", "VO LEAD", "MATCH DRIVE",
    "BG LEAD", "BG DRIVE", "MS-1959 I", "MS-1959 I+II",
    "R-FIER VINTAGE", "R-FIER MODERN", "T-AMP LEAD", "[BROWN]",
    "[LEAD]",
    "VAR [ACOUSTIC]", "VAR [CLEAN]", "VAR [CRUNCH]", "VAR [LEAD]", "VAR [BROWN]",
];

pub const AMP_TYPE_VALUES: &[u8] = &[
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B,
    0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x1C, 0x1D, 0x1E, 0x1F, 0x20,
];

pub const BOOSTER_TYPES: &[&str] = &[
    "MID BOOST", "CLEAN BOOST", "TREBLE BOOST", "CRUNCH OD", "NATURAL OD",
    "WARM OD", "FAT DS", "METAL DS", "OCT FUZZ", "BLUES DRIVE", "OVERDRIVE",
    "TUBESCREAMER", "TURBO OD", "DISTORTION", "RAT", "GUV DS", "DST+",
    "METAL ZONE", "'60s FUZZ", "MUFF FUZZ", "HM-2", "METAL CORE", "CENTA OD",
];

pub const BOOSTER_TYPE_VALUES: &[u8] = &[
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
    0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
];

pub const MOD_TYPES: &[&str] = &[
    "TOUCH WAH", "AUTO WAH", "PEDAL WAH", "COMPRESSOR", "LIMITER", "GRAPHIC EQ",
    "PARAMETRIC EQ", "GUITAR SIM", "SLOW GEAR", "WAVE SYNTH", "OCTAVE",
    "PITCH SHIFTER", "HARMONIST", "AC PROCESSOR", "PHASER", "FLANGER", "TREMOLO",
    "ROTARY", "UNI-V", "SLICER", "VIBRATO", "RING MOD", "HUMANIZER", "2X2 CHORUS",
    "AC GUITAR SIM", "PHASER 90E", "FLANGER 117E", "WAH 95E", "DC-30",
    "HEAVY OCTAVE", "PEDAL BEND",
];

pub const MOD_TYPE_VALUES: &[u8] = &[
    0x00, 0x01, 0x02, 0x03, 0x04, 0x06, 0x07, 0x09, 0x0A, 0x0C, 0x0E, 0x0F,
    0x10, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,
    0x1F, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
];

pub const FX_TYPES: &[&str] = MOD_TYPES;
pub const FX_TYPE_VALUES: &[u8] = MOD_TYPE_VALUES;

pub const DELAY_TYPES: &[&str] = &[
    "DIGITAL", "PAN", "STEREO", "DUAL SERIES", "DUAL PARALLEL", "DUAL L/R",
    "REVERSE", "ANALOG", "TAPE ECHO", "MODULATE", "SDE-3000",
];

pub const DELAY_TYPE_VALUES: &[u8] = &[
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
];

pub const REVERB_TYPES: &[&str] = &[
    "AMBIENCE", "ROOM", "HALL 1", "HALL 2", "PLATE", "SPRING", "MODULATE",
];

pub const REVERB_TYPE_VALUES: &[u8] = &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];

#[derive(Debug, Clone, Copy)]
pub struct EffectParam {
    pub name: &'static str,
    pub page_offset: u8,
    pub addr: u8,
    pub max: u8,
}

const fn ep(name: &'static str, page: u8, addr: u8) -> Option<EffectParam> {
    Some(EffectParam { name, page_offset: page, addr, max: 100 })
}

const fn ep_max(name: &'static str, page: u8, addr: u8, max: u8) -> Option<EffectParam> {
    Some(EffectParam { name, page_offset: page, addr, max })
}

const N: Option<EffectParam> = None;

pub const MAIN_PARAMS: [[Option<EffectParam>; 4]; 31] = [
    [ep("SENS", 0, 0x04), ep("FREQ", 0, 0x05), ep("PEAK", 0, 0x06), ep("LEVEL", 0, 0x08)],
    [ep("FREQ", 0, 0x0A), ep("PEAK", 0, 0x0B), ep("RATE", 0, 0x0C), ep("DEPTH", 0, 0x0D)],
    [ep("POS", 0, 0x11), ep("MIN", 0, 0x12), ep("MAX", 0, 0x13), ep("LEVEL", 0, 0x14)],
    [ep("SUSTAIN", 0, 0x17), ep("ATTACK", 0, 0x18), ep("TONE", 0, 0x19), ep("LEVEL", 0, 0x1A)],
    [ep("THRESH", 0, 0x1D), ep("ATTACK", 0, 0x1C), ep("RATIO", 0, 0x1E), ep("LEVEL", 0, 0x20)],
    [ep_max("100Hz", 0, 0x23, 40), ep_max("1kHz", 0, 0x26, 40), ep_max("4kHz", 0, 0x28, 40), ep("LEVEL", 0, 0x2B)],
    [ep_max("LO GAIN", 0, 0x2D, 40), ep_max("MID GAIN", 0, 0x30, 40), ep_max("HI GAIN", 0, 0x34, 40), ep("LEVEL", 0, 0x36)],
    [ep("LOW", 0, 0x38), ep("HIGH", 0, 0x39), ep("BODY", 0, 0x3B), ep("LEVEL", 0, 0x3A)],
    [ep("SENS", 0, 0x3C), ep("RISE", 0, 0x3D), ep("LEVEL", 0, 0x3E), N],
    [ep("CUTOFF", 0, 0x40), ep("RESON", 0, 0x41), ep("SENS", 0, 0x42), ep("LEVEL", 0, 0x45)],
    [ep("RANGE", 0, 0x47), ep("OCTAVE", 0, 0x48), ep("MIX", 0, 0x49), N],
    [ep_max("PITCH", 0, 0x4C, 48), ep("FINE", 0, 0x4D), ep("DELAY", 0, 0x4E), ep("LEVEL", 0, 0x50)],
    [ep("HARMONY", 0, 0x5A), ep("DELAY", 0, 0x5B), ep("LEVEL", 0, 0x5D), ep("FEEDBACK", 0, 0x62)],
    [ep("BASS", 0, 0x7D), ep("MIDDLE", 0, 0x7E), ep("TREBLE", 1, 0x00), ep("LEVEL", 1, 0x02)],
    [ep("RATE", 1, 0x04), ep("DEPTH", 1, 0x05), ep("MANUAL", 1, 0x06), ep("RESON", 1, 0x07)],
    [ep("RATE", 1, 0x0B), ep("DEPTH", 1, 0x0C), ep("MANUAL", 1, 0x0D), ep("RESON", 1, 0x0E)],
    [ep("RATE", 1, 0x14), ep("DEPTH", 1, 0x15), ep("LEVEL", 1, 0x16), N],
    [ep("SLOW", 1, 0x18), ep("FAST", 1, 0x19), ep("DEPTH", 1, 0x1C), ep("LEVEL", 1, 0x1D)],
    [ep("RATE", 1, 0x1E), ep("DEPTH", 1, 0x1F), ep("LEVEL", 1, 0x20), N],
    [ep("PATTERN", 1, 0x21), ep("RATE", 1, 0x22), ep("SENS", 1, 0x23), ep("LEVEL", 1, 0x24)],
    [ep("RATE", 1, 0x26), ep("DEPTH", 1, 0x27), ep("RISE", 1, 0x29), ep("LEVEL", 1, 0x2A)],
    [ep("FREQ", 1, 0x2C), ep("LEVEL", 1, 0x2D), ep("MIX", 1, 0x2E), N],
    [ep("RATE", 1, 0x33), ep("DEPTH", 1, 0x34), ep("SENS", 1, 0x32), ep("LEVEL", 1, 0x36)],
    [ep("RATE", 1, 0x38), ep("DEPTH", 1, 0x39), ep("PRE-DLY", 1, 0x3A), ep("LEVEL", 1, 0x40)],
    [ep("TOP", 1, 0x41), ep("BODY", 1, 0x42), ep("LOW", 1, 0x43), ep("HIGH", 1, 0x44)],
    [ep_max("SCRIPT", 1, 0x46, 1), ep("SPEED", 1, 0x47), N, N],
    [ep("MANUAL", 1, 0x48), ep("WIDTH", 1, 0x49), ep("SPEED", 1, 0x4A), ep("REGEN", 1, 0x4B)],
    [ep("POS", 1, 0x4C), ep("MIN", 1, 0x4D), ep("MAX", 1, 0x4E), ep("LEVEL", 1, 0x4F)],
    [ep("INPUT", 1, 0x52), ep("INTENS", 1, 0x53), ep("RATE", 1, 0x54), ep("VOL", 1, 0x57)],
    [ep("OCT -1", 1, 0x5A), ep("OCT -2", 1, 0x5B), ep("DRY", 1, 0x5C), N],
    [ep_max("PITCH", 1, 0x5D, 48), ep("POS", 1, 0x5E), ep("LEVEL", 1, 0x5F), ep("MIX", 1, 0x60)],
];

pub const MOD_BASE_PAGE: u8 = 0x01;
pub const FX_BASE_PAGE: u8 = 0x03;
