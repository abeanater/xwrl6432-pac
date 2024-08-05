#[doc = "Register `ONEMCU_TPIU_PIDR5` reader"]
pub type R = crate::R<OnemcuTpiuPidr5Spec>;
#[doc = "Register `ONEMCU_TPIU_PIDR5` writer"]
pub type W = crate::W<OnemcuTpiuPidr5Spec>;
#[doc = "Field `ONEMCU_TPIU_PIDR5` reader - 31:0\\]
Peripheral ID5"]
pub type OnemcuTpiuPidr5R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_PIDR5` writer - 31:0\\]
Peripheral ID5"]
pub type OnemcuTpiuPidr5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Peripheral ID5"]
    #[inline(always)]
    pub fn onemcu_tpiu_pidr5(&self) -> OnemcuTpiuPidr5R {
        OnemcuTpiuPidr5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Peripheral ID5"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_pidr5(&mut self) -> OnemcuTpiuPidr5W<OnemcuTpiuPidr5Spec> {
        OnemcuTpiuPidr5W::new(self, 0)
    }
}
#[doc = "Peripheral ID5\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuPidr5Spec;
impl crate::RegisterSpec for OnemcuTpiuPidr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_pidr5::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuPidr5Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_pidr5::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuPidr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_PIDR5 to value 0"]
impl crate::Resettable for OnemcuTpiuPidr5Spec {
    const RESET_VALUE: u32 = 0;
}
