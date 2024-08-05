#[doc = "Register `PBIST_OVR` reader"]
pub type R = crate::R<PbistOvrSpec>;
#[doc = "Register `PBIST_OVR` writer"]
pub type W = crate::W<PbistOvrSpec>;
#[doc = "Field `PBIST_OVR` reader - 3:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type PbistOvrR = crate::FieldReader;
#[doc = "Field `PBIST_OVR` writer - 3:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type PbistOvrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn pbist_ovr(&self) -> PbistOvrR {
        PbistOvrR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_ovr(&mut self) -> PbistOvrW<PbistOvrSpec> {
        PbistOvrW::new(self, 0)
    }
}
#[doc = "PBIST Overrides\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ovr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ovr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistOvrSpec;
impl crate::RegisterSpec for PbistOvrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pbist_ovr::R`](R) reader structure"]
impl crate::Readable for PbistOvrSpec {}
#[doc = "`write(|w| ..)` method takes [`pbist_ovr::W`](W) writer structure"]
impl crate::Writable for PbistOvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PBIST_OVR to value 0"]
impl crate::Resettable for PbistOvrSpec {
    const RESET_VALUE: u8 = 0;
}
