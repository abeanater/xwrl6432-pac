#[doc = "Register `BLOCKRESET1` reader"]
pub type R = crate::R<Blockreset1Spec>;
#[doc = "Register `BLOCKRESET1` writer"]
pub type W = crate::W<Blockreset1Spec>;
#[doc = "Field `uart1` reader - 2:0\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type Uart1R = crate::FieldReader;
#[doc = "Field `uart1` writer - 2:0\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type Uart1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `uart2` reader - 5:3\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type Uart2R = crate::FieldReader;
#[doc = "Field `uart2` writer - 5:3\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type Uart2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `spi1` reader - 8:6\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type Spi1R = crate::FieldReader;
#[doc = "Field `spi1` writer - 8:6\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type Spi1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `spi2` reader - 11:9\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type Spi2R = crate::FieldReader;
#[doc = "Field `spi2` writer - 11:9\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type Spi2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `can` reader - 14:12\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type CanR = crate::FieldReader;
#[doc = "Field `can` writer - 14:12\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type CanW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `lin` reader - 17:15\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type LinR = crate::FieldReader;
#[doc = "Field `lin` writer - 17:15\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type LinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pwm` reader - 20:18\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type PwmR = crate::FieldReader;
#[doc = "Field `pwm` writer - 20:18\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type PwmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `crc` reader - 23:21\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type CrcR = crate::FieldReader;
#[doc = "Field `crc` writer - 23:21\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type CrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ctrl_reg` reader - 26:24\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type CtrlRegR = crate::FieldReader;
#[doc = "Field `ctrl_reg` writer - 26:24\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type CtrlRegW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `res` reader - 29:27\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type ResR = crate::FieldReader;
#[doc = "Field `res` writer - 29:27\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    pub fn spi2(&self) -> Spi2R {
        Spi2R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    pub fn can(&self) -> CanR {
        CanR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - 17:15\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    pub fn lin(&self) -> LinR {
        LinR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - 20:18\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    pub fn pwm(&self) -> PwmR {
        PwmR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    pub fn ctrl_reg(&self) -> CtrlRegR {
        CtrlRegR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - 29:27\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> Uart1W<Blockreset1Spec> {
        Uart1W::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> Uart2W<Blockreset1Spec> {
        Uart2W::new(self, 3)
    }
    #[doc = "Bits 6:8 - 8:6\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> Spi1W<Blockreset1Spec> {
        Spi1W::new(self, 6)
    }
    #[doc = "Bits 9:11 - 11:9\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> Spi2W<Blockreset1Spec> {
        Spi2W::new(self, 9)
    }
    #[doc = "Bits 12:14 - 14:12\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn can(&mut self) -> CanW<Blockreset1Spec> {
        CanW::new(self, 12)
    }
    #[doc = "Bits 15:17 - 17:15\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn lin(&mut self) -> LinW<Blockreset1Spec> {
        LinW::new(self, 15)
    }
    #[doc = "Bits 18:20 - 20:18\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwm(&mut self) -> PwmW<Blockreset1Spec> {
        PwmW::new(self, 18)
    }
    #[doc = "Bits 21:23 - 23:21\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CrcW<Blockreset1Spec> {
        CrcW::new(self, 21)
    }
    #[doc = "Bits 24:26 - 26:24\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_reg(&mut self) -> CtrlRegW<Blockreset1Spec> {
        CtrlRegW::new(self, 24)
    }
    #[doc = "Bits 27:29 - 29:27\\]
0x0 : Release the reset#br#0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> ResW<Blockreset1Spec> {
        ResW::new(self, 27)
    }
}
#[doc = "BLOCKRESET1\n\nYou can [`read`](crate::Reg::read) this register and get [`blockreset1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blockreset1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Blockreset1Spec;
impl crate::RegisterSpec for Blockreset1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blockreset1::R`](R) reader structure"]
impl crate::Readable for Blockreset1Spec {}
#[doc = "`write(|w| ..)` method takes [`blockreset1::W`](W) writer structure"]
impl crate::Writable for Blockreset1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLOCKRESET1 to value 0"]
impl crate::Resettable for Blockreset1Spec {
    const RESET_VALUE: u32 = 0;
}
