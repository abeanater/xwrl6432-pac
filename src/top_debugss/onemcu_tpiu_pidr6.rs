#[doc = "Register `ONEMCU_TPIU_PIDR6` reader"]
pub type R = crate::R<OnemcuTpiuPidr6Spec>;
#[doc = "Register `ONEMCU_TPIU_PIDR6` writer"]
pub type W = crate::W<OnemcuTpiuPidr6Spec>;
#[doc = "Field `ONEMCU_TPIU_PIDR6` reader - 31:0\\]
Peripheral ID6"]
pub type OnemcuTpiuPidr6R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_PIDR6` writer - 31:0\\]
Peripheral ID6"]
pub type OnemcuTpiuPidr6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Peripheral ID6"]
    #[inline(always)]
    pub fn onemcu_tpiu_pidr6(&self) -> OnemcuTpiuPidr6R {
        OnemcuTpiuPidr6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Peripheral ID6"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_pidr6(&mut self) -> OnemcuTpiuPidr6W<OnemcuTpiuPidr6Spec> {
        OnemcuTpiuPidr6W::new(self, 0)
    }
}
#[doc = "Peripheral ID6\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuPidr6Spec;
impl crate::RegisterSpec for OnemcuTpiuPidr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_pidr6::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuPidr6Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_pidr6::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuPidr6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_PIDR6 to value 0"]
impl crate::Resettable for OnemcuTpiuPidr6Spec {
    const RESET_VALUE: u32 = 0;
}
