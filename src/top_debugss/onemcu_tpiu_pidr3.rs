#[doc = "Register `ONEMCU_TPIU_PIDR3` reader"]
pub type R = crate::R<OnemcuTpiuPidr3Spec>;
#[doc = "Register `ONEMCU_TPIU_PIDR3` writer"]
pub type W = crate::W<OnemcuTpiuPidr3Spec>;
#[doc = "Field `ONEMCU_TPIU_PIDR3` reader - 31:0\\]
Peripheral ID3"]
pub type OnemcuTpiuPidr3R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_PIDR3` writer - 31:0\\]
Peripheral ID3"]
pub type OnemcuTpiuPidr3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Peripheral ID3"]
    #[inline(always)]
    pub fn onemcu_tpiu_pidr3(&self) -> OnemcuTpiuPidr3R {
        OnemcuTpiuPidr3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Peripheral ID3"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_pidr3(&mut self) -> OnemcuTpiuPidr3W<OnemcuTpiuPidr3Spec> {
        OnemcuTpiuPidr3W::new(self, 0)
    }
}
#[doc = "Peripheral ID3\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuPidr3Spec;
impl crate::RegisterSpec for OnemcuTpiuPidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_pidr3::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuPidr3Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_pidr3::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuPidr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_PIDR3 to value 0"]
impl crate::Resettable for OnemcuTpiuPidr3Spec {
    const RESET_VALUE: u32 = 0;
}
