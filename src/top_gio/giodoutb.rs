#[doc = "Register `GIODOUTB` reader"]
pub type R = crate::R<GiodoutbSpec>;
#[doc = "Register `GIODOUTB` writer"]
pub type W = crate::W<GiodoutbSpec>;
#[doc = "Field `GIODOUTB` reader - 7:0\\]
GIO data output for pins in port B"]
pub type GiodoutbR = crate::FieldReader;
#[doc = "Field `GIODOUTB` writer - 7:0\\]
GIO data output for pins in port B"]
pub type GiodoutbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU18` reader - 31:8\\]
Reserved"]
pub type Nu18R = crate::FieldReader<u32>;
#[doc = "Field `NU18` writer - 31:8\\]
Reserved"]
pub type Nu18W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data output for pins in port B"]
    #[inline(always)]
    pub fn giodoutb(&self) -> GiodoutbR {
        GiodoutbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu18(&self) -> Nu18R {
        Nu18R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data output for pins in port B"]
    #[inline(always)]
    #[must_use]
    pub fn giodoutb(&mut self) -> GiodoutbW<GiodoutbSpec> {
        GiodoutbW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu18(&mut self) -> Nu18W<GiodoutbSpec> {
        Nu18W::new(self, 8)
    }
}
#[doc = "GIO data output for pins in port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giodoutb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodoutb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodoutbSpec;
impl crate::RegisterSpec for GiodoutbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodoutb::R`](R) reader structure"]
impl crate::Readable for GiodoutbSpec {}
#[doc = "`write(|w| ..)` method takes [`giodoutb::W`](W) writer structure"]
impl crate::Writable for GiodoutbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODOUTB to value 0"]
impl crate::Resettable for GiodoutbSpec {
    const RESET_VALUE: u32 = 0;
}
