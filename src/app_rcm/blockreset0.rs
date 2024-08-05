#[doc = "Register `BLOCKRESET0` reader"]
pub type R = crate::R<Blockreset0Spec>;
#[doc = "Register `BLOCKRESET0` writer"]
pub type W = crate::W<Blockreset0Spec>;
#[doc = "Field `hwass` reader - 2:0\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type HwassR = crate::FieldReader;
#[doc = "Field `hwass` writer - 2:0\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type HwassW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_qspi` reader - 5:3\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type AppQspiR = crate::FieldReader;
#[doc = "Field `app_qspi` writer - 5:3\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type AppQspiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tptc_a0` reader - 8:6\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type TptcA0R = crate::FieldReader;
#[doc = "Field `tptc_a0` writer - 8:6\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type TptcA0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tptc_a1` reader - 11:9\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type TptcA1R = crate::FieldReader;
#[doc = "Field `tptc_a1` writer - 11:9\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type TptcA1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tpcc_a` reader - 14:12\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type TpccAR = crate::FieldReader;
#[doc = "Field `tpcc_a` writer - 14:12\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type TpccAW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_esm` reader - 17:15\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type AppEsmR = crate::FieldReader;
#[doc = "Field `app_esm` writer - 17:15\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type AppEsmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_rti` reader - 20:18\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type AppRtiR = crate::FieldReader;
#[doc = "Field `app_rti` writer - 20:18\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type AppRtiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_wd` reader - 23:21\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type AppWdR = crate::FieldReader;
#[doc = "Field `app_wd` writer - 23:21\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type AppWdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_dcc` reader - 26:24\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type AppDccR = crate::FieldReader;
#[doc = "Field `app_dcc` writer - 26:24\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type AppDccW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_i2c` reader - 29:27\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type AppI2cR = crate::FieldReader;
#[doc = "Field `app_i2c` writer - 29:27\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type AppI2cW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    pub fn hwass(&self) -> HwassR {
        HwassR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    pub fn app_qspi(&self) -> AppQspiR {
        AppQspiR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    pub fn tptc_a0(&self) -> TptcA0R {
        TptcA0R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    pub fn tptc_a1(&self) -> TptcA1R {
        TptcA1R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    pub fn tpcc_a(&self) -> TpccAR {
        TpccAR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - 17:15\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    pub fn app_esm(&self) -> AppEsmR {
        AppEsmR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - 20:18\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    pub fn app_rti(&self) -> AppRtiR {
        AppRtiR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    pub fn app_wd(&self) -> AppWdR {
        AppWdR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    pub fn app_dcc(&self) -> AppDccR {
        AppDccR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - 29:27\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    pub fn app_i2c(&self) -> AppI2cR {
        AppI2cR::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn hwass(&mut self) -> HwassW<Blockreset0Spec> {
        HwassW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn app_qspi(&mut self) -> AppQspiW<Blockreset0Spec> {
        AppQspiW::new(self, 3)
    }
    #[doc = "Bits 6:8 - 8:6\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0(&mut self) -> TptcA0W<Blockreset0Spec> {
        TptcA0W::new(self, 6)
    }
    #[doc = "Bits 9:11 - 11:9\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a1(&mut self) -> TptcA1W<Blockreset0Spec> {
        TptcA1W::new(self, 9)
    }
    #[doc = "Bits 12:14 - 14:12\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a(&mut self) -> TpccAW<Blockreset0Spec> {
        TpccAW::new(self, 12)
    }
    #[doc = "Bits 15:17 - 17:15\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn app_esm(&mut self) -> AppEsmW<Blockreset0Spec> {
        AppEsmW::new(self, 15)
    }
    #[doc = "Bits 18:20 - 20:18\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn app_rti(&mut self) -> AppRtiW<Blockreset0Spec> {
        AppRtiW::new(self, 18)
    }
    #[doc = "Bits 21:23 - 23:21\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn app_wd(&mut self) -> AppWdW<Blockreset0Spec> {
        AppWdW::new(self, 21)
    }
    #[doc = "Bits 24:26 - 26:24\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn app_dcc(&mut self) -> AppDccW<Blockreset0Spec> {
        AppDccW::new(self, 24)
    }
    #[doc = "Bits 27:29 - 29:27\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn app_i2c(&mut self) -> AppI2cW<Blockreset0Spec> {
        AppI2cW::new(self, 27)
    }
}
#[doc = "BLOCKRESET0\n\nYou can [`read`](crate::Reg::read) this register and get [`blockreset0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blockreset0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Blockreset0Spec;
impl crate::RegisterSpec for Blockreset0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blockreset0::R`](R) reader structure"]
impl crate::Readable for Blockreset0Spec {}
#[doc = "`write(|w| ..)` method takes [`blockreset0::W`](W) writer structure"]
impl crate::Writable for Blockreset0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLOCKRESET0 to value 0"]
impl crate::Resettable for Blockreset0Spec {
    const RESET_VALUE: u32 = 0;
}
