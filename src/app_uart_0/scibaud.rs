#[doc = "Register `SCIBAUD` reader"]
pub type R = crate::R<ScibaudSpec>;
#[doc = "Register `SCIBAUD` writer"]
pub type W = crate::W<ScibaudSpec>;
#[doc = "Field `BAUD` reader - 23:0\\]
SCI 24-bit baud selection"]
pub type BaudR = crate::FieldReader<u32>;
#[doc = "Field `BAUD` writer - 23:0\\]
SCI 24-bit baud selection"]
pub type BaudW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
SCI 24-bit baud selection"]
    #[inline(always)]
    pub fn baud(&self) -> BaudR {
        BaudR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
SCI 24-bit baud selection"]
    #[inline(always)]
    #[must_use]
    pub fn baud(&mut self) -> BaudW<ScibaudSpec> {
        BaudW::new(self, 0)
    }
}
#[doc = "SCI Baud Rate Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scibaud::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scibaud::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScibaudSpec;
impl crate::RegisterSpec for ScibaudSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scibaud::R`](R) reader structure"]
impl crate::Readable for ScibaudSpec {}
#[doc = "`write(|w| ..)` method takes [`scibaud::W`](W) writer structure"]
impl crate::Writable for ScibaudSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIBAUD to value 0"]
impl crate::Resettable for ScibaudSpec {
    const RESET_VALUE: u32 = 0;
}
