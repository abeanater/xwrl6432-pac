#[doc = "Register `GIOPOL` reader"]
pub type R = crate::R<GiopolSpec>;
#[doc = "Register `GIOPOL` writer"]
pub type W = crate::W<GiopolSpec>;
#[doc = "Field `GIOPOL_0` reader - 7:0\\]
Interrupt polarity select for pins GIOA\\[7:0\\]"]
pub type Giopol0R = crate::FieldReader;
#[doc = "Field `GIOPOL_0` writer - 7:0\\]
Interrupt polarity select for pins GIOA\\[7:0\\]"]
pub type Giopol0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOPOL_1` reader - 15:8\\]
Interrupt polarity select for pins GIOB\\[7:0\\]"]
pub type Giopol1R = crate::FieldReader;
#[doc = "Field `GIOPOL_1` writer - 15:8\\]
Interrupt polarity select for pins GIOB\\[7:0\\]"]
pub type Giopol1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOPOL_2` reader - 23:16\\]
Interrupt polarity select for pins GIOC\\[7:0\\]"]
pub type Giopol2R = crate::FieldReader;
#[doc = "Field `GIOPOL_2` writer - 23:16\\]
Interrupt polarity select for pins GIOC\\[7:0\\]"]
pub type Giopol2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOPOL_3` reader - 31:24\\]
Interrupt polarity select for pins GIOD\\[7:0\\]"]
pub type Giopol3R = crate::FieldReader;
#[doc = "Field `GIOPOL_3` writer - 31:24\\]
Interrupt polarity select for pins GIOD\\[7:0\\]"]
pub type Giopol3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt polarity select for pins GIOA\\[7:0\\]"]
    #[inline(always)]
    pub fn giopol_0(&self) -> Giopol0R {
        Giopol0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Interrupt polarity select for pins GIOB\\[7:0\\]"]
    #[inline(always)]
    pub fn giopol_1(&self) -> Giopol1R {
        Giopol1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Interrupt polarity select for pins GIOC\\[7:0\\]"]
    #[inline(always)]
    pub fn giopol_2(&self) -> Giopol2R {
        Giopol2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Interrupt polarity select for pins GIOD\\[7:0\\]"]
    #[inline(always)]
    pub fn giopol_3(&self) -> Giopol3R {
        Giopol3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt polarity select for pins GIOA\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn giopol_0(&mut self) -> Giopol0W<GiopolSpec> {
        Giopol0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Interrupt polarity select for pins GIOB\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn giopol_1(&mut self) -> Giopol1W<GiopolSpec> {
        Giopol1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Interrupt polarity select for pins GIOC\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn giopol_2(&mut self) -> Giopol2W<GiopolSpec> {
        Giopol2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Interrupt polarity select for pins GIOD\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn giopol_3(&mut self) -> Giopol3W<GiopolSpec> {
        Giopol3W::new(self, 24)
    }
}
#[doc = "Interrupt polarity select for pins \\[0:1\\]
GIO\\[7:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`giopol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopolSpec;
impl crate::RegisterSpec for GiopolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopol::R`](R) reader structure"]
impl crate::Readable for GiopolSpec {}
#[doc = "`write(|w| ..)` method takes [`giopol::W`](W) writer structure"]
impl crate::Writable for GiopolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPOL to value 0"]
impl crate::Resettable for GiopolSpec {
    const RESET_VALUE: u32 = 0;
}
