#[doc = "Register `GIOLVLSET` reader"]
pub type R = crate::R<GiolvlsetSpec>;
#[doc = "Register `GIOLVLSET` writer"]
pub type W = crate::W<GiolvlsetSpec>;
#[doc = "Field `GIOLVLSET_0` reader - 7:0\\]
GIO high priority interrupt for pins GIOA\\[7:0\\]"]
pub type Giolvlset0R = crate::FieldReader;
#[doc = "Field `GIOLVLSET_0` writer - 7:0\\]
GIO high priority interrupt for pins GIOA\\[7:0\\]"]
pub type Giolvlset0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOLVLSET_1` reader - 15:8\\]
GIO high priority interrupt for pins GIOB\\[7:0\\]"]
pub type Giolvlset1R = crate::FieldReader;
#[doc = "Field `GIOLVLSET_1` writer - 15:8\\]
GIO high priority interrupt for pins GIOB\\[7:0\\]"]
pub type Giolvlset1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOLVLSET_2` reader - 23:16\\]
GIO high priority interrupt for pins GIOC\\[7:0\\]"]
pub type Giolvlset2R = crate::FieldReader;
#[doc = "Field `GIOLVLSET_2` writer - 23:16\\]
GIO high priority interrupt for pins GIOC\\[7:0\\]"]
pub type Giolvlset2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOLVLSET_3` reader - 31:24\\]
GIO high priority interrupt for pins GIOD\\[7:0\\]"]
pub type Giolvlset3R = crate::FieldReader;
#[doc = "Field `GIOLVLSET_3` writer - 31:24\\]
GIO high priority interrupt for pins GIOD\\[7:0\\]"]
pub type Giolvlset3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO high priority interrupt for pins GIOA\\[7:0\\]"]
    #[inline(always)]
    pub fn giolvlset_0(&self) -> Giolvlset0R {
        Giolvlset0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
GIO high priority interrupt for pins GIOB\\[7:0\\]"]
    #[inline(always)]
    pub fn giolvlset_1(&self) -> Giolvlset1R {
        Giolvlset1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
GIO high priority interrupt for pins GIOC\\[7:0\\]"]
    #[inline(always)]
    pub fn giolvlset_2(&self) -> Giolvlset2R {
        Giolvlset2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
GIO high priority interrupt for pins GIOD\\[7:0\\]"]
    #[inline(always)]
    pub fn giolvlset_3(&self) -> Giolvlset3R {
        Giolvlset3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO high priority interrupt for pins GIOA\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn giolvlset_0(&mut self) -> Giolvlset0W<GiolvlsetSpec> {
        Giolvlset0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
GIO high priority interrupt for pins GIOB\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn giolvlset_1(&mut self) -> Giolvlset1W<GiolvlsetSpec> {
        Giolvlset1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
GIO high priority interrupt for pins GIOC\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn giolvlset_2(&mut self) -> Giolvlset2W<GiolvlsetSpec> {
        Giolvlset2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
GIO high priority interrupt for pins GIOD\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn giolvlset_3(&mut self) -> Giolvlset3W<GiolvlsetSpec> {
        Giolvlset3W::new(self, 24)
    }
}
#[doc = "GIO high priority interrupt for pins \\[0:1\\]
GIO\\[7:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`giolvlset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giolvlset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiolvlsetSpec;
impl crate::RegisterSpec for GiolvlsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giolvlset::R`](R) reader structure"]
impl crate::Readable for GiolvlsetSpec {}
#[doc = "`write(|w| ..)` method takes [`giolvlset::W`](W) writer structure"]
impl crate::Writable for GiolvlsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOLVLSET to value 0"]
impl crate::Resettable for GiolvlsetSpec {
    const RESET_VALUE: u32 = 0;
}
