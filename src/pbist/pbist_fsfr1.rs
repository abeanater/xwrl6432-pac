#[doc = "Register `PBIST_FSFR1` reader"]
pub type R = crate::R<PbistFsfr1Spec>;
#[doc = "Register `PBIST_FSFR1` writer"]
pub type W = crate::W<PbistFsfr1Spec>;
#[doc = "Field `PBIST_FSFR1` reader - 0:0\\]
Fail Status Fail Register- Port 1 This register indicates if a failure occurred during a memory self-test. Value 0 = No failure occurred Value 1 = Indicates a failure"]
pub type PbistFsfr1R = crate::BitReader;
#[doc = "Field `PBIST_FSFR1` writer - 0:0\\]
Fail Status Fail Register- Port 1 This register indicates if a failure occurred during a memory self-test. Value 0 = No failure occurred Value 1 = Indicates a failure"]
pub type PbistFsfr1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Fail Status Fail Register- Port 1 This register indicates if a failure occurred during a memory self-test. Value 0 = No failure occurred Value 1 = Indicates a failure"]
    #[inline(always)]
    pub fn pbist_fsfr1(&self) -> PbistFsfr1R {
        PbistFsfr1R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Fail Status Fail Register- Port 1 This register indicates if a failure occurred during a memory self-test. Value 0 = No failure occurred Value 1 = Indicates a failure"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_fsfr1(&mut self) -> PbistFsfr1W<PbistFsfr1Spec> {
        PbistFsfr1W::new(self, 0)
    }
}
#[doc = "Fail status fail - port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsfr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsfr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistFsfr1Spec;
impl crate::RegisterSpec for PbistFsfr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_fsfr1::R`](R) reader structure"]
impl crate::Readable for PbistFsfr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_fsfr1::W`](W) writer structure"]
impl crate::Writable for PbistFsfr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_FSFR1 to value 0"]
impl crate::Resettable for PbistFsfr1Spec {
    const RESET_VALUE: u32 = 0;
}
