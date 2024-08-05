#[doc = "Register `ONEMCU_TPIU_ITATBCTR1` reader"]
pub type R = crate::R<OnemcuTpiuItatbctr1Spec>;
#[doc = "Register `ONEMCU_TPIU_ITATBCTR1` writer"]
pub type W = crate::W<OnemcuTpiuItatbctr1Spec>;
#[doc = "Field `ONEMCU_TPIU_ITATBCTR1` reader - 31:0\\]
Integration Register, ITATBCTR1"]
pub type OnemcuTpiuItatbctr1R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_ITATBCTR1` writer - 31:0\\]
Integration Register, ITATBCTR1"]
pub type OnemcuTpiuItatbctr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Integration Register, ITATBCTR1"]
    #[inline(always)]
    pub fn onemcu_tpiu_itatbctr1(&self) -> OnemcuTpiuItatbctr1R {
        OnemcuTpiuItatbctr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Integration Register, ITATBCTR1"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_itatbctr1(&mut self) -> OnemcuTpiuItatbctr1W<OnemcuTpiuItatbctr1Spec> {
        OnemcuTpiuItatbctr1W::new(self, 0)
    }
}
#[doc = "Integration Register, ITATBCTR1\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_itatbctr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_itatbctr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuItatbctr1Spec;
impl crate::RegisterSpec for OnemcuTpiuItatbctr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_itatbctr1::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuItatbctr1Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_itatbctr1::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuItatbctr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_ITATBCTR1 to value 0"]
impl crate::Resettable for OnemcuTpiuItatbctr1Spec {
    const RESET_VALUE: u32 = 0;
}
