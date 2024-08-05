#[doc = "Register `ONEMCU_TPIU_PIDR0` reader"]
pub type R = crate::R<OnemcuTpiuPidr0Spec>;
#[doc = "Register `ONEMCU_TPIU_PIDR0` writer"]
pub type W = crate::W<OnemcuTpiuPidr0Spec>;
#[doc = "Field `ONEMCU_TPIU_PIDR0` reader - 31:0\\]
Peripheral ID0"]
pub type OnemcuTpiuPidr0R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_PIDR0` writer - 31:0\\]
Peripheral ID0"]
pub type OnemcuTpiuPidr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Peripheral ID0"]
    #[inline(always)]
    pub fn onemcu_tpiu_pidr0(&self) -> OnemcuTpiuPidr0R {
        OnemcuTpiuPidr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Peripheral ID0"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_pidr0(&mut self) -> OnemcuTpiuPidr0W<OnemcuTpiuPidr0Spec> {
        OnemcuTpiuPidr0W::new(self, 0)
    }
}
#[doc = "Peripheral ID0\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuPidr0Spec;
impl crate::RegisterSpec for OnemcuTpiuPidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_pidr0::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuPidr0Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_pidr0::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuPidr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_PIDR0 to value 0"]
impl crate::Resettable for OnemcuTpiuPidr0Spec {
    const RESET_VALUE: u32 = 0;
}
