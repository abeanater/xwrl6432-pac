#[doc = "Register `ONEMCU_TPIU_PIDR4` reader"]
pub type R = crate::R<OnemcuTpiuPidr4Spec>;
#[doc = "Register `ONEMCU_TPIU_PIDR4` writer"]
pub type W = crate::W<OnemcuTpiuPidr4Spec>;
#[doc = "Field `ONEMCU_TPIU_PIDR4` reader - 31:0\\]
Peripheral ID4"]
pub type OnemcuTpiuPidr4R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_PIDR4` writer - 31:0\\]
Peripheral ID4"]
pub type OnemcuTpiuPidr4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Peripheral ID4"]
    #[inline(always)]
    pub fn onemcu_tpiu_pidr4(&self) -> OnemcuTpiuPidr4R {
        OnemcuTpiuPidr4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Peripheral ID4"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_pidr4(&mut self) -> OnemcuTpiuPidr4W<OnemcuTpiuPidr4Spec> {
        OnemcuTpiuPidr4W::new(self, 0)
    }
}
#[doc = "Peripheral ID4\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuPidr4Spec;
impl crate::RegisterSpec for OnemcuTpiuPidr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_pidr4::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuPidr4Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_pidr4::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuPidr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_PIDR4 to value 0"]
impl crate::Resettable for OnemcuTpiuPidr4Spec {
    const RESET_VALUE: u32 = 0;
}
