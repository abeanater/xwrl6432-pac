#[doc = "Register `RST2ASSERTDLY` reader"]
pub type R = crate::R<Rst2assertdlySpec>;
#[doc = "Register `RST2ASSERTDLY` writer"]
pub type W = crate::W<Rst2assertdlySpec>;
#[doc = "Field `cpu` reader - 15:8\\]
Value decides number of cycles to be held before asserting reset for app cpu"]
pub type CpuR = crate::FieldReader;
#[doc = "Field `cpu` writer - 15:8\\]
Value decides number of cycles to be held before asserting reset for app cpu"]
pub type CpuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - 15:8\\]
Value decides number of cycles to be held before asserting reset for app cpu"]
    #[inline(always)]
    pub fn cpu(&self) -> CpuR {
        CpuR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15:8\\]
Value decides number of cycles to be held before asserting reset for app cpu"]
    #[inline(always)]
    #[must_use]
    pub fn cpu(&mut self) -> CpuW<Rst2assertdlySpec> {
        CpuW::new(self, 8)
    }
}
#[doc = "RST2ASSERTDLY\n\nYou can [`read`](crate::Reg::read) this register and get [`rst2assertdly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst2assertdly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rst2assertdlySpec;
impl crate::RegisterSpec for Rst2assertdlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst2assertdly::R`](R) reader structure"]
impl crate::Readable for Rst2assertdlySpec {}
#[doc = "`write(|w| ..)` method takes [`rst2assertdly::W`](W) writer structure"]
impl crate::Writable for Rst2assertdlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST2ASSERTDLY to value 0"]
impl crate::Resettable for Rst2assertdlySpec {
    const RESET_VALUE: u32 = 0;
}
