#[doc = "Register `GIOSETF` reader"]
pub type R = crate::R<GiosetfSpec>;
#[doc = "Register `GIOSETF` writer"]
pub type W = crate::W<GiosetfSpec>;
#[doc = "Field `GIODSETF` reader - 7:0\\]
GIO data set for port F"]
pub type GiodsetfR = crate::FieldReader;
#[doc = "Field `GIODSETF` writer - 7:0\\]
GIO data set for port F"]
pub type GiodsetfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU28` reader - 31:8\\]
Reserved"]
pub type Nu28R = crate::FieldReader<u32>;
#[doc = "Field `NU28` writer - 31:8\\]
Reserved"]
pub type Nu28W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data set for port F"]
    #[inline(always)]
    pub fn giodsetf(&self) -> GiodsetfR {
        GiodsetfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu28(&self) -> Nu28R {
        Nu28R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data set for port F"]
    #[inline(always)]
    #[must_use]
    pub fn giodsetf(&mut self) -> GiodsetfW<GiosetfSpec> {
        GiodsetfW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu28(&mut self) -> Nu28W<GiosetfSpec> {
        Nu28W::new(self, 8)
    }
}
#[doc = "GIO data set for Port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giosetf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosetf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiosetfSpec;
impl crate::RegisterSpec for GiosetfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giosetf::R`](R) reader structure"]
impl crate::Readable for GiosetfSpec {}
#[doc = "`write(|w| ..)` method takes [`giosetf::W`](W) writer structure"]
impl crate::Writable for GiosetfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOSETF to value 0"]
impl crate::Resettable for GiosetfSpec {
    const RESET_VALUE: u32 = 0;
}
