#[doc = "Register `GIODIRG` reader"]
pub type R = crate::R<GiodirgSpec>;
#[doc = "Register `GIODIRG` writer"]
pub type W = crate::W<GiodirgSpec>;
#[doc = "Field `GIODIRG` reader - 7:0\\]
GIO data direction of pins in Port G"]
pub type GiodirgR = crate::FieldReader;
#[doc = "Field `GIODIRG` writer - 7:0\\]
GIO data direction of pins in Port G"]
pub type GiodirgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU9` reader - 31:8\\]
Reserved"]
pub type Nu9R = crate::FieldReader<u32>;
#[doc = "Field `NU9` writer - 31:8\\]
Reserved"]
pub type Nu9W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data direction of pins in Port G"]
    #[inline(always)]
    pub fn giodirg(&self) -> GiodirgR {
        GiodirgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu9(&self) -> Nu9R {
        Nu9R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data direction of pins in Port G"]
    #[inline(always)]
    #[must_use]
    pub fn giodirg(&mut self) -> GiodirgW<GiodirgSpec> {
        GiodirgW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu9(&mut self) -> Nu9W<GiodirgSpec> {
        Nu9W::new(self, 8)
    }
}
#[doc = "GIO data direction of pins in Port G\n\nYou can [`read`](crate::Reg::read) this register and get [`giodirg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodirg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodirgSpec;
impl crate::RegisterSpec for GiodirgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodirg::R`](R) reader structure"]
impl crate::Readable for GiodirgSpec {}
#[doc = "`write(|w| ..)` method takes [`giodirg::W`](W) writer structure"]
impl crate::Writable for GiodirgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODIRG to value 0"]
impl crate::Resettable for GiodirgSpec {
    const RESET_VALUE: u32 = 0;
}
