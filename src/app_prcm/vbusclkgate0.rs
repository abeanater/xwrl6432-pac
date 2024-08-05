#[doc = "Register `VBUSCLKGATE0` reader"]
pub type R = crate::R<Vbusclkgate0Spec>;
#[doc = "Register `VBUSCLKGATE0` writer"]
pub type W = crate::W<Vbusclkgate0Spec>;
#[doc = "Field `xbara` reader - 2:0\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type XbaraR = crate::FieldReader;
#[doc = "Field `xbara` writer - 2:0\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type XbaraW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `qspi` reader - 5:3\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type QspiR = crate::FieldReader;
#[doc = "Field `qspi` writer - 5:3\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type QspiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tptc1` reader - 8:6\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Tptc1R = crate::FieldReader;
#[doc = "Field `tptc1` writer - 8:6\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Tptc1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tptc2` reader - 11:9\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Tptc2R = crate::FieldReader;
#[doc = "Field `tptc2` writer - 11:9\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type Tptc2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tpcc` reader - 14:12\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type TpccR = crate::FieldReader;
#[doc = "Field `tpcc` writer - 14:12\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type TpccW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `esm` reader - 17:15\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type EsmR = crate::FieldReader;
#[doc = "Field `esm` writer - 17:15\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type EsmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rti` reader - 20:18\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type RtiR = crate::FieldReader;
#[doc = "Field `rti` writer - 20:18\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type RtiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `wdt` reader - 23:21\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type WdtR = crate::FieldReader;
#[doc = "Field `wdt` writer - 23:21\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type WdtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcc` reader - 26:24\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type DccR = crate::FieldReader;
#[doc = "Field `dcc` writer - 26:24\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type DccW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `i2c` reader - 29:27\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type I2cR = crate::FieldReader;
#[doc = "Field `i2c` writer - 29:27\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
pub type I2cW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn xbara(&self) -> XbaraR {
        XbaraR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn qspi(&self) -> QspiR {
        QspiR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn tptc1(&self) -> Tptc1R {
        Tptc1R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn tptc2(&self) -> Tptc2R {
        Tptc2R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn tpcc(&self) -> TpccR {
        TpccR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - 17:15\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn esm(&self) -> EsmR {
        EsmR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - 20:18\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn rti(&self) -> RtiR {
        RtiR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn wdt(&self) -> WdtR {
        WdtR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn dcc(&self) -> DccR {
        DccR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - 29:27\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    pub fn i2c(&self) -> I2cR {
        I2cR::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn xbara(&mut self) -> XbaraW<Vbusclkgate0Spec> {
        XbaraW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn qspi(&mut self) -> QspiW<Vbusclkgate0Spec> {
        QspiW::new(self, 3)
    }
    #[doc = "Bits 6:8 - 8:6\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn tptc1(&mut self) -> Tptc1W<Vbusclkgate0Spec> {
        Tptc1W::new(self, 6)
    }
    #[doc = "Bits 9:11 - 11:9\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn tptc2(&mut self) -> Tptc2W<Vbusclkgate0Spec> {
        Tptc2W::new(self, 9)
    }
    #[doc = "Bits 12:14 - 14:12\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc(&mut self) -> TpccW<Vbusclkgate0Spec> {
        TpccW::new(self, 12)
    }
    #[doc = "Bits 15:17 - 17:15\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn esm(&mut self) -> EsmW<Vbusclkgate0Spec> {
        EsmW::new(self, 15)
    }
    #[doc = "Bits 18:20 - 20:18\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn rti(&mut self) -> RtiW<Vbusclkgate0Spec> {
        RtiW::new(self, 18)
    }
    #[doc = "Bits 21:23 - 23:21\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WdtW<Vbusclkgate0Spec> {
        WdtW::new(self, 21)
    }
    #[doc = "Bits 24:26 - 26:24\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn dcc(&mut self) -> DccW<Vbusclkgate0Spec> {
        DccW::new(self, 24)
    }
    #[doc = "Bits 27:29 - 29:27\\]
0x0 : Enable the Clock#br#0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2cW<Vbusclkgate0Spec> {
        I2cW::new(self, 27)
    }
}
#[doc = "VBUSCLKGATE0\n\nYou can [`read`](crate::Reg::read) this register and get [`vbusclkgate0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbusclkgate0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusclkgate0Spec;
impl crate::RegisterSpec for Vbusclkgate0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusclkgate0::R`](R) reader structure"]
impl crate::Readable for Vbusclkgate0Spec {}
#[doc = "`write(|w| ..)` method takes [`vbusclkgate0::W`](W) writer structure"]
impl crate::Writable for Vbusclkgate0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSCLKGATE0 to value 0"]
impl crate::Resettable for Vbusclkgate0Spec {
    const RESET_VALUE: u32 = 0;
}
