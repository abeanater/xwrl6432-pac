#[doc = "Register `ONEMCU_TPIU_CIDR0` reader"]
pub type R = crate::R<OnemcuTpiuCidr0Spec>;
#[doc = "Register `ONEMCU_TPIU_CIDR0` writer"]
pub type W = crate::W<OnemcuTpiuCidr0Spec>;
#[doc = "Field `ONEMCU_TPIU_CIDR0` reader - 31:0\\]
Component ID0"]
pub type OnemcuTpiuCidr0R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_CIDR0` writer - 31:0\\]
Component ID0"]
pub type OnemcuTpiuCidr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Component ID0"]
    #[inline(always)]
    pub fn onemcu_tpiu_cidr0(&self) -> OnemcuTpiuCidr0R {
        OnemcuTpiuCidr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Component ID0"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_cidr0(&mut self) -> OnemcuTpiuCidr0W<OnemcuTpiuCidr0Spec> {
        OnemcuTpiuCidr0W::new(self, 0)
    }
}
#[doc = "Component ID0\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_cidr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_cidr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuCidr0Spec;
impl crate::RegisterSpec for OnemcuTpiuCidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_cidr0::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuCidr0Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_cidr0::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuCidr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_CIDR0 to value 0"]
impl crate::Resettable for OnemcuTpiuCidr0Spec {
    const RESET_VALUE: u32 = 0;
}
