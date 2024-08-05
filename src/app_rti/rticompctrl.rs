#[doc = "Register `RTICOMPCTRL` reader"]
pub type R = crate::R<RticompctrlSpec>;
#[doc = "Register `RTICOMPCTRL` writer"]
pub type W = crate::W<RticompctrlSpec>;
#[doc = "Field `COMP0SEL` reader - 0:0\\]
COMPSEL0: Compare Select 0. This bit determines the counter with which the compare value hold in compare register 0 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Comp0selR = crate::BitReader;
#[doc = "Field `COMP0SEL` writer - 0:0\\]
COMPSEL0: Compare Select 0. This bit determines the counter with which the compare value hold in compare register 0 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Comp0selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED5` reader - 3:1\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - 3:1\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP1SEL` reader - 4:4\\]
COMPSEL1: Compare Select 1. This bit determines the counter with which the compare value hold in compare register 1 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Comp1selR = crate::BitReader;
#[doc = "Field `COMP1SEL` writer - 4:4\\]
COMPSEL1: Compare Select 1. This bit determines the counter with which the compare value hold in compare register 1 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Comp1selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 7:5\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 7:5\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP2SEL` reader - 8:8\\]
COMPSEL2: Compare Select 2. This bit determines the counter with which the compare value hold in compare register 2 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Comp2selR = crate::BitReader;
#[doc = "Field `COMP2SEL` writer - 8:8\\]
COMPSEL2: Compare Select 2. This bit determines the counter with which the compare value hold in compare register 2 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Comp2selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED7` reader - 11:9\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved7R = crate::FieldReader;
#[doc = "Field `RESERVED7` writer - 11:9\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP3SEL` reader - 12:12\\]
COMPSEL3: Compare Select 3. This bit determines the counter with which the compare value hold in compare register 3 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Comp3selR = crate::BitReader;
#[doc = "Field `COMP3SEL` writer - 12:12\\]
COMPSEL3: Compare Select 3. This bit determines the counter with which the compare value hold in compare register 3 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Comp3selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 31:13\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:13\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
COMPSEL0: Compare Select 0. This bit determines the counter with which the compare value hold in compare register 0 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    pub fn comp0sel(&self) -> Comp0selR {
        Comp0selR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
COMPSEL1: Compare Select 1. This bit determines the counter with which the compare value hold in compare register 1 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    pub fn comp1sel(&self) -> Comp1selR {
        Comp1selR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
COMPSEL2: Compare Select 2. This bit determines the counter with which the compare value hold in compare register 2 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    pub fn comp2sel(&self) -> Comp2selR {
        Comp2selR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
COMPSEL3: Compare Select 3. This bit determines the counter with which the compare value hold in compare register 3 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    pub fn comp3sel(&self) -> Comp3selR {
        Comp3selR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
COMPSEL0: Compare Select 0. This bit determines the counter with which the compare value hold in compare register 0 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    #[must_use]
    pub fn comp0sel(&mut self) -> Comp0selW<RticompctrlSpec> {
        Comp0selW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<RticompctrlSpec> {
        Reserved5W::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
COMPSEL1: Compare Select 1. This bit determines the counter with which the compare value hold in compare register 1 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    #[must_use]
    pub fn comp1sel(&mut self) -> Comp1selW<RticompctrlSpec> {
        Comp1selW::new(self, 4)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<RticompctrlSpec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
COMPSEL2: Compare Select 2. This bit determines the counter with which the compare value hold in compare register 2 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    #[must_use]
    pub fn comp2sel(&mut self) -> Comp2selW<RticompctrlSpec> {
        Comp2selW::new(self, 8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<RticompctrlSpec> {
        Reserved7W::new(self, 9)
    }
    #[doc = "Bit 12 - 12:12\\]
COMPSEL3: Compare Select 3. This bit determines the counter with which the compare value hold in compare register 3 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    #[must_use]
    pub fn comp3sel(&mut self) -> Comp3selW<RticompctrlSpec> {
        Comp3selW::new(self, 12)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<RticompctrlSpec> {
        Reserved8W::new(self, 13)
    }
}
#[doc = "Compare Control controls the source for the compare registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rticompctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticompctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RticompctrlSpec;
impl crate::RegisterSpec for RticompctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rticompctrl::R`](R) reader structure"]
impl crate::Readable for RticompctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rticompctrl::W`](W) writer structure"]
impl crate::Writable for RticompctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTICOMPCTRL to value 0"]
impl crate::Resettable for RticompctrlSpec {
    const RESET_VALUE: u32 = 0;
}
