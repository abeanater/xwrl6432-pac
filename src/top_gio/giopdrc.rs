#[doc = "Register `GIOPDRC` reader"]
pub type R = crate::R<GiopdrcSpec>;
#[doc = "Register `GIOPDRC` writer"]
pub type W = crate::W<GiopdrcSpec>;
#[doc = "Field `GIOPDRC` reader - 7:0\\]
GIO open drain for port C"]
pub type GiopdrcR = crate::FieldReader;
#[doc = "Field `GIOPDRC` writer - 7:0\\]
GIO open drain for port C"]
pub type GiopdrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU37` reader - 31:8\\]
Reserved"]
pub type Nu37R = crate::FieldReader<u32>;
#[doc = "Field `NU37` writer - 31:8\\]
Reserved"]
pub type Nu37W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO open drain for port C"]
    #[inline(always)]
    pub fn giopdrc(&self) -> GiopdrcR {
        GiopdrcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu37(&self) -> Nu37R {
        Nu37R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO open drain for port C"]
    #[inline(always)]
    #[must_use]
    pub fn giopdrc(&mut self) -> GiopdrcW<GiopdrcSpec> {
        GiopdrcW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu37(&mut self) -> Nu37W<GiopdrcSpec> {
        Nu37W::new(self, 8)
    }
}
#[doc = "GIO open drain for port C\n\nYou can [`read`](crate::Reg::read) this register and get [`giopdrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopdrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopdrcSpec;
impl crate::RegisterSpec for GiopdrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopdrc::R`](R) reader structure"]
impl crate::Readable for GiopdrcSpec {}
#[doc = "`write(|w| ..)` method takes [`giopdrc::W`](W) writer structure"]
impl crate::Writable for GiopdrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPDRC to value 0"]
impl crate::Resettable for GiopdrcSpec {
    const RESET_VALUE: u32 = 0;
}
