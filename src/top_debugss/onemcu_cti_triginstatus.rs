#[doc = "Register `ONEMCU_CTI_TRIGINSTATUS` reader"]
pub type R = crate::R<OnemcuCtiTriginstatusSpec>;
#[doc = "Register `ONEMCU_CTI_TRIGINSTATUS` writer"]
pub type W = crate::W<OnemcuCtiTriginstatusSpec>;
#[doc = "Field `ONEMCU_CTI_TRIGINSTATUS` reader - "]
pub type OnemcuCtiTriginstatusR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_TRIGINSTATUS` writer - "]
pub type OnemcuCtiTriginstatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_triginstatus(&self) -> OnemcuCtiTriginstatusR {
        OnemcuCtiTriginstatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_triginstatus(&mut self) -> OnemcuCtiTriginstatusW<OnemcuCtiTriginstatusSpec> {
        OnemcuCtiTriginstatusW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_TRIGINSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_triginstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_triginstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiTriginstatusSpec;
impl crate::RegisterSpec for OnemcuCtiTriginstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_triginstatus::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiTriginstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_triginstatus::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiTriginstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_TRIGINSTATUS to value 0"]
impl crate::Resettable for OnemcuCtiTriginstatusSpec {
    const RESET_VALUE: u32 = 0;
}
