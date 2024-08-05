#[doc = "Register `GIOPDRE` reader"]
pub type R = crate::R<GiopdreSpec>;
#[doc = "Register `GIOPDRE` writer"]
pub type W = crate::W<GiopdreSpec>;
#[doc = "Field `GIOPDRE` reader - 7:0\\]
GIO open drain for port E"]
pub type GiopdreR = crate::FieldReader;
#[doc = "Field `GIOPDRE` writer - 7:0\\]
GIO open drain for port E"]
pub type GiopdreW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU39` reader - 31:8\\]
Reserved"]
pub type Nu39R = crate::FieldReader<u32>;
#[doc = "Field `NU39` writer - 31:8\\]
Reserved"]
pub type Nu39W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO open drain for port E"]
    #[inline(always)]
    pub fn giopdre(&self) -> GiopdreR {
        GiopdreR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu39(&self) -> Nu39R {
        Nu39R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO open drain for port E"]
    #[inline(always)]
    #[must_use]
    pub fn giopdre(&mut self) -> GiopdreW<GiopdreSpec> {
        GiopdreW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu39(&mut self) -> Nu39W<GiopdreSpec> {
        Nu39W::new(self, 8)
    }
}
#[doc = "GIO open drain for port E\n\nYou can [`read`](crate::Reg::read) this register and get [`giopdre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopdre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopdreSpec;
impl crate::RegisterSpec for GiopdreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopdre::R`](R) reader structure"]
impl crate::Readable for GiopdreSpec {}
#[doc = "`write(|w| ..)` method takes [`giopdre::W`](W) writer structure"]
impl crate::Writable for GiopdreSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPDRE to value 0"]
impl crate::Resettable for GiopdreSpec {
    const RESET_VALUE: u32 = 0;
}
