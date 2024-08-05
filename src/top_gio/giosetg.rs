#[doc = "Register `GIOSETG` reader"]
pub type R = crate::R<GiosetgSpec>;
#[doc = "Register `GIOSETG` writer"]
pub type W = crate::W<GiosetgSpec>;
#[doc = "Field `GIODSETG` reader - 7:0\\]
GIO data set for port G"]
pub type GiodsetgR = crate::FieldReader;
#[doc = "Field `GIODSETG` writer - 7:0\\]
GIO data set for port G"]
pub type GiodsetgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU27` reader - 31:8\\]
Reserved"]
pub type Nu27R = crate::FieldReader<u32>;
#[doc = "Field `NU27` writer - 31:8\\]
Reserved"]
pub type Nu27W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data set for port G"]
    #[inline(always)]
    pub fn giodsetg(&self) -> GiodsetgR {
        GiodsetgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu27(&self) -> Nu27R {
        Nu27R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data set for port G"]
    #[inline(always)]
    #[must_use]
    pub fn giodsetg(&mut self) -> GiodsetgW<GiosetgSpec> {
        GiodsetgW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu27(&mut self) -> Nu27W<GiosetgSpec> {
        Nu27W::new(self, 8)
    }
}
#[doc = "GIO data set for port G\n\nYou can [`read`](crate::Reg::read) this register and get [`giosetg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosetg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiosetgSpec;
impl crate::RegisterSpec for GiosetgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giosetg::R`](R) reader structure"]
impl crate::Readable for GiosetgSpec {}
#[doc = "`write(|w| ..)` method takes [`giosetg::W`](W) writer structure"]
impl crate::Writable for GiosetgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOSETG to value 0"]
impl crate::Resettable for GiosetgSpec {
    const RESET_VALUE: u32 = 0;
}
