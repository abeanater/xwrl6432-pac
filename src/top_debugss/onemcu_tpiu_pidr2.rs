#[doc = "Register `ONEMCU_TPIU_PIDR2` reader"]
pub type R = crate::R<OnemcuTpiuPidr2Spec>;
#[doc = "Register `ONEMCU_TPIU_PIDR2` writer"]
pub type W = crate::W<OnemcuTpiuPidr2Spec>;
#[doc = "Field `ONEMCU_TPIU_PIDR2` reader - 31:0\\]
Peripheral ID2"]
pub type OnemcuTpiuPidr2R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_PIDR2` writer - 31:0\\]
Peripheral ID2"]
pub type OnemcuTpiuPidr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Peripheral ID2"]
    #[inline(always)]
    pub fn onemcu_tpiu_pidr2(&self) -> OnemcuTpiuPidr2R {
        OnemcuTpiuPidr2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Peripheral ID2"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_pidr2(&mut self) -> OnemcuTpiuPidr2W<OnemcuTpiuPidr2Spec> {
        OnemcuTpiuPidr2W::new(self, 0)
    }
}
#[doc = "Peripheral ID2\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuPidr2Spec;
impl crate::RegisterSpec for OnemcuTpiuPidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_pidr2::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuPidr2Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_pidr2::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuPidr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_PIDR2 to value 0"]
impl crate::Resettable for OnemcuTpiuPidr2Spec {
    const RESET_VALUE: u32 = 0;
}
