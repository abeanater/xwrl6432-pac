#[doc = "Register `ONEMCU_CTI_TRIGOUTSTATUS` reader"]
pub type R = crate::R<OnemcuCtiTrigoutstatusSpec>;
#[doc = "Register `ONEMCU_CTI_TRIGOUTSTATUS` writer"]
pub type W = crate::W<OnemcuCtiTrigoutstatusSpec>;
#[doc = "Field `ONEMCU_CTI_TRIGOUTSTATUS` reader - "]
pub type OnemcuCtiTrigoutstatusR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_TRIGOUTSTATUS` writer - "]
pub type OnemcuCtiTrigoutstatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_trigoutstatus(&self) -> OnemcuCtiTrigoutstatusR {
        OnemcuCtiTrigoutstatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_trigoutstatus(
        &mut self,
    ) -> OnemcuCtiTrigoutstatusW<OnemcuCtiTrigoutstatusSpec> {
        OnemcuCtiTrigoutstatusW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_TRIGOUTSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_trigoutstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_trigoutstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiTrigoutstatusSpec;
impl crate::RegisterSpec for OnemcuCtiTrigoutstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_trigoutstatus::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiTrigoutstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_trigoutstatus::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiTrigoutstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_TRIGOUTSTATUS to value 0"]
impl crate::Resettable for OnemcuCtiTrigoutstatusSpec {
    const RESET_VALUE: u32 = 0;
}
