#[doc = "Register `GIOFLG` reader"]
pub type R = crate::R<GioflgSpec>;
#[doc = "Register `GIOFLG` writer"]
pub type W = crate::W<GioflgSpec>;
#[doc = "Field `GIOFLG_0` reader - 7:0\\]
GIO flag for pins GIOA\\[7:0\\]"]
pub type Gioflg0R = crate::FieldReader;
#[doc = "Field `GIOFLG_0` writer - 7:0\\]
GIO flag for pins GIOA\\[7:0\\]"]
pub type Gioflg0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOFLG_1` reader - 15:8\\]
GIO flag for pins GIOB\\[7:0\\]"]
pub type Gioflg1R = crate::FieldReader;
#[doc = "Field `GIOFLG_1` writer - 15:8\\]
GIO flag for pins GIOB\\[7:0\\]"]
pub type Gioflg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOFLG_2` reader - 23:16\\]
GIO flag for pins GIOC\\[7:0\\]"]
pub type Gioflg2R = crate::FieldReader;
#[doc = "Field `GIOFLG_2` writer - 23:16\\]
GIO flag for pins GIOC\\[7:0\\]"]
pub type Gioflg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOFLG_3` reader - 31:24\\]
GIO flag for pins GIOD\\[7:0\\]"]
pub type Gioflg3R = crate::FieldReader;
#[doc = "Field `GIOFLG_3` writer - 31:24\\]
GIO flag for pins GIOD\\[7:0\\]"]
pub type Gioflg3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO flag for pins GIOA\\[7:0\\]"]
    #[inline(always)]
    pub fn gioflg_0(&self) -> Gioflg0R {
        Gioflg0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
GIO flag for pins GIOB\\[7:0\\]"]
    #[inline(always)]
    pub fn gioflg_1(&self) -> Gioflg1R {
        Gioflg1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
GIO flag for pins GIOC\\[7:0\\]"]
    #[inline(always)]
    pub fn gioflg_2(&self) -> Gioflg2R {
        Gioflg2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
GIO flag for pins GIOD\\[7:0\\]"]
    #[inline(always)]
    pub fn gioflg_3(&self) -> Gioflg3R {
        Gioflg3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO flag for pins GIOA\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gioflg_0(&mut self) -> Gioflg0W<GioflgSpec> {
        Gioflg0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
GIO flag for pins GIOB\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gioflg_1(&mut self) -> Gioflg1W<GioflgSpec> {
        Gioflg1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
GIO flag for pins GIOC\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gioflg_2(&mut self) -> Gioflg2W<GioflgSpec> {
        Gioflg2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
GIO flag for pins GIOD\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gioflg_3(&mut self) -> Gioflg3W<GioflgSpec> {
        Gioflg3W::new(self, 24)
    }
}
#[doc = "GIO flag for pins \\[0:1\\]
GIO\\[7:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`gioflg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioflg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GioflgSpec;
impl crate::RegisterSpec for GioflgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gioflg::R`](R) reader structure"]
impl crate::Readable for GioflgSpec {}
#[doc = "`write(|w| ..)` method takes [`gioflg::W`](W) writer structure"]
impl crate::Writable for GioflgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOFLG to value 0"]
impl crate::Resettable for GioflgSpec {
    const RESET_VALUE: u32 = 0;
}
