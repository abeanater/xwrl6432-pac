#[doc = "Register `GIOENASET` reader"]
pub type R = crate::R<GioenasetSpec>;
#[doc = "Register `GIOENASET` writer"]
pub type W = crate::W<GioenasetSpec>;
#[doc = "Field `GIOENASET_0` reader - 7:0\\]
Interrupt enable for pins GIOA \\[7:0\\]"]
pub type Gioenaset0R = crate::FieldReader;
#[doc = "Field `GIOENASET_0` writer - 7:0\\]
Interrupt enable for pins GIOA \\[7:0\\]"]
pub type Gioenaset0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOENASET_1` reader - 15:8\\]
Interrupt enable for pins GIOB \\[7:0\\]"]
pub type Gioenaset1R = crate::FieldReader;
#[doc = "Field `GIOENASET_1` writer - 15:8\\]
Interrupt enable for pins GIOB \\[7:0\\]"]
pub type Gioenaset1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOENASET_2` reader - 23:16\\]
Interrupt enable for pins GIOC \\[7:0\\]"]
pub type Gioenaset2R = crate::FieldReader;
#[doc = "Field `GIOENASET_2` writer - 23:16\\]
Interrupt enable for pins GIOC \\[7:0\\]"]
pub type Gioenaset2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOENASET_3` reader - 31:24\\]
Interrupt enable for pins GIOD \\[7:0\\]"]
pub type Gioenaset3R = crate::FieldReader;
#[doc = "Field `GIOENASET_3` writer - 31:24\\]
Interrupt enable for pins GIOD \\[7:0\\]"]
pub type Gioenaset3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt enable for pins GIOA \\[7:0\\]"]
    #[inline(always)]
    pub fn gioenaset_0(&self) -> Gioenaset0R {
        Gioenaset0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Interrupt enable for pins GIOB \\[7:0\\]"]
    #[inline(always)]
    pub fn gioenaset_1(&self) -> Gioenaset1R {
        Gioenaset1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Interrupt enable for pins GIOC \\[7:0\\]"]
    #[inline(always)]
    pub fn gioenaset_2(&self) -> Gioenaset2R {
        Gioenaset2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Interrupt enable for pins GIOD \\[7:0\\]"]
    #[inline(always)]
    pub fn gioenaset_3(&self) -> Gioenaset3R {
        Gioenaset3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt enable for pins GIOA \\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gioenaset_0(&mut self) -> Gioenaset0W<GioenasetSpec> {
        Gioenaset0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Interrupt enable for pins GIOB \\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gioenaset_1(&mut self) -> Gioenaset1W<GioenasetSpec> {
        Gioenaset1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Interrupt enable for pins GIOC \\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gioenaset_2(&mut self) -> Gioenaset2W<GioenasetSpec> {
        Gioenaset2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Interrupt enable for pins GIOD \\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gioenaset_3(&mut self) -> Gioenaset3W<GioenasetSpec> {
        Gioenaset3W::new(self, 24)
    }
}
#[doc = "Interrupt enable for pins \\[0:1\\]
GIO\\[7:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`gioenaset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioenaset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GioenasetSpec;
impl crate::RegisterSpec for GioenasetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gioenaset::R`](R) reader structure"]
impl crate::Readable for GioenasetSpec {}
#[doc = "`write(|w| ..)` method takes [`gioenaset::W`](W) writer structure"]
impl crate::Writable for GioenasetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOENASET to value 0"]
impl crate::Resettable for GioenasetSpec {
    const RESET_VALUE: u32 = 0;
}
