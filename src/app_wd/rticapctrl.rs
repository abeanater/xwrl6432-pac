#[doc = "Register `RTICAPCTRL` reader"]
pub type R = crate::R<RticapctrlSpec>;
#[doc = "Register `RTICAPCTRL` writer"]
pub type W = crate::W<RticapctrlSpec>;
#[doc = "Field `CAPCNTR0` reader - 0:0\\]
CAPCNTR0: Capture Counter 0. This bit determines, which external interrupt source triggers a capture event of both UC0 and FRC0. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1 11 indexed 10 reserved 01 post-increment 00 constant"]
pub type Capcntr0R = crate::BitReader;
#[doc = "Field `CAPCNTR0` writer - 0:0\\]
CAPCNTR0: Capture Counter 0. This bit determines, which external interrupt source triggers a capture event of both UC0 and FRC0. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1 11 indexed 10 reserved 01 post-increment 00 constant"]
pub type Capcntr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCNTR1` reader - 1:1\\]
CAPCNTR1: Capture Counter 1. This bit determines, which external interrupt source triggers a capture event of both UC1 and FRC1. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1"]
pub type Capcntr1R = crate::BitReader;
#[doc = "Field `CAPCNTR1` writer - 1:1\\]
CAPCNTR1: Capture Counter 1. This bit determines, which external interrupt source triggers a capture event of both UC1 and FRC1. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1"]
pub type Capcntr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 31:2\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:2\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CAPCNTR0: Capture Counter 0. This bit determines, which external interrupt source triggers a capture event of both UC0 and FRC0. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1 11 indexed 10 reserved 01 post-increment 00 constant"]
    #[inline(always)]
    pub fn capcntr0(&self) -> Capcntr0R {
        Capcntr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CAPCNTR1: Capture Counter 1. This bit determines, which external interrupt source triggers a capture event of both UC1 and FRC1. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1"]
    #[inline(always)]
    pub fn capcntr1(&self) -> Capcntr1R {
        Capcntr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CAPCNTR0: Capture Counter 0. This bit determines, which external interrupt source triggers a capture event of both UC0 and FRC0. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1 11 indexed 10 reserved 01 post-increment 00 constant"]
    #[inline(always)]
    #[must_use]
    pub fn capcntr0(&mut self) -> Capcntr0W<RticapctrlSpec> {
        Capcntr0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CAPCNTR1: Capture Counter 1. This bit determines, which external interrupt source triggers a capture event of both UC1 and FRC1. User and privilege mode (read): 0 = capture event is triggered by Capture Event Source 0 1 = capture event is triggered by Capture Event Source 1 Privilege mode (write): 0 = enable capture event triggered by Capture Event Source 0 1 = enable capture event triggered by Capture Event Source 1"]
    #[inline(always)]
    #[must_use]
    pub fn capcntr1(&mut self) -> Capcntr1W<RticapctrlSpec> {
        Capcntr1W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<RticapctrlSpec> {
        Reserved4W::new(self, 2)
    }
}
#[doc = "Capture Control controls the capture source for the counters\n\nYou can [`read`](crate::Reg::read) this register and get [`rticapctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticapctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RticapctrlSpec;
impl crate::RegisterSpec for RticapctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rticapctrl::R`](R) reader structure"]
impl crate::Readable for RticapctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rticapctrl::W`](W) writer structure"]
impl crate::Writable for RticapctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTICAPCTRL to value 0"]
impl crate::Resettable for RticapctrlSpec {
    const RESET_VALUE: u32 = 0;
}
