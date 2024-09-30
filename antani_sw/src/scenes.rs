use embassy_sync::lazy_lock::LazyLock;
use heapless::Vec;

use crate::rgbeffects::{ColorPalette, FragmentShader, LedPattern, Pattern, RenderCommand};

pub struct Patterns {
    pub power_100: LedPattern,
    pub power_75: LedPattern,
    pub power_50: LedPattern,
    pub power_25: LedPattern,
    pub dado1: LedPattern,
    pub dado2: LedPattern,
    pub dado3: LedPattern,
    pub dado4: LedPattern,
    pub dado5: LedPattern,
    pub dado6: LedPattern,
    pub all_on: LedPattern,
    pub vertical_stripe_1: LedPattern,
    pub vertical_stripe_2: LedPattern,
    pub vertical_stripe_3: LedPattern,
    pub everything_once: &'static [LedPattern],
    pub boot_animation: &'static [LedPattern],
}

pub static PATTERNS: LazyLock<Patterns> = LazyLock::new(|| Patterns {
    // patterns for light power
    power_100: 0b111111111,
    power_75: 0b000111111,
    power_50: 0b000000111,
    power_25: 0b000000001,

    dado1: 0b000010000,
    dado2: 0b001000100,
    dado3: 0b001010100,
    dado4: 0b101000101,
    dado5: 0b101010101,
    dado6: 0b101101101,
    all_on: 0b111111111,
    vertical_stripe_1: 0b100100100,
    vertical_stripe_2: 0b010010010,
    vertical_stripe_3: 0b001001001,

    everything_once: &[
        0b100000000,
        0b010000000,
        0b001000000,
        0b000100000,
        0b000010000,
        0b000001000,
        0b000000100,
        0b000000010,
        0b000000001,
    ],
    boot_animation: &[
        0b010000000,
        0b010010000,
        0b111111000,
        0b000111111,
        0b000000111,
        0b000000010,
        0b000000000,
        0b000000000,
        0b000000000,
        0b000000000,
    ],
});

pub type Scenes = Vec<Vec<RenderCommand, 8>, 20>;
pub fn scenes() -> Scenes {
    let patterns = PATTERNS.get();

    Vec::from_slice(&[
        // normal glider
        Vec::from_slice(&[RenderCommand {
            effect: Pattern::Simple(patterns.dado1),
            color: ColorPalette::Solid((255, 0, 0).into()),
            ..Default::default()
        }])
        .unwrap(),
        Vec::from_slice(&[RenderCommand {
            effect: Pattern::Simple(patterns.dado2),
            color: ColorPalette::Solid((255, 0, 0).into()),
            ..Default::default()
        }])
        .unwrap(),
        Vec::from_slice(&[RenderCommand {
            effect: Pattern::Simple(patterns.dado3),
            color: ColorPalette::Solid((255, 0, 0).into()),
            ..Default::default()
        }])
        .unwrap(),
        Vec::from_slice(&[RenderCommand {
            effect: Pattern::Simple(patterns.dado4),
            color: ColorPalette::Solid((255, 0, 0).into()),
            ..Default::default()
        }])
        .unwrap(),
        Vec::from_slice(&[RenderCommand {
            effect: Pattern::Simple(patterns.dado5),
            color: ColorPalette::Solid((255, 0, 0).into()),
            ..Default::default()
        }])
        .unwrap(),
        Vec::from_slice(&[RenderCommand {
            effect: Pattern::Simple(patterns.dado6),
            color: ColorPalette::Solid((255, 0, 0).into()),
            ..Default::default()
        }])
        .unwrap(),
    ])
    .unwrap()
}
