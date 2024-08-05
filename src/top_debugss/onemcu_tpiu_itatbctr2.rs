#[doc = "Register `ONEMCU_TPIU_ITATBCTR2` reader"]
pub type R = crate::R<OnemcuTpiuItatbctr2Spec>;
#[doc = "Register `ONEMCU_TPIU_ITATBCTR2` writer"]
pub type W = crate::W<OnemcuTpiuItatbctr2Spec>;
#[doc = "Field `ONEMCU_TPIU_ITATBCTR2` reader - 31:0\\]
Integration Register, ITATBCTR2"]
pub type OnemcuTpiuItatbctr2R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_ITATBCTR2` writer - 31:0\\]
Integration Register, ITATBCTR2"]
pub type OnemcuTpiuItatbctr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Integration Register, ITATBCTR2"]
    #[inline(always)]
    pub fn onemcu_tpiu_itatbctr2(&self) -> OnemcuTpiuItatbctr2R {
        OnemcuTpiuItatbctr2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Integration Register, ITATBCTR2"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_itatbctr2(&mut self) -> OnemcuTpiuItatbctr2W<OnemcuTpiuItatbctr2Spec> {
        OnemcuTpiuItatbctr2W::new(self, 0)
    }
}
#[doc = "Integration Register, ITATBCTR2\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_itatbctr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_itatbctr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuItatbctr2Spec;
impl crate::RegisterSpec for OnemcuTpiuItatbctr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_itatbctr2::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuItatbctr2Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_itatbctr2::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuItatbctr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_ITATBCTR2 to value 0"]
impl crate::Resettable for OnemcuTpiuItatbctr2Spec {
    const RESET_VALUE: u32 = 0;
}
