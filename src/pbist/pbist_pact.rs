#[doc = "Register `PBIST_PACT` reader"]
pub type R = crate::R<PbistPactSpec>;
#[doc = "Register `PBIST_PACT` writer"]
pub type W = crate::W<PbistPactSpec>;
#[doc = "Field `PBIST_PACT` reader - 0:0\\]
Pbist Active/ROM Clock Enable Register \\[0\\]: This bit must be set to turn on internal PBIST clocks. Setting this bit asserts an internal signal that is used as the clock gate enable. As long as this bit is 0, any access to PBIST will not go through, and PBIST will remain in an almost zero-power mode. Value 0 = Disable internal PBIST clocks Value 1 = Enable internal PBIST clocks"]
pub type PbistPactR = crate::BitReader;
#[doc = "Field `PBIST_PACT` writer - 0:0\\]
Pbist Active/ROM Clock Enable Register \\[0\\]: This bit must be set to turn on internal PBIST clocks. Setting this bit asserts an internal signal that is used as the clock gate enable. As long as this bit is 0, any access to PBIST will not go through, and PBIST will remain in an almost zero-power mode. Value 0 = Disable internal PBIST clocks Value 1 = Enable internal PBIST clocks"]
pub type PbistPactW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Pbist Active/ROM Clock Enable Register \\[0\\]: This bit must be set to turn on internal PBIST clocks. Setting this bit asserts an internal signal that is used as the clock gate enable. As long as this bit is 0, any access to PBIST will not go through, and PBIST will remain in an almost zero-power mode. Value 0 = Disable internal PBIST clocks Value 1 = Enable internal PBIST clocks"]
    #[inline(always)]
    pub fn pbist_pact(&self) -> PbistPactR {
        PbistPactR::new(bool(self.bits))
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Pbist Active/ROM Clock Enable Register \\[0\\]: This bit must be set to turn on internal PBIST clocks. Setting this bit asserts an internal signal that is used as the clock gate enable. As long as this bit is 0, any access to PBIST will not go through, and PBIST will remain in an almost zero-power mode. Value 0 = Disable internal PBIST clocks Value 1 = Enable internal PBIST clocks"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_pact(&mut self) -> PbistPactW<PbistPactSpec> {
        PbistPactW::new(self, 0)
    }
}
#[doc = "Pbist Active\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_pact::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_pact::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistPactSpec;
impl crate::RegisterSpec for PbistPactSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pbist_pact::R`](R) reader structure"]
impl crate::Readable for PbistPactSpec {}
#[doc = "`write(|w| ..)` method takes [`pbist_pact::W`](W) writer structure"]
impl crate::Writable for PbistPactSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PBIST_PACT to value 0"]
impl crate::Resettable for PbistPactSpec {
    const RESET_VALUE: u8 = 0;
}
