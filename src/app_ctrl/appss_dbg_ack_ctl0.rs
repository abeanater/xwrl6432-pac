#[doc = "Register `APPSS_DBG_ACK_CTL0` reader"]
pub type R = crate::R<AppssDbgAckCtl0Spec>;
#[doc = "Register `APPSS_DBG_ACK_CTL0` writer"]
pub type W = crate::W<AppssDbgAckCtl0Spec>;
#[doc = "Field `can` reader - 0:0\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type CanR = crate::BitReader;
#[doc = "Field `can` writer - 0:0\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type CanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rti` reader - 4:4\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type RtiR = crate::BitReader;
#[doc = "Field `rti` writer - 4:4\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type RtiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdt` reader - 8:8\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type WdtR = crate::BitReader;
#[doc = "Field `wdt` writer - 8:8\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type WdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mcrc` reader - 12:12\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type McrcR = crate::BitReader;
#[doc = "Field `mcrc` writer - 12:12\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type McrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c` reader - 16:16\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type I2cR = crate::BitReader;
#[doc = "Field `i2c` writer - 16:16\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type I2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scia` reader - 20:20\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type SciaR = crate::BitReader;
#[doc = "Field `scia` writer - 20:20\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type SciaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scib` reader - 24:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type ScibR = crate::BitReader;
#[doc = "Field `scib` writer - 24:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type ScibW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lin` reader - 28:28\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type LinR = crate::BitReader;
#[doc = "Field `lin` writer - 28:28\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
pub type LinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn can(&self) -> CanR {
        CanR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn rti(&self) -> RtiR {
        RtiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn wdt(&self) -> WdtR {
        WdtR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn mcrc(&self) -> McrcR {
        McrcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn i2c(&self) -> I2cR {
        I2cR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn scia(&self) -> SciaR {
        SciaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn scib(&self) -> ScibR {
        ScibR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    pub fn lin(&self) -> LinR {
        LinR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn can(&mut self) -> CanW<AppssDbgAckCtl0Spec> {
        CanW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn rti(&mut self) -> RtiW<AppssDbgAckCtl0Spec> {
        RtiW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WdtW<AppssDbgAckCtl0Spec> {
        WdtW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn mcrc(&mut self) -> McrcW<AppssDbgAckCtl0Spec> {
        McrcW::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2cW<AppssDbgAckCtl0Spec> {
        I2cW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn scia(&mut self) -> SciaW<AppssDbgAckCtl0Spec> {
        SciaW::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn scib(&mut self) -> ScibW<AppssDbgAckCtl0Spec> {
        ScibW::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Enable Suspend control for the peripheral. 0 :Peripheral not suspended along with processor 1: Peripehal Suspended along with procesor"]
    #[inline(always)]
    #[must_use]
    pub fn lin(&mut self) -> LinW<AppssDbgAckCtl0Spec> {
        LinW::new(self, 28)
    }
}
#[doc = "APPSS_DBG_ACK_CTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_dbg_ack_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_dbg_ack_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssDbgAckCtl0Spec;
impl crate::RegisterSpec for AppssDbgAckCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_dbg_ack_ctl0::R`](R) reader structure"]
impl crate::Readable for AppssDbgAckCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`appss_dbg_ack_ctl0::W`](W) writer structure"]
impl crate::Writable for AppssDbgAckCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_DBG_ACK_CTL0 to value 0"]
impl crate::Resettable for AppssDbgAckCtl0Spec {
    const RESET_VALUE: u32 = 0;
}
