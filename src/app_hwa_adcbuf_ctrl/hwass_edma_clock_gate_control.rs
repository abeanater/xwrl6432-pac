#[doc = "Register `HWASS_EDMA_CLOCK_GATE_CONTROL` reader"]
pub type R = crate::R<HwassEdmaClockGateControlSpec>;
#[doc = "Register `HWASS_EDMA_CLOCK_GATE_CONTROL` writer"]
pub type W = crate::W<HwassEdmaClockGateControlSpec>;
#[doc = "Field `hwa_edma_clock_gating_en` reader - 2:0\\]
Writing 3'b111 will gate the clock to HWA EDMA.Writing 3'b000 will ungate the clock"]
pub type HwaEdmaClockGatingEnR = crate::FieldReader;
#[doc = "Field `hwa_edma_clock_gating_en` writer - 2:0\\]
Writing 3'b111 will gate the clock to HWA EDMA.Writing 3'b000 will ungate the clock"]
pub type HwaEdmaClockGatingEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Writing 3'b111 will gate the clock to HWA EDMA.Writing 3'b000 will ungate the clock"]
    #[inline(always)]
    pub fn hwa_edma_clock_gating_en(&self) -> HwaEdmaClockGatingEnR {
        HwaEdmaClockGatingEnR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Writing 3'b111 will gate the clock to HWA EDMA.Writing 3'b000 will ungate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_edma_clock_gating_en(
        &mut self,
    ) -> HwaEdmaClockGatingEnW<HwassEdmaClockGateControlSpec> {
        HwaEdmaClockGatingEnW::new(self, 0)
    }
}
#[doc = "HWASS_EDMA_CLOCK_GATE_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_edma_clock_gate_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_edma_clock_gate_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwassEdmaClockGateControlSpec;
impl crate::RegisterSpec for HwassEdmaClockGateControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwass_edma_clock_gate_control::R`](R) reader structure"]
impl crate::Readable for HwassEdmaClockGateControlSpec {}
#[doc = "`write(|w| ..)` method takes [`hwass_edma_clock_gate_control::W`](W) writer structure"]
impl crate::Writable for HwassEdmaClockGateControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWASS_EDMA_CLOCK_GATE_CONTROL to value 0"]
impl crate::Resettable for HwassEdmaClockGateControlSpec {
    const RESET_VALUE: u32 = 0;
}
