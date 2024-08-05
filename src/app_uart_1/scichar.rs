#[doc = "Register `SCICHAR` reader"]
pub type R = crate::R<ScicharSpec>;
#[doc = "Register `SCICHAR` writer"]
pub type W = crate::W<ScicharSpec>;
#[doc = "Field `CHAR` reader - 2:0\\]
Sets the SCI data length from 1 to 8 bits"]
pub type CharR = crate::FieldReader;
#[doc = "Field `CHAR` writer - 2:0\\]
Sets the SCI data length from 1 to 8 bits"]
pub type CharW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Sets the SCI data length from 1 to 8 bits"]
    #[inline(always)]
    pub fn char(&self) -> CharR {
        CharR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Sets the SCI data length from 1 to 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn char(&mut self) -> CharW<ScicharSpec> {
        CharW::new(self, 0)
    }
}
#[doc = "SCI Character Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scichar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scichar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScicharSpec;
impl crate::RegisterSpec for ScicharSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scichar::R`](R) reader structure"]
impl crate::Readable for ScicharSpec {}
#[doc = "`write(|w| ..)` method takes [`scichar::W`](W) writer structure"]
impl crate::Writable for ScicharSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCICHAR to value 0"]
impl crate::Resettable for ScicharSpec {
    const RESET_VALUE: u32 = 0;
}
