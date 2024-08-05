#[doc = "Register `HWASS_RAM_160KB_CLOCK_GATE` reader"]
pub type R = crate::R<HwassRam160kbClockGateSpec>;
#[doc = "Register `HWASS_RAM_160KB_CLOCK_GATE` writer"]
pub type W = crate::W<HwassRam160kbClockGateSpec>;
#[doc = "Field `enable` reader - 2:0\\]
3'b000 : Ungate clock to 160KB RAM 3'b111 : Gate Clock to 160KB RAM"]
pub type EnableR = crate::FieldReader;
#[doc = "Field `enable` writer - 2:0\\]
3'b000 : Ungate clock to 160KB RAM 3'b111 : Gate Clock to 160KB RAM"]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
3'b000 : Ungate clock to 160KB RAM 3'b111 : Gate Clock to 160KB RAM"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
3'b000 : Ungate clock to 160KB RAM 3'b111 : Gate Clock to 160KB RAM"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<HwassRam160kbClockGateSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "HWASS_RAM_160KB_CLOCK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_ram_160kb_clock_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_ram_160kb_clock_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwassRam160kbClockGateSpec;
impl crate::RegisterSpec for HwassRam160kbClockGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwass_ram_160kb_clock_gate::R`](R) reader structure"]
impl crate::Readable for HwassRam160kbClockGateSpec {}
#[doc = "`write(|w| ..)` method takes [`hwass_ram_160kb_clock_gate::W`](W) writer structure"]
impl crate::Writable for HwassRam160kbClockGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWASS_RAM_160KB_CLOCK_GATE to value 0"]
impl crate::Resettable for HwassRam160kbClockGateSpec {
    const RESET_VALUE: u32 = 0;
}
