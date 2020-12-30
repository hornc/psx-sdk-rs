use super::{TimerCounter, TimerMode};

use crate::mmio::Address;
use crate::value::LoadMut;

/// [Timer 1 mode](http://problemkaputt.de/psx-spx.htm#timers) register at `0x1F80_1114`.
/// Used to configure timer 1.
pub struct MODE;

/// [Timer 1 counter](http://problemkaputt.de/psx-spx.htm#timers) register at `0x1F80_1110`.
/// Contains the timer's current value.
pub struct CNT;

/// [Timer 1 target](http://problemkaputt.de/psx-spx.htm#timers) register at `0x1F80_1118`.
/// Contains the timer's target value.
pub struct TGT;

impl Address<u16> for MODE {
    const ADDRESS: u32 = 0x1F80_1114;
}

impl LoadMut<u16> for MODE {}

impl TimerMode for MODE {}

impl Address<u16> for CNT {
    const ADDRESS: u32 = 0x1F80_1110;
}

impl LoadMut<u16> for CNT {}

impl TimerCounter for CNT {}

impl Address<u16> for TGT {
    const ADDRESS: u32 = 0x1F80_1118;
}

impl LoadMut<u16> for TGT {}
