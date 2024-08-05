#[doc = "Register `RTISETINT` reader"]
pub type R = crate::R<RtisetintSpec>;
#[doc = "Register `RTISETINT` writer"]
pub type W = crate::W<RtisetintSpec>;
#[doc = "Field `SETINT0` reader - 0:0\\]
SETINT0: Set Compare Interrupt 0. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setint0R = crate::BitReader;
#[doc = "Field `SETINT0` writer - 0:0\\]
SETINT0: Set Compare Interrupt 0. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setint0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETINT1` reader - 1:1\\]
SETINT1: Set Compare Interrupt 1. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setint1R = crate::BitReader;
#[doc = "Field `SETINT1` writer - 1:1\\]
SETINT1: Set Compare Interrupt 1. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setint1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETINT2` reader - 2:2\\]
SETINT2: Set Compare Interrupt 2. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setint2R = crate::BitReader;
#[doc = "Field `SETINT2` writer - 2:2\\]
SETINT2: Set Compare Interrupt 2. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setint2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETINT3` reader - 3:3\\]
SETINT3: Set Compare Interrupt 3. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged"]
pub type Setint3R = crate::BitReader;
#[doc = "Field `SETINT3` writer - 3:3\\]
SETINT3: Set Compare Interrupt 3. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged"]
pub type Setint3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 7:4\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `RESERVED9` writer - 7:4\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SETDMA0` reader - 8:8\\]
SETDMA0: Set Compare DMA Request 0. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma0R = crate::BitReader;
#[doc = "Field `SETDMA0` writer - 8:8\\]
SETDMA0: Set Compare DMA Request 0. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETDMA1` reader - 9:9\\]
SETDMA1: Set Compare DMA Request 1. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma1R = crate::BitReader;
#[doc = "Field `SETDMA1` writer - 9:9\\]
SETDMA1: Set Compare DMA Request 1. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETDMA2` reader - 10:10\\]
SETDMA2: Set Compare DMA Request 2. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma2R = crate::BitReader;
#[doc = "Field `SETDMA2` writer - 10:10\\]
SETDMA2: Set Compare DMA Request 2. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETDMA3` reader - 11:11\\]
SETDMA3: Set Compare DMA Request 3. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma3R = crate::BitReader;
#[doc = "Field `SETDMA3` writer - 11:11\\]
SETDMA3: Set Compare DMA Request 3. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED10` reader - 15:12\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved10R = crate::FieldReader;
#[doc = "Field `RESERVED10` writer - 15:12\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SETTBINT` reader - 16:16\\]
SETTBINT: Set Timebase Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SettbintR = crate::BitReader;
#[doc = "Field `SETTBINT` writer - 16:16\\]
SETTBINT: Set Timebase Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SettbintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETOVL0INT` reader - 17:17\\]
SETOVL0INT: Set Free Running Counter 0 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setovl0intR = crate::BitReader;
#[doc = "Field `SETOVL0INT` writer - 17:17\\]
SETOVL0INT: Set Free Running Counter 0 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setovl0intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETOVL1INT` reader - 18:18\\]
SETOVL1INT: Set Free Running Counter 1 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setovl1intR = crate::BitReader;
#[doc = "Field `SETOVL1INT` writer - 18:18\\]
SETOVL1INT: Set Free Running Counter 1 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setovl1intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED11` reader - 31:19\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved11R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED11` writer - 31:19\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SETINT0: Set Compare Interrupt 0. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn setint0(&self) -> Setint0R {
        Setint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SETINT1: Set Compare Interrupt 1. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn setint1(&self) -> Setint1R {
        Setint1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
SETINT2: Set Compare Interrupt 2. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn setint2(&self) -> Setint2R {
        Setint2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
SETINT3: Set Compare Interrupt 3. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged"]
    #[inline(always)]
    pub fn setint3(&self) -> Setint3R {
        Setint3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
SETDMA0: Set Compare DMA Request 0. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    pub fn setdma0(&self) -> Setdma0R {
        Setdma0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
SETDMA1: Set Compare DMA Request 1. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    pub fn setdma1(&self) -> Setdma1R {
        Setdma1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
SETDMA2: Set Compare DMA Request 2. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    pub fn setdma2(&self) -> Setdma2R {
        Setdma2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
SETDMA3: Set Compare DMA Request 3. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    pub fn setdma3(&self) -> Setdma3R {
        Setdma3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
SETTBINT: Set Timebase Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn settbint(&self) -> SettbintR {
        SettbintR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
SETOVL0INT: Set Free Running Counter 0 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn setovl0int(&self) -> Setovl0intR {
        Setovl0intR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
SETOVL1INT: Set Free Running Counter 1 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn setovl1int(&self) -> Setovl1intR {
        Setovl1intR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SETINT0: Set Compare Interrupt 0. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn setint0(&mut self) -> Setint0W<RtisetintSpec> {
        Setint0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SETINT1: Set Compare Interrupt 1. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn setint1(&mut self) -> Setint1W<RtisetintSpec> {
        Setint1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
SETINT2: Set Compare Interrupt 2. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn setint2(&mut self) -> Setint2W<RtisetintSpec> {
        Setint2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
SETINT3: Set Compare Interrupt 3. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged"]
    #[inline(always)]
    #[must_use]
    pub fn setint3(&mut self) -> Setint3W<RtisetintSpec> {
        Setint3W::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<RtisetintSpec> {
        Reserved9W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
SETDMA0: Set Compare DMA Request 0. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn setdma0(&mut self) -> Setdma0W<RtisetintSpec> {
        Setdma0W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
SETDMA1: Set Compare DMA Request 1. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn setdma1(&mut self) -> Setdma1W<RtisetintSpec> {
        Setdma1W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
SETDMA2: Set Compare DMA Request 2. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn setdma2(&mut self) -> Setdma2W<RtisetintSpec> {
        Setdma2W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
SETDMA3: Set Compare DMA Request 3. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn setdma3(&mut self) -> Setdma3W<RtisetintSpec> {
        Setdma3W::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<RtisetintSpec> {
        Reserved10W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
SETTBINT: Set Timebase Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn settbint(&mut self) -> SettbintW<RtisetintSpec> {
        SettbintW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
SETOVL0INT: Set Free Running Counter 0 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn setovl0int(&mut self) -> Setovl0intW<RtisetintSpec> {
        Setovl0intW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
SETOVL1INT: Set Free Running Counter 1 Overflow Interrupt. User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn setovl1int(&mut self) -> Setovl1intW<RtisetintSpec> {
        Setovl1intW::new(self, 18)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<RtisetintSpec> {
        Reserved11W::new(self, 19)
    }
}
#[doc = "Set Interrupt Enable sets interrupt enable bits int RTIINTCTRL without having to do a read-modify-write operation\n\nYou can [`read`](crate::Reg::read) this register and get [`rtisetint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtisetint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtisetintSpec;
impl crate::RegisterSpec for RtisetintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtisetint::R`](R) reader structure"]
impl crate::Readable for RtisetintSpec {}
#[doc = "`write(|w| ..)` method takes [`rtisetint::W`](W) writer structure"]
impl crate::Writable for RtisetintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTISETINT to value 0"]
impl crate::Resettable for RtisetintSpec {
    const RESET_VALUE: u32 = 0;
}
