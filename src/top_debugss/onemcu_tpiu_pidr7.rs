#[doc = "Register `ONEMCU_TPIU_PIDR7` reader"]
pub type R = crate::R<OnemcuTpiuPidr7Spec>;
#[doc = "Register `ONEMCU_TPIU_PIDR7` writer"]
pub type W = crate::W<OnemcuTpiuPidr7Spec>;
#[doc = "Field `ONEMCU_TPIU_PIDR7` reader - 31:0\\]
Peripheral ID7"]
pub type OnemcuTpiuPidr7R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_PIDR7` writer - 31:0\\]
Peripheral ID7"]
pub type OnemcuTpiuPidr7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Peripheral ID7"]
    #[inline(always)]
    pub fn onemcu_tpiu_pidr7(&self) -> OnemcuTpiuPidr7R {
        OnemcuTpiuPidr7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Peripheral ID7"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_pidr7(&mut self) -> OnemcuTpiuPidr7W<OnemcuTpiuPidr7Spec> {
        OnemcuTpiuPidr7W::new(self, 0)
    }
}
#[doc = "Peripheral ID7\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuPidr7Spec;
impl crate::RegisterSpec for OnemcuTpiuPidr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_pidr7::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuPidr7Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_pidr7::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuPidr7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_PIDR7 to value 0"]
impl crate::Resettable for OnemcuTpiuPidr7Spec {
    const RESET_VALUE: u32 = 0;
}
