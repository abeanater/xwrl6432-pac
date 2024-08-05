#[doc = "Register `PBIST_FSFR0` reader"]
pub type R = crate::R<PbistFsfr0Spec>;
#[doc = "Register `PBIST_FSFR0` writer"]
pub type W = crate::W<PbistFsfr0Spec>;
#[doc = "Field `PBIST_FSFR0` reader - 0:0\\]
Fail Status Fail Register- Port 0 This register indicates if a failure occurred during a memory self-test. Value 0 = No failure occurred Value 1 = Indicates a failure"]
pub type PbistFsfr0R = crate::BitReader;
#[doc = "Field `PBIST_FSFR0` writer - 0:0\\]
Fail Status Fail Register- Port 0 This register indicates if a failure occurred during a memory self-test. Value 0 = No failure occurred Value 1 = Indicates a failure"]
pub type PbistFsfr0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Fail Status Fail Register- Port 0 This register indicates if a failure occurred during a memory self-test. Value 0 = No failure occurred Value 1 = Indicates a failure"]
    #[inline(always)]
    pub fn pbist_fsfr0(&self) -> PbistFsfr0R {
        PbistFsfr0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Fail Status Fail Register- Port 0 This register indicates if a failure occurred during a memory self-test. Value 0 = No failure occurred Value 1 = Indicates a failure"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_fsfr0(&mut self) -> PbistFsfr0W<PbistFsfr0Spec> {
        PbistFsfr0W::new(self, 0)
    }
}
#[doc = "Fail status fail - port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsfr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsfr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistFsfr0Spec;
impl crate::RegisterSpec for PbistFsfr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_fsfr0::R`](R) reader structure"]
impl crate::Readable for PbistFsfr0Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_fsfr0::W`](W) writer structure"]
impl crate::Writable for PbistFsfr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_FSFR0 to value 0"]
impl crate::Resettable for PbistFsfr0Spec {
    const RESET_VALUE: u32 = 0;
}
