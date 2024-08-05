#[doc = "Register `GIODINB` reader"]
pub type R = crate::R<GiodinbSpec>;
#[doc = "Register `GIODINB` writer"]
pub type W = crate::W<GiodinbSpec>;
#[doc = "Field `GIODINB` reader - 7:0\\]
GIO data input for pins in port B"]
pub type GiodinbR = crate::FieldReader;
#[doc = "Field `GIODINB` writer - 7:0\\]
GIO data input for pins in port B"]
pub type GiodinbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU12` reader - 31:8\\]
Reserved"]
pub type Nu12R = crate::FieldReader<u32>;
#[doc = "Field `NU12` writer - 31:8\\]
Reserved"]
pub type Nu12W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data input for pins in port B"]
    #[inline(always)]
    pub fn giodinb(&self) -> GiodinbR {
        GiodinbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu12(&self) -> Nu12R {
        Nu12R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data input for pins in port B"]
    #[inline(always)]
    #[must_use]
    pub fn giodinb(&mut self) -> GiodinbW<GiodinbSpec> {
        GiodinbW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu12(&mut self) -> Nu12W<GiodinbSpec> {
        Nu12W::new(self, 8)
    }
}
#[doc = "GIO data input for pins in port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giodinb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodinb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodinbSpec;
impl crate::RegisterSpec for GiodinbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodinb::R`](R) reader structure"]
impl crate::Readable for GiodinbSpec {}
#[doc = "`write(|w| ..)` method takes [`giodinb::W`](W) writer structure"]
impl crate::Writable for GiodinbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODINB to value 0"]
impl crate::Resettable for GiodinbSpec {
    const RESET_VALUE: u32 = 0;
}
