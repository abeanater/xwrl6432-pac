#[doc = "Register `IPCFGCLKGATE1` reader"]
pub type R = crate::R<Ipcfgclkgate1Spec>;
#[doc = "Register `IPCFGCLKGATE1` writer"]
pub type W = crate::W<Ipcfgclkgate1Spec>;
#[doc = "Field `app_uart_0` reader - 2:0\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppUart0R = crate::FieldReader;
#[doc = "Field `app_uart_0` writer - 2:0\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppUart0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_uart_1` reader - 5:3\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppUart1R = crate::FieldReader;
#[doc = "Field `app_uart_1` writer - 5:3\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppUart1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_spi_0` reader - 8:6\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppSpi0R = crate::FieldReader;
#[doc = "Field `app_spi_0` writer - 8:6\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppSpi0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_spi_1` reader - 11:9\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppSpi1R = crate::FieldReader;
#[doc = "Field `app_spi_1` writer - 11:9\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppSpi1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_can` reader - 14:12\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppCanR = crate::FieldReader;
#[doc = "Field `app_can` writer - 14:12\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppCanW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_lin` reader - 17:15\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppLinR = crate::FieldReader;
#[doc = "Field `app_lin` writer - 17:15\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppLinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_pwm` reader - 20:18\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppPwmR = crate::FieldReader;
#[doc = "Field `app_pwm` writer - 20:18\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppPwmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_crc` reader - 23:21\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppCrcR = crate::FieldReader;
#[doc = "Field `app_crc` writer - 23:21\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppCrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_ctrl` reader - 26:24\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppCtrlR = crate::FieldReader;
#[doc = "Field `app_ctrl` writer - 26:24\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type AppCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `res` reader - 29:27\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type ResR = crate::FieldReader;
#[doc = "Field `res` writer - 29:27\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn app_uart_0(&self) -> AppUart0R {
        AppUart0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn app_uart_1(&self) -> AppUart1R {
        AppUart1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn app_spi_0(&self) -> AppSpi0R {
        AppSpi0R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn app_spi_1(&self) -> AppSpi1R {
        AppSpi1R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn app_can(&self) -> AppCanR {
        AppCanR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - 17:15\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn app_lin(&self) -> AppLinR {
        AppLinR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - 20:18\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn app_pwm(&self) -> AppPwmR {
        AppPwmR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn app_crc(&self) -> AppCrcR {
        AppCrcR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn app_ctrl(&self) -> AppCtrlR {
        AppCtrlR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - 29:27\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn app_uart_0(&mut self) -> AppUart0W<Ipcfgclkgate1Spec> {
        AppUart0W::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn app_uart_1(&mut self) -> AppUart1W<Ipcfgclkgate1Spec> {
        AppUart1W::new(self, 3)
    }
    #[doc = "Bits 6:8 - 8:6\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn app_spi_0(&mut self) -> AppSpi0W<Ipcfgclkgate1Spec> {
        AppSpi0W::new(self, 6)
    }
    #[doc = "Bits 9:11 - 11:9\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn app_spi_1(&mut self) -> AppSpi1W<Ipcfgclkgate1Spec> {
        AppSpi1W::new(self, 9)
    }
    #[doc = "Bits 12:14 - 14:12\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn app_can(&mut self) -> AppCanW<Ipcfgclkgate1Spec> {
        AppCanW::new(self, 12)
    }
    #[doc = "Bits 15:17 - 17:15\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn app_lin(&mut self) -> AppLinW<Ipcfgclkgate1Spec> {
        AppLinW::new(self, 15)
    }
    #[doc = "Bits 18:20 - 20:18\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn app_pwm(&mut self) -> AppPwmW<Ipcfgclkgate1Spec> {
        AppPwmW::new(self, 18)
    }
    #[doc = "Bits 21:23 - 23:21\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn app_crc(&mut self) -> AppCrcW<Ipcfgclkgate1Spec> {
        AppCrcW::new(self, 21)
    }
    #[doc = "Bits 24:26 - 26:24\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl(&mut self) -> AppCtrlW<Ipcfgclkgate1Spec> {
        AppCtrlW::new(self, 24)
    }
    #[doc = "Bits 27:29 - 29:27\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> ResW<Ipcfgclkgate1Spec> {
        ResW::new(self, 27)
    }
}
#[doc = "IPCFGCLKGATE1\n\nYou can [`read`](crate::Reg::read) this register and get [`ipcfgclkgate1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcfgclkgate1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipcfgclkgate1Spec;
impl crate::RegisterSpec for Ipcfgclkgate1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipcfgclkgate1::R`](R) reader structure"]
impl crate::Readable for Ipcfgclkgate1Spec {}
#[doc = "`write(|w| ..)` method takes [`ipcfgclkgate1::W`](W) writer structure"]
impl crate::Writable for Ipcfgclkgate1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPCFGCLKGATE1 to value 0"]
impl crate::Resettable for Ipcfgclkgate1Spec {
    const RESET_VALUE: u32 = 0;
}
