#[doc = "Register `GIOPDRA` reader"]
pub type R = crate::R<GiopdraSpec>;
#[doc = "Register `GIOPDRA` writer"]
pub type W = crate::W<GiopdraSpec>;
#[doc = "Field `GIOPDRA` reader - 7:0\\]
GIO open drain for port A"]
pub type GiopdraR = crate::FieldReader;
#[doc = "Field `GIOPDRA` writer - 7:0\\]
GIO open drain for port A"]
pub type GiopdraW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU35` reader - 31:8\\]
Reserved"]
pub type Nu35R = crate::FieldReader<u32>;
#[doc = "Field `NU35` writer - 31:8\\]
Reserved"]
pub type Nu35W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO open drain for port A"]
    #[inline(always)]
    pub fn giopdra(&self) -> GiopdraR {
        GiopdraR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu35(&self) -> Nu35R {
        Nu35R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO open drain for port A"]
    #[inline(always)]
    #[must_use]
    pub fn giopdra(&mut self) -> GiopdraW<GiopdraSpec> {
        GiopdraW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu35(&mut self) -> Nu35W<GiopdraSpec> {
        Nu35W::new(self, 8)
    }
}
#[doc = "GIO open drain for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`giopdra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopdra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopdraSpec;
impl crate::RegisterSpec for GiopdraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopdra::R`](R) reader structure"]
impl crate::Readable for GiopdraSpec {}
#[doc = "`write(|w| ..)` method takes [`giopdra::W`](W) writer structure"]
impl crate::Writable for GiopdraSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPDRA to value 0"]
impl crate::Resettable for GiopdraSpec {
    const RESET_VALUE: u32 = 0;
}
