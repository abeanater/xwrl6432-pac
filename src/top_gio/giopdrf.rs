#[doc = "Register `GIOPDRF` reader"]
pub type R = crate::R<GiopdrfSpec>;
#[doc = "Register `GIOPDRF` writer"]
pub type W = crate::W<GiopdrfSpec>;
#[doc = "Field `GIOPDRF` reader - 7:0\\]
GIO open drain for port F"]
pub type GiopdrfR = crate::FieldReader;
#[doc = "Field `GIOPDRF` writer - 7:0\\]
GIO open drain for port F"]
pub type GiopdrfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU40` reader - 31:8\\]
Reserved"]
pub type Nu40R = crate::FieldReader<u32>;
#[doc = "Field `NU40` writer - 31:8\\]
Reserved"]
pub type Nu40W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO open drain for port F"]
    #[inline(always)]
    pub fn giopdrf(&self) -> GiopdrfR {
        GiopdrfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu40(&self) -> Nu40R {
        Nu40R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO open drain for port F"]
    #[inline(always)]
    #[must_use]
    pub fn giopdrf(&mut self) -> GiopdrfW<GiopdrfSpec> {
        GiopdrfW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu40(&mut self) -> Nu40W<GiopdrfSpec> {
        Nu40W::new(self, 8)
    }
}
#[doc = "GIO open drain for Port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giopdrf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopdrf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopdrfSpec;
impl crate::RegisterSpec for GiopdrfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopdrf::R`](R) reader structure"]
impl crate::Readable for GiopdrfSpec {}
#[doc = "`write(|w| ..)` method takes [`giopdrf::W`](W) writer structure"]
impl crate::Writable for GiopdrfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPDRF to value 0"]
impl crate::Resettable for GiopdrfSpec {
    const RESET_VALUE: u32 = 0;
}
