#[doc = "Register `ONEMCU_TPIU_CIDR1` reader"]
pub type R = crate::R<OnemcuTpiuCidr1Spec>;
#[doc = "Register `ONEMCU_TPIU_CIDR1` writer"]
pub type W = crate::W<OnemcuTpiuCidr1Spec>;
#[doc = "Field `ONEMCU_TPIU_CIDR1` reader - 31:0\\]
Component ID1"]
pub type OnemcuTpiuCidr1R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_CIDR1` writer - 31:0\\]
Component ID1"]
pub type OnemcuTpiuCidr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Component ID1"]
    #[inline(always)]
    pub fn onemcu_tpiu_cidr1(&self) -> OnemcuTpiuCidr1R {
        OnemcuTpiuCidr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Component ID1"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_cidr1(&mut self) -> OnemcuTpiuCidr1W<OnemcuTpiuCidr1Spec> {
        OnemcuTpiuCidr1W::new(self, 0)
    }
}
#[doc = "Component ID1\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_cidr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_cidr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuCidr1Spec;
impl crate::RegisterSpec for OnemcuTpiuCidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_cidr1::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuCidr1Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_cidr1::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuCidr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_CIDR1 to value 0"]
impl crate::Resettable for OnemcuTpiuCidr1Spec {
    const RESET_VALUE: u32 = 0;
}
