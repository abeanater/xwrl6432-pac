#[doc = "Register `GIOLVLCLR` reader"]
pub type R = crate::R<GiolvlclrSpec>;
#[doc = "Register `GIOLVLCLR` writer"]
pub type W = crate::W<GiolvlclrSpec>;
#[doc = "Field `GIOLVLCLR_0` reader - 7:0\\]
GIO low priority interrupt for pins GIOA\\[7:0\\]"]
pub type Giolvlclr0R = crate::FieldReader;
#[doc = "Field `GIOLVLCLR_0` writer - 7:0\\]
GIO low priority interrupt for pins GIOA\\[7:0\\]"]
pub type Giolvlclr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOLVLCLR_1` reader - 15:8\\]
GIO low priority interrupt for pins GIOB\\[7:0\\]"]
pub type Giolvlclr1R = crate::FieldReader;
#[doc = "Field `GIOLVLCLR_1` writer - 15:8\\]
GIO low priority interrupt for pins GIOB\\[7:0\\]"]
pub type Giolvlclr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOLVLCLR_2` reader - 23:16\\]
GIO low priority interrupt for pins GIOC\\[7:0\\]"]
pub type Giolvlclr2R = crate::FieldReader;
#[doc = "Field `GIOLVLCLR_2` writer - 23:16\\]
GIO low priority interrupt for pins GIOC\\[7:0\\]"]
pub type Giolvlclr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOLVLCLR_3` reader - 31:24\\]
GIO low priority interrupt for pins GIOD\\[7:0\\]"]
pub type Giolvlclr3R = crate::FieldReader;
#[doc = "Field `GIOLVLCLR_3` writer - 31:24\\]
GIO low priority interrupt for pins GIOD\\[7:0\\]"]
pub type Giolvlclr3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO low priority interrupt for pins GIOA\\[7:0\\]"]
    #[inline(always)]
    pub fn giolvlclr_0(&self) -> Giolvlclr0R {
        Giolvlclr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
GIO low priority interrupt for pins GIOB\\[7:0\\]"]
    #[inline(always)]
    pub fn giolvlclr_1(&self) -> Giolvlclr1R {
        Giolvlclr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
GIO low priority interrupt for pins GIOC\\[7:0\\]"]
    #[inline(always)]
    pub fn giolvlclr_2(&self) -> Giolvlclr2R {
        Giolvlclr2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
GIO low priority interrupt for pins GIOD\\[7:0\\]"]
    #[inline(always)]
    pub fn giolvlclr_3(&self) -> Giolvlclr3R {
        Giolvlclr3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO low priority interrupt for pins GIOA\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn giolvlclr_0(&mut self) -> Giolvlclr0W<GiolvlclrSpec> {
        Giolvlclr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
GIO low priority interrupt for pins GIOB\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn giolvlclr_1(&mut self) -> Giolvlclr1W<GiolvlclrSpec> {
        Giolvlclr1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
GIO low priority interrupt for pins GIOC\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn giolvlclr_2(&mut self) -> Giolvlclr2W<GiolvlclrSpec> {
        Giolvlclr2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
GIO low priority interrupt for pins GIOD\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn giolvlclr_3(&mut self) -> Giolvlclr3W<GiolvlclrSpec> {
        Giolvlclr3W::new(self, 24)
    }
}
#[doc = "GIO low priority interrupt for pins \\[0:1\\]
GIO\\[7:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`giolvlclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giolvlclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiolvlclrSpec;
impl crate::RegisterSpec for GiolvlclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giolvlclr::R`](R) reader structure"]
impl crate::Readable for GiolvlclrSpec {}
#[doc = "`write(|w| ..)` method takes [`giolvlclr::W`](W) writer structure"]
impl crate::Writable for GiolvlclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOLVLCLR to value 0"]
impl crate::Resettable for GiolvlclrSpec {
    const RESET_VALUE: u32 = 0;
}
