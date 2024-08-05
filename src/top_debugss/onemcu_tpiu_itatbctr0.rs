#[doc = "Register `ONEMCU_TPIU_ITATBCTR0` reader"]
pub type R = crate::R<OnemcuTpiuItatbctr0Spec>;
#[doc = "Register `ONEMCU_TPIU_ITATBCTR0` writer"]
pub type W = crate::W<OnemcuTpiuItatbctr0Spec>;
#[doc = "Field `ONEMCU_TPIU_ITATBCTR0` reader - 31:0\\]
Integration Register, ITATBCTR0"]
pub type OnemcuTpiuItatbctr0R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_ITATBCTR0` writer - 31:0\\]
Integration Register, ITATBCTR0"]
pub type OnemcuTpiuItatbctr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Integration Register, ITATBCTR0"]
    #[inline(always)]
    pub fn onemcu_tpiu_itatbctr0(&self) -> OnemcuTpiuItatbctr0R {
        OnemcuTpiuItatbctr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Integration Register, ITATBCTR0"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_itatbctr0(&mut self) -> OnemcuTpiuItatbctr0W<OnemcuTpiuItatbctr0Spec> {
        OnemcuTpiuItatbctr0W::new(self, 0)
    }
}
#[doc = "Integration Register, ITATBCTR0\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_itatbctr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_itatbctr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuItatbctr0Spec;
impl crate::RegisterSpec for OnemcuTpiuItatbctr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_itatbctr0::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuItatbctr0Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_itatbctr0::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuItatbctr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_ITATBCTR0 to value 0"]
impl crate::Resettable for OnemcuTpiuItatbctr0Spec {
    const RESET_VALUE: u32 = 0;
}
