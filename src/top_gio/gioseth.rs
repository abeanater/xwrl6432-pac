#[doc = "Register `GIOSETH` reader"]
pub type R = crate::R<GiosethSpec>;
#[doc = "Register `GIOSETH` writer"]
pub type W = crate::W<GiosethSpec>;
#[doc = "Field `GIODSETH` reader - 7:0\\]
GIO data set for port H"]
pub type GiodsethR = crate::FieldReader;
#[doc = "Field `GIODSETH` writer - 7:0\\]
GIO data set for port H"]
pub type GiodsethW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU28` reader - 31:8\\]
Reserved"]
pub type Nu28R = crate::FieldReader<u32>;
#[doc = "Field `NU28` writer - 31:8\\]
Reserved"]
pub type Nu28W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data set for port H"]
    #[inline(always)]
    pub fn giodseth(&self) -> GiodsethR {
        GiodsethR::new((self.bits & 0xff) as u8)
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
GIO data set for port H"]
    #[inline(always)]
    #[must_use]
    pub fn giodseth(&mut self) -> GiodsethW<GiosethSpec> {
        GiodsethW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu28(&mut self) -> Nu28W<GiosethSpec> {
        Nu28W::new(self, 8)
    }
}
#[doc = "GIO data set for Port H\n\nYou can [`read`](crate::Reg::read) this register and get [`gioseth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioseth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiosethSpec;
impl crate::RegisterSpec for GiosethSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gioseth::R`](R) reader structure"]
impl crate::Readable for GiosethSpec {}
#[doc = "`write(|w| ..)` method takes [`gioseth::W`](W) writer structure"]
impl crate::Writable for GiosethSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOSETH to value 0"]
impl crate::Resettable for GiosethSpec {
    const RESET_VALUE: u32 = 0;
}
