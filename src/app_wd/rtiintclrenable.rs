#[doc = "Register `RTIINTCLRENABLE` reader"]
pub type R = crate::R<RtiintclrenableSpec>;
#[doc = "Register `RTIINTCLRENABLE` writer"]
pub type W = crate::W<RtiintclrenableSpec>;
#[doc = "Field `INTCLRENABLE0` reader - 3:0\\]
INTCLRENABLE0. Enables the auto-clear functionality on the compare 0 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 0 interrupt is disabled. Any other value = Auto-clear for compare 0 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 0 interrupt. Any other value = Enables the auto-clear functionality on the compare 0 interrupt."]
pub type Intclrenable0R = crate::FieldReader;
#[doc = "Field `INTCLRENABLE0` writer - 3:0\\]
INTCLRENABLE0. Enables the auto-clear functionality on the compare 0 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 0 interrupt is disabled. Any other value = Auto-clear for compare 0 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 0 interrupt. Any other value = Enables the auto-clear functionality on the compare 0 interrupt."]
pub type Intclrenable0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED22` reader - 7:4\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved22R = crate::FieldReader;
#[doc = "Field `RESERVED22` writer - 7:4\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved22W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTCLRENABLE1` reader - 11:8\\]
INTCLRENABLE1. Enables the auto-clear functionality on the compare 1 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 1 interrupt is disabled. Any other value = Auto-clear for compare 1 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 1 interrupt. Any other value = Enables the auto-clear functionality on the compare 1 interrupt."]
pub type Intclrenable1R = crate::FieldReader;
#[doc = "Field `INTCLRENABLE1` writer - 11:8\\]
INTCLRENABLE1. Enables the auto-clear functionality on the compare 1 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 1 interrupt is disabled. Any other value = Auto-clear for compare 1 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 1 interrupt. Any other value = Enables the auto-clear functionality on the compare 1 interrupt."]
pub type Intclrenable1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED23` reader - 15:12\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved23R = crate::FieldReader;
#[doc = "Field `RESERVED23` writer - 15:12\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved23W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTCLRENABLE2` reader - 19:16\\]
INTCLRENABLE2. Enables the auto-clear functionality on the compare 2 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 2 interrupt is disabled. Any other value = Auto-clear for compare 2 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 2 interrupt. Any other value = Enables the auto-clear functionality on the compare 2 interrupt."]
pub type Intclrenable2R = crate::FieldReader;
#[doc = "Field `INTCLRENABLE2` writer - 19:16\\]
INTCLRENABLE2. Enables the auto-clear functionality on the compare 2 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 2 interrupt is disabled. Any other value = Auto-clear for compare 2 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 2 interrupt. Any other value = Enables the auto-clear functionality on the compare 2 interrupt."]
pub type Intclrenable2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED24` reader - 23:20\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 23:20\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTCLRENABLE3` reader - 27:24\\]
INTCLRENABLE3. Enables the auto-clear functionality on the compare 3 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 3 interrupt is disabled. Any other value = Auto-clear for compare 3 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 3 interrupt. Any other value = Enables the auto-clear functionality on the compare 3 interrupt."]
pub type Intclrenable3R = crate::FieldReader;
#[doc = "Field `INTCLRENABLE3` writer - 27:24\\]
INTCLRENABLE3. Enables the auto-clear functionality on the compare 3 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 3 interrupt is disabled. Any other value = Auto-clear for compare 3 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 3 interrupt. Any other value = Enables the auto-clear functionality on the compare 3 interrupt."]
pub type Intclrenable3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED25` reader - 31:28\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 31:28\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
INTCLRENABLE0. Enables the auto-clear functionality on the compare 0 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 0 interrupt is disabled. Any other value = Auto-clear for compare 0 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 0 interrupt. Any other value = Enables the auto-clear functionality on the compare 0 interrupt."]
    #[inline(always)]
    pub fn intclrenable0(&self) -> Intclrenable0R {
        Intclrenable0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved22(&self) -> Reserved22R {
        Reserved22R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
INTCLRENABLE1. Enables the auto-clear functionality on the compare 1 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 1 interrupt is disabled. Any other value = Auto-clear for compare 1 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 1 interrupt. Any other value = Enables the auto-clear functionality on the compare 1 interrupt."]
    #[inline(always)]
    pub fn intclrenable1(&self) -> Intclrenable1R {
        Intclrenable1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved23(&self) -> Reserved23R {
        Reserved23R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
INTCLRENABLE2. Enables the auto-clear functionality on the compare 2 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 2 interrupt is disabled. Any other value = Auto-clear for compare 2 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 2 interrupt. Any other value = Enables the auto-clear functionality on the compare 2 interrupt."]
    #[inline(always)]
    pub fn intclrenable2(&self) -> Intclrenable2R {
        Intclrenable2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
INTCLRENABLE3. Enables the auto-clear functionality on the compare 3 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 3 interrupt is disabled. Any other value = Auto-clear for compare 3 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 3 interrupt. Any other value = Enables the auto-clear functionality on the compare 3 interrupt."]
    #[inline(always)]
    pub fn intclrenable3(&self) -> Intclrenable3R {
        Intclrenable3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
INTCLRENABLE0. Enables the auto-clear functionality on the compare 0 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 0 interrupt is disabled. Any other value = Auto-clear for compare 0 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 0 interrupt. Any other value = Enables the auto-clear functionality on the compare 0 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn intclrenable0(&mut self) -> Intclrenable0W<RtiintclrenableSpec> {
        Intclrenable0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved22(&mut self) -> Reserved22W<RtiintclrenableSpec> {
        Reserved22W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
INTCLRENABLE1. Enables the auto-clear functionality on the compare 1 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 1 interrupt is disabled. Any other value = Auto-clear for compare 1 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 1 interrupt. Any other value = Enables the auto-clear functionality on the compare 1 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn intclrenable1(&mut self) -> Intclrenable1W<RtiintclrenableSpec> {
        Intclrenable1W::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved23(&mut self) -> Reserved23W<RtiintclrenableSpec> {
        Reserved23W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
INTCLRENABLE2. Enables the auto-clear functionality on the compare 2 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 2 interrupt is disabled. Any other value = Auto-clear for compare 2 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 2 interrupt. Any other value = Enables the auto-clear functionality on the compare 2 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn intclrenable2(&mut self) -> Intclrenable2W<RtiintclrenableSpec> {
        Intclrenable2W::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<RtiintclrenableSpec> {
        Reserved24W::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
INTCLRENABLE3. Enables the auto-clear functionality on the compare 3 interrupt. User and Privileged mode (read): 0x5 = Auto-clear for compare 3 interrupt is disabled. Any other value = Auto-clear for compare 3 interrupt is enabled. Privileged mode (write): 0x5 = Disables the auto-clear functionality on the compare 3 interrupt. Any other value = Enables the auto-clear functionality on the compare 3 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn intclrenable3(&mut self) -> Intclrenable3W<RtiintclrenableSpec> {
        Intclrenable3W::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> Reserved25W<RtiintclrenableSpec> {
        Reserved25W::new(self, 28)
    }
}
#[doc = "RTI Compare Interrupt Clear Enable enable the auto clear functionality for each of the compare interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiintclrenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiintclrenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtiintclrenableSpec;
impl crate::RegisterSpec for RtiintclrenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtiintclrenable::R`](R) reader structure"]
impl crate::Readable for RtiintclrenableSpec {}
#[doc = "`write(|w| ..)` method takes [`rtiintclrenable::W`](W) writer structure"]
impl crate::Writable for RtiintclrenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIINTCLRENABLE to value 0"]
impl crate::Resettable for RtiintclrenableSpec {
    const RESET_VALUE: u32 = 0;
}
