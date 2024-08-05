#[doc = "Register `ONEMCU_TPIU_EXCTLIN` reader"]
pub type R = crate::R<OnemcuTpiuExctlinSpec>;
#[doc = "Register `ONEMCU_TPIU_EXCTLIN` writer"]
pub type W = crate::W<OnemcuTpiuExctlinSpec>;
#[doc = "Field `ONEMCU_TPIU_EXCTLIN` reader - 31:0\\]
EXTCTL In Port"]
pub type OnemcuTpiuExctlinR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_EXCTLIN` writer - 31:0\\]
EXTCTL In Port"]
pub type OnemcuTpiuExctlinW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
EXTCTL In Port"]
    #[inline(always)]
    pub fn onemcu_tpiu_exctlin(&self) -> OnemcuTpiuExctlinR {
        OnemcuTpiuExctlinR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
EXTCTL In Port"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_exctlin(&mut self) -> OnemcuTpiuExctlinW<OnemcuTpiuExctlinSpec> {
        OnemcuTpiuExctlinW::new(self, 0)
    }
}
#[doc = "EXTCTL In Port\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_exctlin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_exctlin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuExctlinSpec;
impl crate::RegisterSpec for OnemcuTpiuExctlinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_exctlin::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuExctlinSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_exctlin::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuExctlinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_EXCTLIN to value 0"]
impl crate::Resettable for OnemcuTpiuExctlinSpec {
    const RESET_VALUE: u32 = 0;
}
