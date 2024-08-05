#[doc = "Register `GIOSETB` reader"]
pub type R = crate::R<GiosetbSpec>;
#[doc = "Register `GIOSETB` writer"]
pub type W = crate::W<GiosetbSpec>;
#[doc = "Field `GIODSETB` reader - 7:0\\]
GIO data set for port B"]
pub type GiodsetbR = crate::FieldReader;
#[doc = "Field `GIODSETB` writer - 7:0\\]
GIO data set for port B"]
pub type GiodsetbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU24` reader - 31:8\\]
Reserved"]
pub type Nu24R = crate::FieldReader<u32>;
#[doc = "Field `NU24` writer - 31:8\\]
Reserved"]
pub type Nu24W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data set for port B"]
    #[inline(always)]
    pub fn giodsetb(&self) -> GiodsetbR {
        GiodsetbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu24(&self) -> Nu24R {
        Nu24R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data set for port B"]
    #[inline(always)]
    #[must_use]
    pub fn giodsetb(&mut self) -> GiodsetbW<GiosetbSpec> {
        GiodsetbW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu24(&mut self) -> Nu24W<GiosetbSpec> {
        Nu24W::new(self, 8)
    }
}
#[doc = "GIO data set for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giosetb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosetb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiosetbSpec;
impl crate::RegisterSpec for GiosetbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giosetb::R`](R) reader structure"]
impl crate::Readable for GiosetbSpec {}
#[doc = "`write(|w| ..)` method takes [`giosetb::W`](W) writer structure"]
impl crate::Writable for GiosetbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOSETB to value 0"]
impl crate::Resettable for GiosetbSpec {
    const RESET_VALUE: u32 = 0;
}
