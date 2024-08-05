#[doc = "Register `TZFLG_TZCLR` reader"]
pub type R = crate::R<TzflgTzclrSpec>;
#[doc = "Register `TZFLG_TZCLR` writer"]
pub type W = crate::W<TzflgTzclrSpec>;
#[doc = "Field `TZFLG_INT` reader - 0:0\\]
Latched Trip Interrupt Status Flag 0 Indicates no interrupt has been generated. 1 Indicates an EPWMx_TZINT VIM interrupt was generated because of a trip condition. No further EPWMx_TZINT VIM interrupts will be generated until this flag is cleared. If the interrupt flag is cleared when either CBC or OST is set, then another interrupt pulse will be generated. Clearing all flag bits will prevent further interrupts. This bit is cleared by writing the appropriate value to the TZCLR register"]
pub type TzflgIntR = crate::BitReader;
#[doc = "Field `TZFLG_INT` writer - 0:0\\]
Latched Trip Interrupt Status Flag 0 Indicates no interrupt has been generated. 1 Indicates an EPWMx_TZINT VIM interrupt was generated because of a trip condition. No further EPWMx_TZINT VIM interrupts will be generated until this flag is cleared. If the interrupt flag is cleared when either CBC or OST is set, then another interrupt pulse will be generated. Clearing all flag bits will prevent further interrupts. This bit is cleared by writing the appropriate value to the TZCLR register"]
pub type TzflgIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZFLG_CBC` reader - 1:1\\]
Latched Status Flag for Cycle-By-Cycle Trip Event 0 No cycle-by-cycle trip event has occurred. 1 Indicates a trip event has occurred on a signal selected as a cycle-by-cycle trip source. The TZFLG\\[CBC\\]
bit will remain set until it is manually cleared by the user. If the cycle-by-cycle trip event is still present when the CBC bit is cleared, then CBC will be immediately set again. The specified condition on the signal is automatically cleared when the ePWM time-base counter reaches zero (TBCTR = 0x0000) if the trip condition is no longer present. The condition on the signal is only cleared when the TBCTR = 0x0000 no matter where in the cycle the CBC flag is cleared. This bit is cleared by writing the appropriate value to the TZCLR register"]
pub type TzflgCbcR = crate::BitReader;
#[doc = "Field `TZFLG_CBC` writer - 1:1\\]
Latched Status Flag for Cycle-By-Cycle Trip Event 0 No cycle-by-cycle trip event has occurred. 1 Indicates a trip event has occurred on a signal selected as a cycle-by-cycle trip source. The TZFLG\\[CBC\\]
bit will remain set until it is manually cleared by the user. If the cycle-by-cycle trip event is still present when the CBC bit is cleared, then CBC will be immediately set again. The specified condition on the signal is automatically cleared when the ePWM time-base counter reaches zero (TBCTR = 0x0000) if the trip condition is no longer present. The condition on the signal is only cleared when the TBCTR = 0x0000 no matter where in the cycle the CBC flag is cleared. This bit is cleared by writing the appropriate value to the TZCLR register"]
pub type TzflgCbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZFLG_OST` reader - 2:2\\]
Latched Status Flag for A One-Shot Trip Event 0 No one-shot trip event has occurred. 1 Indicates a trip event has occurred on a pin selected as a one-shot trip source. This bit is cleared by writing the appropriate value to the TZCLR register"]
pub type TzflgOstR = crate::BitReader;
#[doc = "Field `TZFLG_OST` writer - 2:2\\]
Latched Status Flag for A One-Shot Trip Event 0 No one-shot trip event has occurred. 1 Indicates a trip event has occurred on a pin selected as a one-shot trip source. This bit is cleared by writing the appropriate value to the TZCLR register"]
pub type TzflgOstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZFLG_DCAEVT1` reader - 3:3\\]
Latched Status Flag for Digital Compare Output A Event 1 0 Indicates no trip event has occurred on DCAEVT1 1 Indicates a trip event has occurred for the event defined for DCAEVT1"]
pub type TzflgDcaevt1R = crate::BitReader;
#[doc = "Field `TZFLG_DCAEVT1` writer - 3:3\\]
Latched Status Flag for Digital Compare Output A Event 1 0 Indicates no trip event has occurred on DCAEVT1 1 Indicates a trip event has occurred for the event defined for DCAEVT1"]
pub type TzflgDcaevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZFLG_DCAEVT2` reader - 4:4\\]
Latched Status Flag for Digital Compare Output A Event 2 0 Indicates no trip event has occurred on DCAEVT2 1 Indicates a trip event has occurred for the event defined for DCAEVT2"]
pub type TzflgDcaevt2R = crate::BitReader;
#[doc = "Field `TZFLG_DCAEVT2` writer - 4:4\\]
Latched Status Flag for Digital Compare Output A Event 2 0 Indicates no trip event has occurred on DCAEVT2 1 Indicates a trip event has occurred for the event defined for DCAEVT2"]
pub type TzflgDcaevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZFLG_DCBEVT1` reader - 5:5\\]
Latched Status Flag for Digital Compare Output B Event 1 0 Indicates no trip event has occurred on DCBEVT1 1 Indicates a trip event has occurred for the event defined for DCBEVT1"]
pub type TzflgDcbevt1R = crate::BitReader;
#[doc = "Field `TZFLG_DCBEVT1` writer - 5:5\\]
Latched Status Flag for Digital Compare Output B Event 1 0 Indicates no trip event has occurred on DCBEVT1 1 Indicates a trip event has occurred for the event defined for DCBEVT1"]
pub type TzflgDcbevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZFLG_DCBEVT2` reader - 6:6\\]
Latched Status Flag for Digital Compare Output B Event 2 0 Indicates no trip event has occurred on DCBEVT2 1 Indicates a trip event has occurred for the event defined for DCBEVT2"]
pub type TzflgDcbevt2R = crate::BitReader;
#[doc = "Field `TZFLG_DCBEVT2` writer - 6:6\\]
Latched Status Flag for Digital Compare Output B Event 2 0 Indicates no trip event has occurred on DCBEVT2 1 Indicates a trip event has occurred for the event defined for DCBEVT2"]
pub type TzflgDcbevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 15:7\\]
Reserved"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `Reserved2` writer - 15:7\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TZCLR_INT` reader - 16:16\\]
Global Interrupt Clear Flag 0 Has no effect. Always reads back a 0. 1 Clears the trip-interrupt flag for this ePWM module (TZFLG\\[INT\\]). NOTE: No further EPWMx_TZINT VIM interrupts will be generated until the flag is cleared. If the TZFLG\\[INT\\]
bit is cleared and any of the other flag bits are set, then another interrupt pulse will be generated. Clearing all flag bits will prevent further interrupts."]
pub type TzclrIntR = crate::BitReader;
#[doc = "Field `TZCLR_INT` writer - 16:16\\]
Global Interrupt Clear Flag 0 Has no effect. Always reads back a 0. 1 Clears the trip-interrupt flag for this ePWM module (TZFLG\\[INT\\]). NOTE: No further EPWMx_TZINT VIM interrupts will be generated until the flag is cleared. If the TZFLG\\[INT\\]
bit is cleared and any of the other flag bits are set, then another interrupt pulse will be generated. Clearing all flag bits will prevent further interrupts."]
pub type TzclrIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZCLR_CBC` reader - 17:17\\]
Clear Flag for Cycle-By-Cycle (CBC) Trip Latch 0 Has no effect. Always reads back a 0. 1 Clears this Trip (set) condition."]
pub type TzclrCbcR = crate::BitReader;
#[doc = "Field `TZCLR_CBC` writer - 17:17\\]
Clear Flag for Cycle-By-Cycle (CBC) Trip Latch 0 Has no effect. Always reads back a 0. 1 Clears this Trip (set) condition."]
pub type TzclrCbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZCLR_OST` reader - 18:18\\]
Clear Flag for One-Shot Trip (OST) Latch 0 Has no effect. Always reads back a 0. 1 Clears this Trip (set) condition."]
pub type TzclrOstR = crate::BitReader;
#[doc = "Field `TZCLR_OST` writer - 18:18\\]
Clear Flag for One-Shot Trip (OST) Latch 0 Has no effect. Always reads back a 0. 1 Clears this Trip (set) condition."]
pub type TzclrOstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZCLR_DCAEVT1` reader - 19:19\\]
Clear Flag for Digital Compare Output A Event 1 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCAEVT1 event trip condition."]
pub type TzclrDcaevt1R = crate::BitReader;
#[doc = "Field `TZCLR_DCAEVT1` writer - 19:19\\]
Clear Flag for Digital Compare Output A Event 1 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCAEVT1 event trip condition."]
pub type TzclrDcaevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZCLR_DCAEVT2` reader - 20:20\\]
Clear Flag for Digital Compare Output A Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCAEVT2 event trip condition"]
pub type TzclrDcaevt2R = crate::BitReader;
#[doc = "Field `TZCLR_DCAEVT2` writer - 20:20\\]
Clear Flag for Digital Compare Output A Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCAEVT2 event trip condition"]
pub type TzclrDcaevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZCLR_DCBEVT1` reader - 21:21\\]
Clear Flag for Digital Compare Output B Event 1 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCBEVT1 event trip condition"]
pub type TzclrDcbevt1R = crate::BitReader;
#[doc = "Field `TZCLR_DCBEVT1` writer - 21:21\\]
Clear Flag for Digital Compare Output B Event 1 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCBEVT1 event trip condition"]
pub type TzclrDcbevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZCLR_DCBEVT2` reader - 22:22\\]
Clear Flag for Digital Compare Output B Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCBEVT2 event trip condition."]
pub type TzclrDcbevt2R = crate::BitReader;
#[doc = "Field `TZCLR_DCBEVT2` writer - 22:22\\]
Clear Flag for Digital Compare Output B Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCBEVT2 event trip condition."]
pub type TzclrDcbevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 31:23\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 31:23\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Latched Trip Interrupt Status Flag 0 Indicates no interrupt has been generated. 1 Indicates an EPWMx_TZINT VIM interrupt was generated because of a trip condition. No further EPWMx_TZINT VIM interrupts will be generated until this flag is cleared. If the interrupt flag is cleared when either CBC or OST is set, then another interrupt pulse will be generated. Clearing all flag bits will prevent further interrupts. This bit is cleared by writing the appropriate value to the TZCLR register"]
    #[inline(always)]
    pub fn tzflg_int(&self) -> TzflgIntR {
        TzflgIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Latched Status Flag for Cycle-By-Cycle Trip Event 0 No cycle-by-cycle trip event has occurred. 1 Indicates a trip event has occurred on a signal selected as a cycle-by-cycle trip source. The TZFLG\\[CBC\\]
bit will remain set until it is manually cleared by the user. If the cycle-by-cycle trip event is still present when the CBC bit is cleared, then CBC will be immediately set again. The specified condition on the signal is automatically cleared when the ePWM time-base counter reaches zero (TBCTR = 0x0000) if the trip condition is no longer present. The condition on the signal is only cleared when the TBCTR = 0x0000 no matter where in the cycle the CBC flag is cleared. This bit is cleared by writing the appropriate value to the TZCLR register"]
    #[inline(always)]
    pub fn tzflg_cbc(&self) -> TzflgCbcR {
        TzflgCbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Latched Status Flag for A One-Shot Trip Event 0 No one-shot trip event has occurred. 1 Indicates a trip event has occurred on a pin selected as a one-shot trip source. This bit is cleared by writing the appropriate value to the TZCLR register"]
    #[inline(always)]
    pub fn tzflg_ost(&self) -> TzflgOstR {
        TzflgOstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Latched Status Flag for Digital Compare Output A Event 1 0 Indicates no trip event has occurred on DCAEVT1 1 Indicates a trip event has occurred for the event defined for DCAEVT1"]
    #[inline(always)]
    pub fn tzflg_dcaevt1(&self) -> TzflgDcaevt1R {
        TzflgDcaevt1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Latched Status Flag for Digital Compare Output A Event 2 0 Indicates no trip event has occurred on DCAEVT2 1 Indicates a trip event has occurred for the event defined for DCAEVT2"]
    #[inline(always)]
    pub fn tzflg_dcaevt2(&self) -> TzflgDcaevt2R {
        TzflgDcaevt2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Latched Status Flag for Digital Compare Output B Event 1 0 Indicates no trip event has occurred on DCBEVT1 1 Indicates a trip event has occurred for the event defined for DCBEVT1"]
    #[inline(always)]
    pub fn tzflg_dcbevt1(&self) -> TzflgDcbevt1R {
        TzflgDcbevt1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Latched Status Flag for Digital Compare Output B Event 2 0 Indicates no trip event has occurred on DCBEVT2 1 Indicates a trip event has occurred for the event defined for DCBEVT2"]
    #[inline(always)]
    pub fn tzflg_dcbevt2(&self) -> TzflgDcbevt2R {
        TzflgDcbevt2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:15 - 15:7\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Global Interrupt Clear Flag 0 Has no effect. Always reads back a 0. 1 Clears the trip-interrupt flag for this ePWM module (TZFLG\\[INT\\]). NOTE: No further EPWMx_TZINT VIM interrupts will be generated until the flag is cleared. If the TZFLG\\[INT\\]
bit is cleared and any of the other flag bits are set, then another interrupt pulse will be generated. Clearing all flag bits will prevent further interrupts."]
    #[inline(always)]
    pub fn tzclr_int(&self) -> TzclrIntR {
        TzclrIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Clear Flag for Cycle-By-Cycle (CBC) Trip Latch 0 Has no effect. Always reads back a 0. 1 Clears this Trip (set) condition."]
    #[inline(always)]
    pub fn tzclr_cbc(&self) -> TzclrCbcR {
        TzclrCbcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Clear Flag for One-Shot Trip (OST) Latch 0 Has no effect. Always reads back a 0. 1 Clears this Trip (set) condition."]
    #[inline(always)]
    pub fn tzclr_ost(&self) -> TzclrOstR {
        TzclrOstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Clear Flag for Digital Compare Output A Event 1 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCAEVT1 event trip condition."]
    #[inline(always)]
    pub fn tzclr_dcaevt1(&self) -> TzclrDcaevt1R {
        TzclrDcaevt1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Clear Flag for Digital Compare Output A Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCAEVT2 event trip condition"]
    #[inline(always)]
    pub fn tzclr_dcaevt2(&self) -> TzclrDcaevt2R {
        TzclrDcaevt2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Clear Flag for Digital Compare Output B Event 1 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCBEVT1 event trip condition"]
    #[inline(always)]
    pub fn tzclr_dcbevt1(&self) -> TzclrDcbevt1R {
        TzclrDcbevt1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Clear Flag for Digital Compare Output B Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCBEVT2 event trip condition."]
    #[inline(always)]
    pub fn tzclr_dcbevt2(&self) -> TzclrDcbevt2R {
        TzclrDcbevt2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Latched Trip Interrupt Status Flag 0 Indicates no interrupt has been generated. 1 Indicates an EPWMx_TZINT VIM interrupt was generated because of a trip condition. No further EPWMx_TZINT VIM interrupts will be generated until this flag is cleared. If the interrupt flag is cleared when either CBC or OST is set, then another interrupt pulse will be generated. Clearing all flag bits will prevent further interrupts. This bit is cleared by writing the appropriate value to the TZCLR register"]
    #[inline(always)]
    #[must_use]
    pub fn tzflg_int(&mut self) -> TzflgIntW<TzflgTzclrSpec> {
        TzflgIntW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Latched Status Flag for Cycle-By-Cycle Trip Event 0 No cycle-by-cycle trip event has occurred. 1 Indicates a trip event has occurred on a signal selected as a cycle-by-cycle trip source. The TZFLG\\[CBC\\]
bit will remain set until it is manually cleared by the user. If the cycle-by-cycle trip event is still present when the CBC bit is cleared, then CBC will be immediately set again. The specified condition on the signal is automatically cleared when the ePWM time-base counter reaches zero (TBCTR = 0x0000) if the trip condition is no longer present. The condition on the signal is only cleared when the TBCTR = 0x0000 no matter where in the cycle the CBC flag is cleared. This bit is cleared by writing the appropriate value to the TZCLR register"]
    #[inline(always)]
    #[must_use]
    pub fn tzflg_cbc(&mut self) -> TzflgCbcW<TzflgTzclrSpec> {
        TzflgCbcW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Latched Status Flag for A One-Shot Trip Event 0 No one-shot trip event has occurred. 1 Indicates a trip event has occurred on a pin selected as a one-shot trip source. This bit is cleared by writing the appropriate value to the TZCLR register"]
    #[inline(always)]
    #[must_use]
    pub fn tzflg_ost(&mut self) -> TzflgOstW<TzflgTzclrSpec> {
        TzflgOstW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Latched Status Flag for Digital Compare Output A Event 1 0 Indicates no trip event has occurred on DCAEVT1 1 Indicates a trip event has occurred for the event defined for DCAEVT1"]
    #[inline(always)]
    #[must_use]
    pub fn tzflg_dcaevt1(&mut self) -> TzflgDcaevt1W<TzflgTzclrSpec> {
        TzflgDcaevt1W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Latched Status Flag for Digital Compare Output A Event 2 0 Indicates no trip event has occurred on DCAEVT2 1 Indicates a trip event has occurred for the event defined for DCAEVT2"]
    #[inline(always)]
    #[must_use]
    pub fn tzflg_dcaevt2(&mut self) -> TzflgDcaevt2W<TzflgTzclrSpec> {
        TzflgDcaevt2W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Latched Status Flag for Digital Compare Output B Event 1 0 Indicates no trip event has occurred on DCBEVT1 1 Indicates a trip event has occurred for the event defined for DCBEVT1"]
    #[inline(always)]
    #[must_use]
    pub fn tzflg_dcbevt1(&mut self) -> TzflgDcbevt1W<TzflgTzclrSpec> {
        TzflgDcbevt1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Latched Status Flag for Digital Compare Output B Event 2 0 Indicates no trip event has occurred on DCBEVT2 1 Indicates a trip event has occurred for the event defined for DCBEVT2"]
    #[inline(always)]
    #[must_use]
    pub fn tzflg_dcbevt2(&mut self) -> TzflgDcbevt2W<TzflgTzclrSpec> {
        TzflgDcbevt2W::new(self, 6)
    }
    #[doc = "Bits 7:15 - 15:7\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<TzflgTzclrSpec> {
        Reserved2W::new(self, 7)
    }
    #[doc = "Bit 16 - 16:16\\]
Global Interrupt Clear Flag 0 Has no effect. Always reads back a 0. 1 Clears the trip-interrupt flag for this ePWM module (TZFLG\\[INT\\]). NOTE: No further EPWMx_TZINT VIM interrupts will be generated until the flag is cleared. If the TZFLG\\[INT\\]
bit is cleared and any of the other flag bits are set, then another interrupt pulse will be generated. Clearing all flag bits will prevent further interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn tzclr_int(&mut self) -> TzclrIntW<TzflgTzclrSpec> {
        TzclrIntW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Clear Flag for Cycle-By-Cycle (CBC) Trip Latch 0 Has no effect. Always reads back a 0. 1 Clears this Trip (set) condition."]
    #[inline(always)]
    #[must_use]
    pub fn tzclr_cbc(&mut self) -> TzclrCbcW<TzflgTzclrSpec> {
        TzclrCbcW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Clear Flag for One-Shot Trip (OST) Latch 0 Has no effect. Always reads back a 0. 1 Clears this Trip (set) condition."]
    #[inline(always)]
    #[must_use]
    pub fn tzclr_ost(&mut self) -> TzclrOstW<TzflgTzclrSpec> {
        TzclrOstW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Clear Flag for Digital Compare Output A Event 1 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCAEVT1 event trip condition."]
    #[inline(always)]
    #[must_use]
    pub fn tzclr_dcaevt1(&mut self) -> TzclrDcaevt1W<TzflgTzclrSpec> {
        TzclrDcaevt1W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Clear Flag for Digital Compare Output A Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCAEVT2 event trip condition"]
    #[inline(always)]
    #[must_use]
    pub fn tzclr_dcaevt2(&mut self) -> TzclrDcaevt2W<TzflgTzclrSpec> {
        TzclrDcaevt2W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Clear Flag for Digital Compare Output B Event 1 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCBEVT1 event trip condition"]
    #[inline(always)]
    #[must_use]
    pub fn tzclr_dcbevt1(&mut self) -> TzclrDcbevt1W<TzflgTzclrSpec> {
        TzclrDcbevt1W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Clear Flag for Digital Compare Output B Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 clears the DCBEVT2 event trip condition."]
    #[inline(always)]
    #[must_use]
    pub fn tzclr_dcbevt2(&mut self) -> TzclrDcbevt2W<TzflgTzclrSpec> {
        TzclrDcbevt2W::new(self, 22)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<TzflgTzclrSpec> {
        Reserved1W::new(self, 23)
    }
}
#[doc = "Trip-Zone Flag Register/ Trip-Zone Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzflg_tzclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzflg_tzclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzflgTzclrSpec;
impl crate::RegisterSpec for TzflgTzclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzflg_tzclr::R`](R) reader structure"]
impl crate::Readable for TzflgTzclrSpec {}
#[doc = "`write(|w| ..)` method takes [`tzflg_tzclr::W`](W) writer structure"]
impl crate::Writable for TzflgTzclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZFLG_TZCLR to value 0"]
impl crate::Resettable for TzflgTzclrSpec {
    const RESET_VALUE: u32 = 0;
}
