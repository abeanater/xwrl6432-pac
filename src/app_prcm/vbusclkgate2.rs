#[doc = "Register `VBUSCLKGATE2` reader"]
pub type R = crate::R<Vbusclkgate2Spec>;
#[doc = "Register `VBUSCLKGATE2` writer"]
pub type W = crate::W<Vbusclkgate2Spec>;
#[doc = "Field `pcr3` reader - 2:0\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Pcr3R = crate::FieldReader;
#[doc = "Field `pcr3` writer - 2:0\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Pcr3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pcr4` reader - 5:3\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Pcr4R = crate::FieldReader;
#[doc = "Field `pcr4` writer - 5:3\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Pcr4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pcr5` reader - 8:6\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Pcr5R = crate::FieldReader;
#[doc = "Field `pcr5` writer - 8:6\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Pcr5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pcr6` reader - 11:9\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Pcr6R = crate::FieldReader;
#[doc = "Field `pcr6` writer - 11:9\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Pcr6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `res0` reader - 14:12\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Res0R = crate::FieldReader;
#[doc = "Field `res0` writer - 14:12\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Res0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `res1` reader - 17:15\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Res1R = crate::FieldReader;
#[doc = "Field `res1` writer - 17:15\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Res1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `res2` reader - 20:18\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Res2R = crate::FieldReader;
#[doc = "Field `res2` writer - 20:18\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Res2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `res3` reader - 23:21\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Res3R = crate::FieldReader;
#[doc = "Field `res3` writer - 23:21\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Res3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `res4` reader - 26:24\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Res4R = crate::FieldReader;
#[doc = "Field `res4` writer - 26:24\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Res4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `res5` reader - 29:27\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Res5R = crate::FieldReader;
#[doc = "Field `res5` writer - 29:27\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Res5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn pcr3(&self) -> Pcr3R {
        Pcr3R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn pcr4(&self) -> Pcr4R {
        Pcr4R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn pcr5(&self) -> Pcr5R {
        Pcr5R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn pcr6(&self) -> Pcr6R {
        Pcr6R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn res0(&self) -> Res0R {
        Res0R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - 17:15\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn res1(&self) -> Res1R {
        Res1R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - 20:18\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn res2(&self) -> Res2R {
        Res2R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn res3(&self) -> Res3R {
        Res3R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn res4(&self) -> Res4R {
        Res4R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - 29:27\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn res5(&self) -> Res5R {
        Res5R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn pcr3(&mut self) -> Pcr3W<Vbusclkgate2Spec> {
        Pcr3W::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn pcr4(&mut self) -> Pcr4W<Vbusclkgate2Spec> {
        Pcr4W::new(self, 3)
    }
    #[doc = "Bits 6:8 - 8:6\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn pcr5(&mut self) -> Pcr5W<Vbusclkgate2Spec> {
        Pcr5W::new(self, 6)
    }
    #[doc = "Bits 9:11 - 11:9\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn pcr6(&mut self) -> Pcr6W<Vbusclkgate2Spec> {
        Pcr6W::new(self, 9)
    }
    #[doc = "Bits 12:14 - 14:12\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn res0(&mut self) -> Res0W<Vbusclkgate2Spec> {
        Res0W::new(self, 12)
    }
    #[doc = "Bits 15:17 - 17:15\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn res1(&mut self) -> Res1W<Vbusclkgate2Spec> {
        Res1W::new(self, 15)
    }
    #[doc = "Bits 18:20 - 20:18\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn res2(&mut self) -> Res2W<Vbusclkgate2Spec> {
        Res2W::new(self, 18)
    }
    #[doc = "Bits 21:23 - 23:21\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn res3(&mut self) -> Res3W<Vbusclkgate2Spec> {
        Res3W::new(self, 21)
    }
    #[doc = "Bits 24:26 - 26:24\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn res4(&mut self) -> Res4W<Vbusclkgate2Spec> {
        Res4W::new(self, 24)
    }
    #[doc = "Bits 27:29 - 29:27\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn res5(&mut self) -> Res5W<Vbusclkgate2Spec> {
        Res5W::new(self, 27)
    }
}
#[doc = "VBUSCLKGATE2\n\nYou can [`read`](crate::Reg::read) this register and get [`vbusclkgate2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbusclkgate2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusclkgate2Spec;
impl crate::RegisterSpec for Vbusclkgate2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusclkgate2::R`](R) reader structure"]
impl crate::Readable for Vbusclkgate2Spec {}
#[doc = "`write(|w| ..)` method takes [`vbusclkgate2::W`](W) writer structure"]
impl crate::Writable for Vbusclkgate2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSCLKGATE2 to value 0"]
impl crate::Resettable for Vbusclkgate2Spec {
    const RESET_VALUE: u32 = 0;
}
