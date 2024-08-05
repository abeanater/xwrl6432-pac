#[doc = "Register `ONEMCU_CTI_CHOUTSTATUS` reader"]
pub type R = crate::R<OnemcuCtiChoutstatusSpec>;
#[doc = "Register `ONEMCU_CTI_CHOUTSTATUS` writer"]
pub type W = crate::W<OnemcuCtiChoutstatusSpec>;
#[doc = "Field `ONEMCU_CTI_CHOUTSTATUS` reader - "]
pub type OnemcuCtiChoutstatusR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_CHOUTSTATUS` writer - "]
pub type OnemcuCtiChoutstatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_choutstatus(&self) -> OnemcuCtiChoutstatusR {
        OnemcuCtiChoutstatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_choutstatus(&mut self) -> OnemcuCtiChoutstatusW<OnemcuCtiChoutstatusSpec> {
        OnemcuCtiChoutstatusW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_CHOUTSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_choutstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_choutstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiChoutstatusSpec;
impl crate::RegisterSpec for OnemcuCtiChoutstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_choutstatus::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiChoutstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_choutstatus::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiChoutstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_CHOUTSTATUS to value 0"]
impl crate::Resettable for OnemcuCtiChoutstatusSpec {
    const RESET_VALUE: u32 = 0;
}
