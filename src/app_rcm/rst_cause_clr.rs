#[doc = "Register `RST_CAUSE_CLR` reader"]
pub type R = crate::R<RstCauseClrSpec>;
#[doc = "Register `RST_CAUSE_CLR` writer"]
pub type W = crate::W<RstCauseClrSpec>;
#[doc = "Field `cpu` reader - 2:0\\]
Writing '111' will clear the RST_CAUSE register"]
pub type CpuR = crate::FieldReader;
#[doc = "Field `cpu` writer - 2:0\\]
Writing '111' will clear the RST_CAUSE register"]
pub type CpuW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Writing '111' will clear the RST_CAUSE register"]
    #[inline(always)]
    pub fn cpu(&self) -> CpuR {
        CpuR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Writing '111' will clear the RST_CAUSE register"]
    #[inline(always)]
    #[must_use]
    pub fn cpu(&mut self) -> CpuW<RstCauseClrSpec> {
        CpuW::new(self, 0)
    }
}
#[doc = "RST_CAUSE_CLR\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_cause_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_cause_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstCauseClrSpec;
impl crate::RegisterSpec for RstCauseClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_cause_clr::R`](R) reader structure"]
impl crate::Readable for RstCauseClrSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_cause_clr::W`](W) writer structure"]
impl crate::Writable for RstCauseClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST_CAUSE_CLR to value 0"]
impl crate::Resettable for RstCauseClrSpec {
    const RESET_VALUE: u32 = 0;
}
