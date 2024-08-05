#[doc = "Register `GIOSETA` reader"]
pub type R = crate::R<GiosetaSpec>;
#[doc = "Register `GIOSETA` writer"]
pub type W = crate::W<GiosetaSpec>;
#[doc = "Field `GIODSETA` reader - 7:0\\]
GIO data set for port A"]
pub type GiodsetaR = crate::FieldReader;
#[doc = "Field `GIODSETA` writer - 7:0\\]
GIO data set for port A"]
pub type GiodsetaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU23` reader - 31:8\\]
Reserved"]
pub type Nu23R = crate::FieldReader<u32>;
#[doc = "Field `NU23` writer - 31:8\\]
Reserved"]
pub type Nu23W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data set for port A"]
    #[inline(always)]
    pub fn giodseta(&self) -> GiodsetaR {
        GiodsetaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu23(&self) -> Nu23R {
        Nu23R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data set for port A"]
    #[inline(always)]
    #[must_use]
    pub fn giodseta(&mut self) -> GiodsetaW<GiosetaSpec> {
        GiodsetaW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu23(&mut self) -> Nu23W<GiosetaSpec> {
        Nu23W::new(self, 8)
    }
}
#[doc = "GIO data set for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`gioseta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioseta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiosetaSpec;
impl crate::RegisterSpec for GiosetaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gioseta::R`](R) reader structure"]
impl crate::Readable for GiosetaSpec {}
#[doc = "`write(|w| ..)` method takes [`gioseta::W`](W) writer structure"]
impl crate::Writable for GiosetaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOSETA to value 0"]
impl crate::Resettable for GiosetaSpec {
    const RESET_VALUE: u32 = 0;
}
