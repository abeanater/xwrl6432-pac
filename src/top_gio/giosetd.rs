#[doc = "Register `GIOSETD` reader"]
pub type R = crate::R<GiosetdSpec>;
#[doc = "Register `GIOSETD` writer"]
pub type W = crate::W<GiosetdSpec>;
#[doc = "Field `GIODSETD` reader - 7:0\\]
GIO data set for port D"]
pub type GiodsetdR = crate::FieldReader;
#[doc = "Field `GIODSETD` writer - 7:0\\]
GIO data set for port D"]
pub type GiodsetdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU26` reader - 31:8\\]
Reserved"]
pub type Nu26R = crate::FieldReader<u32>;
#[doc = "Field `NU26` writer - 31:8\\]
Reserved"]
pub type Nu26W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data set for port D"]
    #[inline(always)]
    pub fn giodsetd(&self) -> GiodsetdR {
        GiodsetdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu26(&self) -> Nu26R {
        Nu26R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data set for port D"]
    #[inline(always)]
    #[must_use]
    pub fn giodsetd(&mut self) -> GiodsetdW<GiosetdSpec> {
        GiodsetdW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu26(&mut self) -> Nu26W<GiosetdSpec> {
        Nu26W::new(self, 8)
    }
}
#[doc = "GIO data set for port D\n\nYou can [`read`](crate::Reg::read) this register and get [`giosetd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosetd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiosetdSpec;
impl crate::RegisterSpec for GiosetdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giosetd::R`](R) reader structure"]
impl crate::Readable for GiosetdSpec {}
#[doc = "`write(|w| ..)` method takes [`giosetd::W`](W) writer structure"]
impl crate::Writable for GiosetdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOSETD to value 0"]
impl crate::Resettable for GiosetdSpec {
    const RESET_VALUE: u32 = 0;
}
