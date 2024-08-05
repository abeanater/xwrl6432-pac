#[doc = "Register `TBCTL_TBSTS` reader"]
pub type R = crate::R<TbctlTbstsSpec>;
#[doc = "Register `TBCTL_TBSTS` writer"]
pub type W = crate::W<TbctlTbstsSpec>;
#[doc = "Field `TBCTL_CTRMODE` reader - 1:0\\]
Counter Mode The time-base counter mode is normally configured once and not changed during normal operation. If you change the mode of the counter, the change will take effect at the next TBCLK edge and the current counter value shall increment or decrement from the value before the mode change. These bits set the time-base counter mode of operation as follows: 0 Up-count mode 1h Down-count mode 2h Up-down-count mode 3h Stop-freeze counter operation (default on reset)"]
pub type TbctlCtrmodeR = crate::FieldReader;
#[doc = "Field `TBCTL_CTRMODE` writer - 1:0\\]
Counter Mode The time-base counter mode is normally configured once and not changed during normal operation. If you change the mode of the counter, the change will take effect at the next TBCLK edge and the current counter value shall increment or decrement from the value before the mode change. These bits set the time-base counter mode of operation as follows: 0 Up-count mode 1h Down-count mode 2h Up-down-count mode 3h Stop-freeze counter operation (default on reset)"]
pub type TbctlCtrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TBCTL_PHSEN` reader - 2:2\\]
Counter Register Load From Phase Register Enable 0 Do not load the time-base counter (TBCTR) from the time-base phase register (TBPHS) 1 Load the time-base counter with the phase register when an EPWMxSYNCI input signal occurs or when a software synchronization is forced by the SWFSYNC bit, or when a digital compare sync event occurs."]
pub type TbctlPhsenR = crate::BitReader;
#[doc = "Field `TBCTL_PHSEN` writer - 2:2\\]
Counter Register Load From Phase Register Enable 0 Do not load the time-base counter (TBCTR) from the time-base phase register (TBPHS) 1 Load the time-base counter with the phase register when an EPWMxSYNCI input signal occurs or when a software synchronization is forced by the SWFSYNC bit, or when a digital compare sync event occurs."]
pub type TbctlPhsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBCTL_PRDLD` reader - 3:3\\]
Active Period Register Load From Shadow Register Select 0 The period register (TBPRD) is loaded from its shadow register when the time-base counter, TBCTR, is equal to zero. A write or read to the TBPRD register accesses the shadow register. 1 Load the TBPRD register immediately without using a shadow register. A write or read to the TBPRD register directly accesses the active register."]
pub type TbctlPrdldR = crate::BitReader;
#[doc = "Field `TBCTL_PRDLD` writer - 3:3\\]
Active Period Register Load From Shadow Register Select 0 The period register (TBPRD) is loaded from its shadow register when the time-base counter, TBCTR, is equal to zero. A write or read to the TBPRD register accesses the shadow register. 1 Load the TBPRD register immediately without using a shadow register. A write or read to the TBPRD register directly accesses the active register."]
pub type TbctlPrdldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBCTL_SYNCOSEL` reader - 5:4\\]
Synchronization Output Select. These bits select the source of the EPWMxSYNCO signal. 0 EPWMxSYNC: 1h CTR = zero: Time-base counter equal to zero (TBCTR = 0x0000) 2h CTR = CMPB : Time-base counter equal to counter-compare B (TBCTR = CMPB) 3h Disable EPWMxSYNCO signal"]
pub type TbctlSyncoselR = crate::FieldReader;
#[doc = "Field `TBCTL_SYNCOSEL` writer - 5:4\\]
Synchronization Output Select. These bits select the source of the EPWMxSYNCO signal. 0 EPWMxSYNC: 1h CTR = zero: Time-base counter equal to zero (TBCTR = 0x0000) 2h CTR = CMPB : Time-base counter equal to counter-compare B (TBCTR = CMPB) 3h Disable EPWMxSYNCO signal"]
pub type TbctlSyncoselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TBCTL_SWFSYNC` reader - 6:6\\]
Software Forced Synchronization Pulse 0 Writing a 0 has no effect and reads always return a 0. 1 Writing a 1 forces a one-time synchronization pulse to be generated. This event is ORed with the EPWMxSYNCI input of the ePWM module. SWFSYNC is valid (operates) only when EPWMxSYNCI is selected by SYNCOSEL = 00."]
pub type TbctlSwfsyncR = crate::BitReader;
#[doc = "Field `TBCTL_SWFSYNC` writer - 6:6\\]
Software Forced Synchronization Pulse 0 Writing a 0 has no effect and reads always return a 0. 1 Writing a 1 forces a one-time synchronization pulse to be generated. This event is ORed with the EPWMxSYNCI input of the ePWM module. SWFSYNC is valid (operates) only when EPWMxSYNCI is selected by SYNCOSEL = 00."]
pub type TbctlSwfsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBCTL_HSPCLKDIV` reader - 9:7\\]
High Speed Time-base Clock Prescale Bits. These bits determine part of the time-base clock prescale value: TBCLK = VCLK3 / (HSPCLKDIV ∩┐╜ CLKDIV) 0 /1 1h /2 (default on reset) 2h /4 3h /6 4h /8 5h /10 6h /12 7h /14"]
pub type TbctlHspclkdivR = crate::FieldReader;
#[doc = "Field `TBCTL_HSPCLKDIV` writer - 9:7\\]
High Speed Time-base Clock Prescale Bits. These bits determine part of the time-base clock prescale value: TBCLK = VCLK3 / (HSPCLKDIV ∩┐╜ CLKDIV) 0 /1 1h /2 (default on reset) 2h /4 3h /6 4h /8 5h /10 6h /12 7h /14"]
pub type TbctlHspclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TBCTL_CLKDIV` reader - 12:10\\]
Time-base Clock Prescale Bits These bits determine part of the time-base clock prescale value. TBCLK = VCLK3 / (HSPCLKDIV ∩┐╜ CLKDIV) 0 /1 (default on reset) 1h /2 2h /4 3h /8 4h /16 5h /32 6h /64 7h /128"]
pub type TbctlClkdivR = crate::FieldReader;
#[doc = "Field `TBCTL_CLKDIV` writer - 12:10\\]
Time-base Clock Prescale Bits These bits determine part of the time-base clock prescale value. TBCLK = VCLK3 / (HSPCLKDIV ∩┐╜ CLKDIV) 0 /1 (default on reset) 1h /2 2h /4 3h /8 4h /16 5h /32 6h /64 7h /128"]
pub type TbctlClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TBCTL_PHSDIR` reader - 13:13\\]
Phase Direction Bit. This bit is only used when the time-base counter is configured in the up-down-count mode. The PHSDIR bit indicates the direction the time-base counter (TBCTR) will count after a synchronization event occurs and a new phase value is loaded from the phase (TBPHS) register. This is irrespective of the direction of the counter before the synchronization event.. In the up-count and down-count modes this bit is ignored. 0 Count down after the synchronization event. 1 Count up after the synchronization event"]
pub type TbctlPhsdirR = crate::BitReader;
#[doc = "Field `TBCTL_PHSDIR` writer - 13:13\\]
Phase Direction Bit. This bit is only used when the time-base counter is configured in the up-down-count mode. The PHSDIR bit indicates the direction the time-base counter (TBCTR) will count after a synchronization event occurs and a new phase value is loaded from the phase (TBPHS) register. This is irrespective of the direction of the counter before the synchronization event.. In the up-count and down-count modes this bit is ignored. 0 Count down after the synchronization event. 1 Count up after the synchronization event"]
pub type TbctlPhsdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBCTL_FREE_SOFT` reader - 15:14\\]
Emulation Mode Bits. These bits select the behavior of the ePWM time-base counter during emulation events: 0 Stop after the next time-base counter increment or decrement 1h Stop when counter completes a whole cycle: ΓÇó Up-count mode: stop when the time-base counter = period (TBCTR = TBPRD) ΓÇó Down-count mode: stop when the time-base counter = 0x0000 (TBCTR = 0x0000) ΓÇó Up-down-count mode: stop when the time-base counter = 0x0000 (TBCTR = 0x0000) 2h-3h Free run"]
pub type TbctlFreeSoftR = crate::FieldReader;
#[doc = "Field `TBCTL_FREE_SOFT` writer - 15:14\\]
Emulation Mode Bits. These bits select the behavior of the ePWM time-base counter during emulation events: 0 Stop after the next time-base counter increment or decrement 1h Stop when counter completes a whole cycle: ΓÇó Up-count mode: stop when the time-base counter = period (TBCTR = TBPRD) ΓÇó Down-count mode: stop when the time-base counter = 0x0000 (TBCTR = 0x0000) ΓÇó Up-down-count mode: stop when the time-base counter = 0x0000 (TBCTR = 0x0000) 2h-3h Free run"]
pub type TbctlFreeSoftW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TBSTS_CTRDIR` reader - 16:16\\]
Time-Base Counter Direction Status Bit. At reset, the counter is frozen; therefore, this bit has no meaning. To make this bit meaningful, you must first set the appropriate mode via TBCTL\\[CTRMODE\\]. 0 Time-Base Counter is currently counting down. 1 Time-Base Counter is currently counting up."]
pub type TbstsCtrdirR = crate::BitReader;
#[doc = "Field `TBSTS_CTRDIR` writer - 16:16\\]
Time-Base Counter Direction Status Bit. At reset, the counter is frozen; therefore, this bit has no meaning. To make this bit meaningful, you must first set the appropriate mode via TBCTL\\[CTRMODE\\]. 0 Time-Base Counter is currently counting down. 1 Time-Base Counter is currently counting up."]
pub type TbstsCtrdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBSTS_SYNCI` reader - 17:17\\]
Input Synchronization Latched Status Bit 0 Read: Indicates no external synchronization event has occurred. Write: No effect. 1 Read: Indicates that an external synchronization event has occurred (EPWMxSYNCI). Write: Clears the latched event."]
pub type TbstsSynciR = crate::BitReader;
#[doc = "Field `TBSTS_SYNCI` writer - 17:17\\]
Input Synchronization Latched Status Bit 0 Read: Indicates no external synchronization event has occurred. Write: No effect. 1 Read: Indicates that an external synchronization event has occurred (EPWMxSYNCI). Write: Clears the latched event."]
pub type TbstsSynciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBSTS_CTRMAX` reader - 18:18\\]
Time-Base Counter Max Latched Status Bit 0 Read: Indicates the time-base counter never reached its maximum value. Write: No effect. 1 Read: Indicates that the time-base counter reached the maximum value 0xFFFF. Write: Clears the latched event."]
pub type TbstsCtrmaxR = crate::BitReader;
#[doc = "Field `TBSTS_CTRMAX` writer - 18:18\\]
Time-Base Counter Max Latched Status Bit 0 Read: Indicates the time-base counter never reached its maximum value. Write: No effect. 1 Read: Indicates that the time-base counter reached the maximum value 0xFFFF. Write: Clears the latched event."]
pub type TbstsCtrmaxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Counter Mode The time-base counter mode is normally configured once and not changed during normal operation. If you change the mode of the counter, the change will take effect at the next TBCLK edge and the current counter value shall increment or decrement from the value before the mode change. These bits set the time-base counter mode of operation as follows: 0 Up-count mode 1h Down-count mode 2h Up-down-count mode 3h Stop-freeze counter operation (default on reset)"]
    #[inline(always)]
    pub fn tbctl_ctrmode(&self) -> TbctlCtrmodeR {
        TbctlCtrmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Counter Register Load From Phase Register Enable 0 Do not load the time-base counter (TBCTR) from the time-base phase register (TBPHS) 1 Load the time-base counter with the phase register when an EPWMxSYNCI input signal occurs or when a software synchronization is forced by the SWFSYNC bit, or when a digital compare sync event occurs."]
    #[inline(always)]
    pub fn tbctl_phsen(&self) -> TbctlPhsenR {
        TbctlPhsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Active Period Register Load From Shadow Register Select 0 The period register (TBPRD) is loaded from its shadow register when the time-base counter, TBCTR, is equal to zero. A write or read to the TBPRD register accesses the shadow register. 1 Load the TBPRD register immediately without using a shadow register. A write or read to the TBPRD register directly accesses the active register."]
    #[inline(always)]
    pub fn tbctl_prdld(&self) -> TbctlPrdldR {
        TbctlPrdldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Synchronization Output Select. These bits select the source of the EPWMxSYNCO signal. 0 EPWMxSYNC: 1h CTR = zero: Time-base counter equal to zero (TBCTR = 0x0000) 2h CTR = CMPB : Time-base counter equal to counter-compare B (TBCTR = CMPB) 3h Disable EPWMxSYNCO signal"]
    #[inline(always)]
    pub fn tbctl_syncosel(&self) -> TbctlSyncoselR {
        TbctlSyncoselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Software Forced Synchronization Pulse 0 Writing a 0 has no effect and reads always return a 0. 1 Writing a 1 forces a one-time synchronization pulse to be generated. This event is ORed with the EPWMxSYNCI input of the ePWM module. SWFSYNC is valid (operates) only when EPWMxSYNCI is selected by SYNCOSEL = 00."]
    #[inline(always)]
    pub fn tbctl_swfsync(&self) -> TbctlSwfsyncR {
        TbctlSwfsyncR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - 9:7\\]
High Speed Time-base Clock Prescale Bits. These bits determine part of the time-base clock prescale value: TBCLK = VCLK3 / (HSPCLKDIV ∩┐╜ CLKDIV) 0 /1 1h /2 (default on reset) 2h /4 3h /6 4h /8 5h /10 6h /12 7h /14"]
    #[inline(always)]
    pub fn tbctl_hspclkdiv(&self) -> TbctlHspclkdivR {
        TbctlHspclkdivR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - 12:10\\]
Time-base Clock Prescale Bits These bits determine part of the time-base clock prescale value. TBCLK = VCLK3 / (HSPCLKDIV ∩┐╜ CLKDIV) 0 /1 (default on reset) 1h /2 2h /4 3h /8 4h /16 5h /32 6h /64 7h /128"]
    #[inline(always)]
    pub fn tbctl_clkdiv(&self) -> TbctlClkdivR {
        TbctlClkdivR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Phase Direction Bit. This bit is only used when the time-base counter is configured in the up-down-count mode. The PHSDIR bit indicates the direction the time-base counter (TBCTR) will count after a synchronization event occurs and a new phase value is loaded from the phase (TBPHS) register. This is irrespective of the direction of the counter before the synchronization event.. In the up-count and down-count modes this bit is ignored. 0 Count down after the synchronization event. 1 Count up after the synchronization event"]
    #[inline(always)]
    pub fn tbctl_phsdir(&self) -> TbctlPhsdirR {
        TbctlPhsdirR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Emulation Mode Bits. These bits select the behavior of the ePWM time-base counter during emulation events: 0 Stop after the next time-base counter increment or decrement 1h Stop when counter completes a whole cycle: ΓÇó Up-count mode: stop when the time-base counter = period (TBCTR = TBPRD) ΓÇó Down-count mode: stop when the time-base counter = 0x0000 (TBCTR = 0x0000) ΓÇó Up-down-count mode: stop when the time-base counter = 0x0000 (TBCTR = 0x0000) 2h-3h Free run"]
    #[inline(always)]
    pub fn tbctl_free_soft(&self) -> TbctlFreeSoftR {
        TbctlFreeSoftR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Time-Base Counter Direction Status Bit. At reset, the counter is frozen; therefore, this bit has no meaning. To make this bit meaningful, you must first set the appropriate mode via TBCTL\\[CTRMODE\\]. 0 Time-Base Counter is currently counting down. 1 Time-Base Counter is currently counting up."]
    #[inline(always)]
    pub fn tbsts_ctrdir(&self) -> TbstsCtrdirR {
        TbstsCtrdirR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Input Synchronization Latched Status Bit 0 Read: Indicates no external synchronization event has occurred. Write: No effect. 1 Read: Indicates that an external synchronization event has occurred (EPWMxSYNCI). Write: Clears the latched event."]
    #[inline(always)]
    pub fn tbsts_synci(&self) -> TbstsSynciR {
        TbstsSynciR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Time-Base Counter Max Latched Status Bit 0 Read: Indicates the time-base counter never reached its maximum value. Write: No effect. 1 Read: Indicates that the time-base counter reached the maximum value 0xFFFF. Write: Clears the latched event."]
    #[inline(always)]
    pub fn tbsts_ctrmax(&self) -> TbstsCtrmaxR {
        TbstsCtrmaxR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Counter Mode The time-base counter mode is normally configured once and not changed during normal operation. If you change the mode of the counter, the change will take effect at the next TBCLK edge and the current counter value shall increment or decrement from the value before the mode change. These bits set the time-base counter mode of operation as follows: 0 Up-count mode 1h Down-count mode 2h Up-down-count mode 3h Stop-freeze counter operation (default on reset)"]
    #[inline(always)]
    #[must_use]
    pub fn tbctl_ctrmode(&mut self) -> TbctlCtrmodeW<TbctlTbstsSpec> {
        TbctlCtrmodeW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Counter Register Load From Phase Register Enable 0 Do not load the time-base counter (TBCTR) from the time-base phase register (TBPHS) 1 Load the time-base counter with the phase register when an EPWMxSYNCI input signal occurs or when a software synchronization is forced by the SWFSYNC bit, or when a digital compare sync event occurs."]
    #[inline(always)]
    #[must_use]
    pub fn tbctl_phsen(&mut self) -> TbctlPhsenW<TbctlTbstsSpec> {
        TbctlPhsenW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Active Period Register Load From Shadow Register Select 0 The period register (TBPRD) is loaded from its shadow register when the time-base counter, TBCTR, is equal to zero. A write or read to the TBPRD register accesses the shadow register. 1 Load the TBPRD register immediately without using a shadow register. A write or read to the TBPRD register directly accesses the active register."]
    #[inline(always)]
    #[must_use]
    pub fn tbctl_prdld(&mut self) -> TbctlPrdldW<TbctlTbstsSpec> {
        TbctlPrdldW::new(self, 3)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Synchronization Output Select. These bits select the source of the EPWMxSYNCO signal. 0 EPWMxSYNC: 1h CTR = zero: Time-base counter equal to zero (TBCTR = 0x0000) 2h CTR = CMPB : Time-base counter equal to counter-compare B (TBCTR = CMPB) 3h Disable EPWMxSYNCO signal"]
    #[inline(always)]
    #[must_use]
    pub fn tbctl_syncosel(&mut self) -> TbctlSyncoselW<TbctlTbstsSpec> {
        TbctlSyncoselW::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
Software Forced Synchronization Pulse 0 Writing a 0 has no effect and reads always return a 0. 1 Writing a 1 forces a one-time synchronization pulse to be generated. This event is ORed with the EPWMxSYNCI input of the ePWM module. SWFSYNC is valid (operates) only when EPWMxSYNCI is selected by SYNCOSEL = 00."]
    #[inline(always)]
    #[must_use]
    pub fn tbctl_swfsync(&mut self) -> TbctlSwfsyncW<TbctlTbstsSpec> {
        TbctlSwfsyncW::new(self, 6)
    }
    #[doc = "Bits 7:9 - 9:7\\]
High Speed Time-base Clock Prescale Bits. These bits determine part of the time-base clock prescale value: TBCLK = VCLK3 / (HSPCLKDIV ∩┐╜ CLKDIV) 0 /1 1h /2 (default on reset) 2h /4 3h /6 4h /8 5h /10 6h /12 7h /14"]
    #[inline(always)]
    #[must_use]
    pub fn tbctl_hspclkdiv(&mut self) -> TbctlHspclkdivW<TbctlTbstsSpec> {
        TbctlHspclkdivW::new(self, 7)
    }
    #[doc = "Bits 10:12 - 12:10\\]
Time-base Clock Prescale Bits These bits determine part of the time-base clock prescale value. TBCLK = VCLK3 / (HSPCLKDIV ∩┐╜ CLKDIV) 0 /1 (default on reset) 1h /2 2h /4 3h /8 4h /16 5h /32 6h /64 7h /128"]
    #[inline(always)]
    #[must_use]
    pub fn tbctl_clkdiv(&mut self) -> TbctlClkdivW<TbctlTbstsSpec> {
        TbctlClkdivW::new(self, 10)
    }
    #[doc = "Bit 13 - 13:13\\]
Phase Direction Bit. This bit is only used when the time-base counter is configured in the up-down-count mode. The PHSDIR bit indicates the direction the time-base counter (TBCTR) will count after a synchronization event occurs and a new phase value is loaded from the phase (TBPHS) register. This is irrespective of the direction of the counter before the synchronization event.. In the up-count and down-count modes this bit is ignored. 0 Count down after the synchronization event. 1 Count up after the synchronization event"]
    #[inline(always)]
    #[must_use]
    pub fn tbctl_phsdir(&mut self) -> TbctlPhsdirW<TbctlTbstsSpec> {
        TbctlPhsdirW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Emulation Mode Bits. These bits select the behavior of the ePWM time-base counter during emulation events: 0 Stop after the next time-base counter increment or decrement 1h Stop when counter completes a whole cycle: ΓÇó Up-count mode: stop when the time-base counter = period (TBCTR = TBPRD) ΓÇó Down-count mode: stop when the time-base counter = 0x0000 (TBCTR = 0x0000) ΓÇó Up-down-count mode: stop when the time-base counter = 0x0000 (TBCTR = 0x0000) 2h-3h Free run"]
    #[inline(always)]
    #[must_use]
    pub fn tbctl_free_soft(&mut self) -> TbctlFreeSoftW<TbctlTbstsSpec> {
        TbctlFreeSoftW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Time-Base Counter Direction Status Bit. At reset, the counter is frozen; therefore, this bit has no meaning. To make this bit meaningful, you must first set the appropriate mode via TBCTL\\[CTRMODE\\]. 0 Time-Base Counter is currently counting down. 1 Time-Base Counter is currently counting up."]
    #[inline(always)]
    #[must_use]
    pub fn tbsts_ctrdir(&mut self) -> TbstsCtrdirW<TbctlTbstsSpec> {
        TbstsCtrdirW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Input Synchronization Latched Status Bit 0 Read: Indicates no external synchronization event has occurred. Write: No effect. 1 Read: Indicates that an external synchronization event has occurred (EPWMxSYNCI). Write: Clears the latched event."]
    #[inline(always)]
    #[must_use]
    pub fn tbsts_synci(&mut self) -> TbstsSynciW<TbctlTbstsSpec> {
        TbstsSynciW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Time-Base Counter Max Latched Status Bit 0 Read: Indicates the time-base counter never reached its maximum value. Write: No effect. 1 Read: Indicates that the time-base counter reached the maximum value 0xFFFF. Write: Clears the latched event."]
    #[inline(always)]
    #[must_use]
    pub fn tbsts_ctrmax(&mut self) -> TbstsCtrmaxW<TbctlTbstsSpec> {
        TbstsCtrmaxW::new(self, 18)
    }
}
#[doc = "Time-Base Control Register/ Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctl_tbsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctl_tbsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbctlTbstsSpec;
impl crate::RegisterSpec for TbctlTbstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbctl_tbsts::R`](R) reader structure"]
impl crate::Readable for TbctlTbstsSpec {}
#[doc = "`write(|w| ..)` method takes [`tbctl_tbsts::W`](W) writer structure"]
impl crate::Writable for TbctlTbstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBCTL_TBSTS to value 0"]
impl crate::Resettable for TbctlTbstsSpec {
    const RESET_VALUE: u32 = 0;
}
