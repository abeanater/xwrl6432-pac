#[doc = "Register `ONEMCU_TPIU_ITTRFLINACK` reader"]
pub type R = crate::R<OnemcuTpiuIttrflinackSpec>;
#[doc = "Register `ONEMCU_TPIU_ITTRFLINACK` writer"]
pub type W = crate::W<OnemcuTpiuIttrflinackSpec>;
#[doc = "Field `ONEMCU_TPIU_ITTRFLINACK` reader - 31:0\\]
Integration Register, ITTRFLINACK"]
pub type OnemcuTpiuIttrflinackR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_ITTRFLINACK` writer - 31:0\\]
Integration Register, ITTRFLINACK"]
pub type OnemcuTpiuIttrflinackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Integration Register, ITTRFLINACK"]
    #[inline(always)]
    pub fn onemcu_tpiu_ittrflinack(&self) -> OnemcuTpiuIttrflinackR {
        OnemcuTpiuIttrflinackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Integration Register, ITTRFLINACK"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_ittrflinack(&mut self) -> OnemcuTpiuIttrflinackW<OnemcuTpiuIttrflinackSpec> {
        OnemcuTpiuIttrflinackW::new(self, 0)
    }
}
#[doc = "Integration Register, ITTRFLINACK\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_ittrflinack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_ittrflinack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuIttrflinackSpec;
impl crate::RegisterSpec for OnemcuTpiuIttrflinackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_ittrflinack::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuIttrflinackSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_ittrflinack::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuIttrflinackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_ITTRFLINACK to value 0"]
impl crate::Resettable for OnemcuTpiuIttrflinackSpec {
    const RESET_VALUE: u32 = 0;
}
