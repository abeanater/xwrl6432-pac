#[doc = "Register `GIOENACLR` reader"]
pub type R = crate::R<GioenaclrSpec>;
#[doc = "Register `GIOENACLR` writer"]
pub type W = crate::W<GioenaclrSpec>;
#[doc = "Field `GIOENACLR_0` reader - 7:0\\]
Interrupt enable for pins GIOA \\[7:0\\]"]
pub type Gioenaclr0R = crate::FieldReader;
#[doc = "Field `GIOENACLR_0` writer - 7:0\\]
Interrupt enable for pins GIOA \\[7:0\\]"]
pub type Gioenaclr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOENACLR_1` reader - 15:8\\]
Interrupt enable for pins GIOB \\[7:0\\]"]
pub type Gioenaclr1R = crate::FieldReader;
#[doc = "Field `GIOENACLR_1` writer - 15:8\\]
Interrupt enable for pins GIOB \\[7:0\\]"]
pub type Gioenaclr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOENACLR_2` reader - 23:16\\]
Interrupt enable for pins GIOC \\[7:0\\]"]
pub type Gioenaclr2R = crate::FieldReader;
#[doc = "Field `GIOENACLR_2` writer - 23:16\\]
Interrupt enable for pins GIOC \\[7:0\\]"]
pub type Gioenaclr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOENACLR_3` reader - 31:24\\]
Interrupt enable for pins GIOD \\[7:0\\]"]
pub type Gioenaclr3R = crate::FieldReader;
#[doc = "Field `GIOENACLR_3` writer - 31:24\\]
Interrupt enable for pins GIOD \\[7:0\\]"]
pub type Gioenaclr3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt enable for pins GIOA \\[7:0\\]"]
    #[inline(always)]
    pub fn gioenaclr_0(&self) -> Gioenaclr0R {
        Gioenaclr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Interrupt enable for pins GIOB \\[7:0\\]"]
    #[inline(always)]
    pub fn gioenaclr_1(&self) -> Gioenaclr1R {
        Gioenaclr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Interrupt enable for pins GIOC \\[7:0\\]"]
    #[inline(always)]
    pub fn gioenaclr_2(&self) -> Gioenaclr2R {
        Gioenaclr2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Interrupt enable for pins GIOD \\[7:0\\]"]
    #[inline(always)]
    pub fn gioenaclr_3(&self) -> Gioenaclr3R {
        Gioenaclr3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt enable for pins GIOA \\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gioenaclr_0(&mut self) -> Gioenaclr0W<GioenaclrSpec> {
        Gioenaclr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Interrupt enable for pins GIOB \\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gioenaclr_1(&mut self) -> Gioenaclr1W<GioenaclrSpec> {
        Gioenaclr1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Interrupt enable for pins GIOC \\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gioenaclr_2(&mut self) -> Gioenaclr2W<GioenaclrSpec> {
        Gioenaclr2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Interrupt enable for pins GIOD \\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gioenaclr_3(&mut self) -> Gioenaclr3W<GioenaclrSpec> {
        Gioenaclr3W::new(self, 24)
    }
}
#[doc = "Interrupt enable for pins \\[0:1\\]
GIO\\[7:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`gioenaclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioenaclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GioenaclrSpec;
impl crate::RegisterSpec for GioenaclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gioenaclr::R`](R) reader structure"]
impl crate::Readable for GioenaclrSpec {}
#[doc = "`write(|w| ..)` method takes [`gioenaclr::W`](W) writer structure"]
impl crate::Writable for GioenaclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOENACLR to value 0"]
impl crate::Resettable for GioenaclrSpec {
    const RESET_VALUE: u32 = 0;
}
