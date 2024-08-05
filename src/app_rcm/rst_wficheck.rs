#[doc = "Register `RST_WFICHECK` reader"]
pub type R = crate::R<RstWficheckSpec>;
#[doc = "Register `RST_WFICHECK` writer"]
pub type W = crate::W<RstWficheckSpec>;
#[doc = "Field `cpu` reader - 2:0\\]
Writing '000' will disable check for WFI before local reset assertion of app cpu"]
pub type CpuR = crate::FieldReader;
#[doc = "Field `cpu` writer - 2:0\\]
Writing '000' will disable check for WFI before local reset assertion of app cpu"]
pub type CpuW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Writing '000' will disable check for WFI before local reset assertion of app cpu"]
    #[inline(always)]
    pub fn cpu(&self) -> CpuR {
        CpuR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Writing '000' will disable check for WFI before local reset assertion of app cpu"]
    #[inline(always)]
    #[must_use]
    pub fn cpu(&mut self) -> CpuW<RstWficheckSpec> {
        CpuW::new(self, 0)
    }
}
#[doc = "RST_WFICHECK\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_wficheck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_wficheck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstWficheckSpec;
impl crate::RegisterSpec for RstWficheckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_wficheck::R`](R) reader structure"]
impl crate::Readable for RstWficheckSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_wficheck::W`](W) writer structure"]
impl crate::Writable for RstWficheckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST_WFICHECK to value 0"]
impl crate::Resettable for RstWficheckSpec {
    const RESET_VALUE: u32 = 0;
}
