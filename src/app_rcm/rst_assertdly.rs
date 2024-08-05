#[doc = "Register `RST_ASSERTDLY` reader"]
pub type R = crate::R<RstAssertdlySpec>;
#[doc = "Register `RST_ASSERTDLY` writer"]
pub type W = crate::W<RstAssertdlySpec>;
#[doc = "Field `common` reader - 7:0\\]
Value decides number of cycles reset should be asserted for cpu"]
pub type CommonR = crate::FieldReader;
#[doc = "Field `common` writer - 7:0\\]
Value decides number of cycles reset should be asserted for cpu"]
pub type CommonW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Value decides number of cycles reset should be asserted for cpu"]
    #[inline(always)]
    pub fn common(&self) -> CommonR {
        CommonR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Value decides number of cycles reset should be asserted for cpu"]
    #[inline(always)]
    #[must_use]
    pub fn common(&mut self) -> CommonW<RstAssertdlySpec> {
        CommonW::new(self, 0)
    }
}
#[doc = "RST_ASSERTDLY\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_assertdly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_assertdly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstAssertdlySpec;
impl crate::RegisterSpec for RstAssertdlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_assertdly::R`](R) reader structure"]
impl crate::Readable for RstAssertdlySpec {}
#[doc = "`write(|w| ..)` method takes [`rst_assertdly::W`](W) writer structure"]
impl crate::Writable for RstAssertdlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST_ASSERTDLY to value 0"]
impl crate::Resettable for RstAssertdlySpec {
    const RESET_VALUE: u32 = 0;
}
