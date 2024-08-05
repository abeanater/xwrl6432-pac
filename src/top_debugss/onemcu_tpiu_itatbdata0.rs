#[doc = "Register `ONEMCU_TPIU_ITATBDATA0` reader"]
pub type R = crate::R<OnemcuTpiuItatbdata0Spec>;
#[doc = "Register `ONEMCU_TPIU_ITATBDATA0` writer"]
pub type W = crate::W<OnemcuTpiuItatbdata0Spec>;
#[doc = "Field `ONEMCU_TPIU_ITATBDATA0` reader - 31:0\\]
Integration Register, ITATBDATA0"]
pub type OnemcuTpiuItatbdata0R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_ITATBDATA0` writer - 31:0\\]
Integration Register, ITATBDATA0"]
pub type OnemcuTpiuItatbdata0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Integration Register, ITATBDATA0"]
    #[inline(always)]
    pub fn onemcu_tpiu_itatbdata0(&self) -> OnemcuTpiuItatbdata0R {
        OnemcuTpiuItatbdata0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Integration Register, ITATBDATA0"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_itatbdata0(&mut self) -> OnemcuTpiuItatbdata0W<OnemcuTpiuItatbdata0Spec> {
        OnemcuTpiuItatbdata0W::new(self, 0)
    }
}
#[doc = "Integration Register, ITATBDATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_itatbdata0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_itatbdata0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuItatbdata0Spec;
impl crate::RegisterSpec for OnemcuTpiuItatbdata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_itatbdata0::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuItatbdata0Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_itatbdata0::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuItatbdata0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_ITATBDATA0 to value 0"]
impl crate::Resettable for OnemcuTpiuItatbdata0Spec {
    const RESET_VALUE: u32 = 0;
}
