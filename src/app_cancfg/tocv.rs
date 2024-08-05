#[doc = "Register `TOCV` reader"]
pub type R = crate::R<TocvSpec>;
#[doc = "Register `TOCV` writer"]
pub type W = crate::W<TocvSpec>;
#[doc = "Field `TOC` reader - 15:0\\]
Timeout Counter"]
pub type TocR = crate::FieldReader<u16>;
#[doc = "Field `TOC` writer - 15:0\\]
Timeout Counter"]
pub type TocW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NU24` reader - 31:16\\]
Reserved"]
pub type Nu24R = crate::FieldReader<u16>;
#[doc = "Field `NU24` writer - 31:16\\]
Reserved"]
pub type Nu24W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Timeout Counter"]
    #[inline(always)]
    pub fn toc(&self) -> TocR {
        TocR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn nu24(&self) -> Nu24R {
        Nu24R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TocW<TocvSpec> {
        TocW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu24(&mut self) -> Nu24W<TocvSpec> {
        Nu24W::new(self, 16)
    }
}
#[doc = "TOCV\n\nYou can [`read`](crate::Reg::read) this register and get [`tocv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TocvSpec;
impl crate::RegisterSpec for TocvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocv::R`](R) reader structure"]
impl crate::Readable for TocvSpec {}
#[doc = "`write(|w| ..)` method takes [`tocv::W`](W) writer structure"]
impl crate::Writable for TocvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOCV to value 0"]
impl crate::Resettable for TocvSpec {
    const RESET_VALUE: u32 = 0;
}
