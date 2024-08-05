#[doc = "Register `GIOPDRG` reader"]
pub type R = crate::R<GiopdrgSpec>;
#[doc = "Register `GIOPDRG` writer"]
pub type W = crate::W<GiopdrgSpec>;
#[doc = "Field `GIOPDRG` reader - 7:0\\]
GIO open drain for port G"]
pub type GiopdrgR = crate::FieldReader;
#[doc = "Field `GIOPDRG` writer - 7:0\\]
GIO open drain for port G"]
pub type GiopdrgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU39` reader - 31:8\\]
Reserved"]
pub type Nu39R = crate::FieldReader<u32>;
#[doc = "Field `NU39` writer - 31:8\\]
Reserved"]
pub type Nu39W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO open drain for port G"]
    #[inline(always)]
    pub fn giopdrg(&self) -> GiopdrgR {
        GiopdrgR::new((self.bits & 0xff) as u8)
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
GIO open drain for port G"]
    #[inline(always)]
    #[must_use]
    pub fn giopdrg(&mut self) -> GiopdrgW<GiopdrgSpec> {
        GiopdrgW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu39(&mut self) -> Nu39W<GiopdrgSpec> {
        Nu39W::new(self, 8)
    }
}
#[doc = "GIO open drain for port G\n\nYou can [`read`](crate::Reg::read) this register and get [`giopdrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopdrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopdrgSpec;
impl crate::RegisterSpec for GiopdrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopdrg::R`](R) reader structure"]
impl crate::Readable for GiopdrgSpec {}
#[doc = "`write(|w| ..)` method takes [`giopdrg::W`](W) writer structure"]
impl crate::Writable for GiopdrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPDRG to value 0"]
impl crate::Resettable for GiopdrgSpec {
    const RESET_VALUE: u32 = 0;
}
