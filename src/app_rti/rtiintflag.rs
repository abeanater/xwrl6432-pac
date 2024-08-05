#[doc = "Register `RTIINTFLAG` reader"]
pub type R = crate::R<RtiintflagSpec>;
#[doc = "Register `RTIINTFLAG` writer"]
pub type W = crate::W<RtiintflagSpec>;
#[doc = "Field `INT0` reader - 0:0\\]
INT0: Interrupt Flag 0. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int0R = crate::BitReader;
#[doc = "Field `INT0` writer - 0:0\\]
INT0: Interrupt Flag 0. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1` reader - 1:1\\]
INT1: Interrupt Flag 1. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int1R = crate::BitReader;
#[doc = "Field `INT1` writer - 1:1\\]
INT1: Interrupt Flag 1. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2` reader - 2:2\\]
INT2: Interrupt Flag 2. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int2R = crate::BitReader;
#[doc = "Field `INT2` writer - 2:2\\]
INT2: Interrupt Flag 2. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT3` reader - 3:3\\]
INT3: Interrupt Flag 3. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int3R = crate::BitReader;
#[doc = "Field `INT3` writer - 3:3\\]
INT3: Interrupt Flag 3. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED15` reader - 15:4\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved15R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED15` writer - 15:4\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved15W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TBINT` reader - 16:16\\]
User and privilege mode (read): this flag is set when the TBEXT bit is cleared by detection of a missing external clockedge. It will not be set by clearing TBEXT by software. determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type TbintR = crate::BitReader;
#[doc = "Field `TBINT` writer - 16:16\\]
User and privilege mode (read): this flag is set when the TBEXT bit is cleared by detection of a missing external clockedge. It will not be set by clearing TBEXT by software. determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type TbintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVL0INT` reader - 17:17\\]
OVL0INT: Free Running Counter 0 Overflow Interrupt Flag. User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Ovl0intR = crate::BitReader;
#[doc = "Field `OVL0INT` writer - 17:17\\]
OVL0INT: Free Running Counter 0 Overflow Interrupt Flag. User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Ovl0intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVL1INT` reader - 18:18\\]
OVL1INT: Free Running Counter 1 Overflow Interrupt Flag. User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Ovl1intR = crate::BitReader;
#[doc = "Field `OVL1INT` writer - 18:18\\]
OVL1INT: Free Running Counter 1 Overflow Interrupt Flag. User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Ovl1intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED16` reader - 31:19\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:19\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
INT0: Interrupt Flag 0. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
INT1: Interrupt Flag 1. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
INT2: Interrupt Flag 2. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    pub fn int2(&self) -> Int2R {
        Int2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
INT3: Interrupt Flag 3. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    pub fn int3(&self) -> Int3R {
        Int3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
User and privilege mode (read): this flag is set when the TBEXT bit is cleared by detection of a missing external clockedge. It will not be set by clearing TBEXT by software. determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    pub fn tbint(&self) -> TbintR {
        TbintR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
OVL0INT: Free Running Counter 0 Overflow Interrupt Flag. User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    pub fn ovl0int(&self) -> Ovl0intR {
        Ovl0intR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
OVL1INT: Free Running Counter 1 Overflow Interrupt Flag. User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    pub fn ovl1int(&self) -> Ovl1intR {
        Ovl1intR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
INT0: Interrupt Flag 0. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> Int0W<RtiintflagSpec> {
        Int0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
INT1: Interrupt Flag 1. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> Int1W<RtiintflagSpec> {
        Int1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
INT2: Interrupt Flag 2. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> Int2W<RtiintflagSpec> {
        Int2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
INT3: Interrupt Flag 3. User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> Int3W<RtiintflagSpec> {
        Int3W::new(self, 3)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<RtiintflagSpec> {
        Reserved15W::new(self, 4)
    }
    #[doc = "Bit 16 - 16:16\\]
User and privilege mode (read): this flag is set when the TBEXT bit is cleared by detection of a missing external clockedge. It will not be set by clearing TBEXT by software. determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn tbint(&mut self) -> TbintW<RtiintflagSpec> {
        TbintW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
OVL0INT: Free Running Counter 0 Overflow Interrupt Flag. User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn ovl0int(&mut self) -> Ovl0intW<RtiintflagSpec> {
        Ovl0intW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
OVL1INT: Free Running Counter 1 Overflow Interrupt Flag. User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn ovl1int(&mut self) -> Ovl1intW<RtiintflagSpec> {
        Ovl1intW::new(self, 18)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<RtiintflagSpec> {
        Reserved16W::new(self, 19)
    }
}
#[doc = "Interrupt Flags interrupt pending bits\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiintflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiintflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtiintflagSpec;
impl crate::RegisterSpec for RtiintflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtiintflag::R`](R) reader structure"]
impl crate::Readable for RtiintflagSpec {}
#[doc = "`write(|w| ..)` method takes [`rtiintflag::W`](W) writer structure"]
impl crate::Writable for RtiintflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIINTFLAG to value 0"]
impl crate::Resettable for RtiintflagSpec {
    const RESET_VALUE: u32 = 0;
}
