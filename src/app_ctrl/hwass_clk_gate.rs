#[doc = "Register `HWASS_CLK_GATE` reader"]
pub type R = crate::R<HwassClkGateSpec>;
#[doc = "Register `HWASS_CLK_GATE` writer"]
pub type W = crate::W<HwassClkGateSpec>;
#[doc = "Field `enable` reader - 2:0\\]
RESERVED"]
pub type EnableR = crate::FieldReader;
#[doc = "Field `enable` writer - 2:0\\]
RESERVED"]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
RESERVED"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<HwassClkGateSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "HWASS_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwassClkGateSpec;
impl crate::RegisterSpec for HwassClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwass_clk_gate::R`](R) reader structure"]
impl crate::Readable for HwassClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`hwass_clk_gate::W`](W) writer structure"]
impl crate::Writable for HwassClkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWASS_CLK_GATE to value 0"]
impl crate::Resettable for HwassClkGateSpec {
    const RESET_VALUE: u32 = 0;
}
