#[doc = "Register `RTICLEARINT` reader"]
pub type R = crate::R<RticlearintSpec>;
#[doc = "Register `RTICLEARINT` writer"]
pub type W = crate::W<RticlearintSpec>;
#[doc = "Field `CLEARINT0` reader - 0:0\\]
CLEARINT0: CLEAR Compare Interrupt 0. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint0R = crate::BitReader;
#[doc = "Field `CLEARINT0` writer - 0:0\\]
CLEARINT0: CLEAR Compare Interrupt 0. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARINT1` reader - 1:1\\]
CLEARINT1: CLEAR Compare Interrupt 1. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint1R = crate::BitReader;
#[doc = "Field `CLEARINT1` writer - 1:1\\]
CLEARINT1: CLEAR Compare Interrupt 1. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARINT2` reader - 2:2\\]
CLEARINT2: CLEAR Compare Interrupt 2. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint2R = crate::BitReader;
#[doc = "Field `CLEARINT2` writer - 2:2\\]
CLEARINT2: CLEAR Compare Interrupt 2. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARINT3` reader - 3:3\\]
CLEARINT3: CLEAR Compare Interrupt 3. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint3R = crate::BitReader;
#[doc = "Field `CLEARINT3` writer - 3:3\\]
CLEARINT3: CLEAR Compare Interrupt 3. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED12` reader - 7:4\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved12R = crate::FieldReader;
#[doc = "Field `RESERVED12` writer - 7:4\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLEARDMA0` reader - 8:8\\]
CLEARDMA0: CLEAR Compare DMA Request 0. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma0R = crate::BitReader;
#[doc = "Field `CLEARDMA0` writer - 8:8\\]
CLEARDMA0: CLEAR Compare DMA Request 0. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARDMA1` reader - 9:9\\]
CLEARDMA1: CLEAR Compare DMA Request 1. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma1R = crate::BitReader;
#[doc = "Field `CLEARDMA1` writer - 9:9\\]
CLEARDMA1: CLEAR Compare DMA Request 1. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARDMA2` reader - 10:10\\]
CLEARDMA2: CLEAR Compare DMA Request 2. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma2R = crate::BitReader;
#[doc = "Field `CLEARDMA2` writer - 10:10\\]
CLEARDMA2: CLEAR Compare DMA Request 2. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARDMA3` reader - 11:11\\]
CLEARDMA3: CLEAR Compare DMA Request 3. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma3R = crate::BitReader;
#[doc = "Field `CLEARDMA3` writer - 11:11\\]
CLEARDMA3: CLEAR Compare DMA Request 3. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED13` reader - 15:12\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved13R = crate::FieldReader;
#[doc = "Field `RESERVED13` writer - 15:12\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLEARTBINT` reader - 16:16\\]
CLEARTBINT: CLEAR Timebase Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type CleartbintR = crate::BitReader;
#[doc = "Field `CLEARTBINT` writer - 16:16\\]
CLEARTBINT: CLEAR Timebase Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type CleartbintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAROVL0INT` reader - 17:17\\]
CLEAROVL0INT: CLEAR Free Running Counter 0 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearovl0intR = crate::BitReader;
#[doc = "Field `CLEAROVL0INT` writer - 17:17\\]
CLEAROVL0INT: CLEAR Free Running Counter 0 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearovl0intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAROVL1INT` reader - 18:18\\]
CLEAROVL1INT: CLEAR Free Running Counter 1 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearovl1intR = crate::BitReader;
#[doc = "Field `CLEAROVL1INT` writer - 18:18\\]
CLEAROVL1INT: CLEAR Free Running Counter 1 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearovl1intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED14` reader - 31:19\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved14R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED14` writer - 31:19\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved14W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CLEARINT0: CLEAR Compare Interrupt 0. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clearint0(&self) -> Clearint0R {
        Clearint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CLEARINT1: CLEAR Compare Interrupt 1. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clearint1(&self) -> Clearint1R {
        Clearint1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CLEARINT2: CLEAR Compare Interrupt 2. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clearint2(&self) -> Clearint2R {
        Clearint2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
CLEARINT3: CLEAR Compare Interrupt 3. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clearint3(&self) -> Clearint3R {
        Clearint3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
CLEARDMA0: CLEAR Compare DMA Request 0. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    pub fn cleardma0(&self) -> Cleardma0R {
        Cleardma0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
CLEARDMA1: CLEAR Compare DMA Request 1. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    pub fn cleardma1(&self) -> Cleardma1R {
        Cleardma1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
CLEARDMA2: CLEAR Compare DMA Request 2. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    pub fn cleardma2(&self) -> Cleardma2R {
        Cleardma2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
CLEARDMA3: CLEAR Compare DMA Request 3. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    pub fn cleardma3(&self) -> Cleardma3R {
        Cleardma3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
CLEARTBINT: CLEAR Timebase Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn cleartbint(&self) -> CleartbintR {
        CleartbintR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
CLEAROVL0INT: CLEAR Free Running Counter 0 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clearovl0int(&self) -> Clearovl0intR {
        Clearovl0intR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
CLEAROVL1INT: CLEAR Free Running Counter 1 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clearovl1int(&self) -> Clearovl1intR {
        Clearovl1intR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CLEARINT0: CLEAR Compare Interrupt 0. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clearint0(&mut self) -> Clearint0W<RticlearintSpec> {
        Clearint0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CLEARINT1: CLEAR Compare Interrupt 1. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clearint1(&mut self) -> Clearint1W<RticlearintSpec> {
        Clearint1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CLEARINT2: CLEAR Compare Interrupt 2. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clearint2(&mut self) -> Clearint2W<RticlearintSpec> {
        Clearint2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
CLEARINT3: CLEAR Compare Interrupt 3. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clearint3(&mut self) -> Clearint3W<RticlearintSpec> {
        Clearint3W::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<RticlearintSpec> {
        Reserved12W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
CLEARDMA0: CLEAR Compare DMA Request 0. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn cleardma0(&mut self) -> Cleardma0W<RticlearintSpec> {
        Cleardma0W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
CLEARDMA1: CLEAR Compare DMA Request 1. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn cleardma1(&mut self) -> Cleardma1W<RticlearintSpec> {
        Cleardma1W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
CLEARDMA2: CLEAR Compare DMA Request 2. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn cleardma2(&mut self) -> Cleardma2W<RticlearintSpec> {
        Cleardma2W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
CLEARDMA3: CLEAR Compare DMA Request 3. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn cleardma3(&mut self) -> Cleardma3W<RticlearintSpec> {
        Cleardma3W::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> Reserved13W<RticlearintSpec> {
        Reserved13W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
CLEARTBINT: CLEAR Timebase Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cleartbint(&mut self) -> CleartbintW<RticlearintSpec> {
        CleartbintW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
CLEAROVL0INT: CLEAR Free Running Counter 0 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clearovl0int(&mut self) -> Clearovl0intW<RticlearintSpec> {
        Clearovl0intW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
CLEAROVL1INT: CLEAR Free Running Counter 1 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clearovl1int(&mut self) -> Clearovl1intW<RticlearintSpec> {
        Clearovl1intW::new(self, 18)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<RticlearintSpec> {
        Reserved14W::new(self, 19)
    }
}
#[doc = "Clear Interrupt Enable clears interrupt enable bits int RTIINTCTRL without having to do a read-modify-write operation\n\nYou can [`read`](crate::Reg::read) this register and get [`rticlearint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticlearint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RticlearintSpec;
impl crate::RegisterSpec for RticlearintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rticlearint::R`](R) reader structure"]
impl crate::Readable for RticlearintSpec {}
#[doc = "`write(|w| ..)` method takes [`rticlearint::W`](W) writer structure"]
impl crate::Writable for RticlearintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTICLEARINT to value 0"]
impl crate::Resettable for RticlearintSpec {
    const RESET_VALUE: u32 = 0;
}
