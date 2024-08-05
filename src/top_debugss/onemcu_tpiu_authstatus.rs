#[doc = "Register `ONEMCU_TPIU_AUTHSTATUS` reader"]
pub type R = crate::R<OnemcuTpiuAuthstatusSpec>;
#[doc = "Register `ONEMCU_TPIU_AUTHSTATUS` writer"]
pub type W = crate::W<OnemcuTpiuAuthstatusSpec>;
#[doc = "Field `ONEMCU_TPIU_AUTHSTATUS` reader - 31:0\\]
Authentication status"]
pub type OnemcuTpiuAuthstatusR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_AUTHSTATUS` writer - 31:0\\]
Authentication status"]
pub type OnemcuTpiuAuthstatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Authentication status"]
    #[inline(always)]
    pub fn onemcu_tpiu_authstatus(&self) -> OnemcuTpiuAuthstatusR {
        OnemcuTpiuAuthstatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Authentication status"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_authstatus(&mut self) -> OnemcuTpiuAuthstatusW<OnemcuTpiuAuthstatusSpec> {
        OnemcuTpiuAuthstatusW::new(self, 0)
    }
}
#[doc = "Authentication status\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_authstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_authstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuAuthstatusSpec;
impl crate::RegisterSpec for OnemcuTpiuAuthstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_authstatus::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuAuthstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_authstatus::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuAuthstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_AUTHSTATUS to value 0"]
impl crate::Resettable for OnemcuTpiuAuthstatusSpec {
    const RESET_VALUE: u32 = 0;
}
