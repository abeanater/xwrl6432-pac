#[doc = "Register `RTIGCTRL` reader"]
pub type R = crate::R<RtigctrlSpec>;
#[doc = "Register `RTIGCTRL` writer"]
pub type W = crate::W<RtigctrlSpec>;
#[doc = "Field `CNT0EN` reader - 0:0\\]
CNT0EN: Counter 0 Enable.The CNT0EN bit starts and stops the operation of counter block 0 (UC0 and FRC0). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters Gives the absolute 32 bits source address (physical)."]
pub type Cnt0enR = crate::BitReader;
#[doc = "Field `CNT0EN` writer - 0:0\\]
CNT0EN: Counter 0 Enable.The CNT0EN bit starts and stops the operation of counter block 0 (UC0 and FRC0). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters Gives the absolute 32 bits source address (physical)."]
pub type Cnt0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT1EN` reader - 1:1\\]
CNT1EN: Counter 1 Enable. The CNT1EN bit starts and stops the operation of counter block 1 (UC1 and FRC1). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters Gives the absolute 32 bit destination address (physical)."]
pub type Cnt1enR = crate::BitReader;
#[doc = "Field `CNT1EN` writer - 1:1\\]
CNT1EN: Counter 1 Enable. The CNT1EN bit starts and stops the operation of counter block 1 (UC1 and FRC1). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters Gives the absolute 32 bit destination address (physical)."]
pub type Cnt1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 14:2\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED1` writer - 14:2\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `COS` reader - 15:15\\]
COS: Continue On Suspend. This bit determines if both counters are stopped when the device goes into debug mode or if they continue counting. User and privilege mode (read): 0 = counters are stopped while in debug mode 1 = counters are running while in debug mode Privilege mode (write): 0 = stop counters in debug mode 1 = continue counting in debug mode"]
pub type CosR = crate::BitReader;
#[doc = "Field `COS` writer - 15:15\\]
COS: Continue On Suspend. This bit determines if both counters are stopped when the device goes into debug mode or if they continue counting. User and privilege mode (read): 0 = counters are stopped while in debug mode 1 = counters are running while in debug mode Privilege mode (write): 0 = stop counters in debug mode 1 = continue counting in debug mode"]
pub type CosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NTUSEL` reader - 19:16\\]
NTUSEL: Select NTU signal. These bits determine which NTU input signal is used as external timebase. There are up to four inputs supported with four valid selection combinations. Any invalid selection value written to the NTUSEL bit-field will result in a TIED LOW being used as the NTU signal. The NTU signal will also be TIED LOW in case of a single-bit flip as it will result in an invalid combination of NTUSEL. User and privilege mode (read): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to ΓÇÿ0ΓÇÖ Privilege mode (write): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to ΓÇÿ0ΓÇÖ"]
pub type NtuselR = crate::FieldReader;
#[doc = "Field `NTUSEL` writer - 19:16\\]
NTUSEL: Select NTU signal. These bits determine which NTU input signal is used as external timebase. There are up to four inputs supported with four valid selection combinations. Any invalid selection value written to the NTUSEL bit-field will result in a TIED LOW being used as the NTU signal. The NTU signal will also be TIED LOW in case of a single-bit flip as it will result in an invalid combination of NTUSEL. User and privilege mode (read): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to ΓÇÿ0ΓÇÖ Privilege mode (write): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to ΓÇÿ0ΓÇÖ"]
pub type NtuselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED2` reader - 31:20\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED2` writer - 31:20\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CNT0EN: Counter 0 Enable.The CNT0EN bit starts and stops the operation of counter block 0 (UC0 and FRC0). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters Gives the absolute 32 bits source address (physical)."]
    #[inline(always)]
    pub fn cnt0en(&self) -> Cnt0enR {
        Cnt0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CNT1EN: Counter 1 Enable. The CNT1EN bit starts and stops the operation of counter block 1 (UC1 and FRC1). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters Gives the absolute 32 bit destination address (physical)."]
    #[inline(always)]
    pub fn cnt1en(&self) -> Cnt1enR {
        Cnt1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:14 - 14:2\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 0x1fff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
COS: Continue On Suspend. This bit determines if both counters are stopped when the device goes into debug mode or if they continue counting. User and privilege mode (read): 0 = counters are stopped while in debug mode 1 = counters are running while in debug mode Privilege mode (write): 0 = stop counters in debug mode 1 = continue counting in debug mode"]
    #[inline(always)]
    pub fn cos(&self) -> CosR {
        CosR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
NTUSEL: Select NTU signal. These bits determine which NTU input signal is used as external timebase. There are up to four inputs supported with four valid selection combinations. Any invalid selection value written to the NTUSEL bit-field will result in a TIED LOW being used as the NTU signal. The NTU signal will also be TIED LOW in case of a single-bit flip as it will result in an invalid combination of NTUSEL. User and privilege mode (read): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to ΓÇÿ0ΓÇÖ Privilege mode (write): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to ΓÇÿ0ΓÇÖ"]
    #[inline(always)]
    pub fn ntusel(&self) -> NtuselR {
        NtuselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CNT0EN: Counter 0 Enable.The CNT0EN bit starts and stops the operation of counter block 0 (UC0 and FRC0). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters Gives the absolute 32 bits source address (physical)."]
    #[inline(always)]
    #[must_use]
    pub fn cnt0en(&mut self) -> Cnt0enW<RtigctrlSpec> {
        Cnt0enW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CNT1EN: Counter 1 Enable. The CNT1EN bit starts and stops the operation of counter block 1 (UC1 and FRC1). User and privilege mode (read): 0 = counters are stopped 1 = counters are running Privilege mode (write): 0 = stop counters 1 = start counters Gives the absolute 32 bit destination address (physical)."]
    #[inline(always)]
    #[must_use]
    pub fn cnt1en(&mut self) -> Cnt1enW<RtigctrlSpec> {
        Cnt1enW::new(self, 1)
    }
    #[doc = "Bits 2:14 - 14:2\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<RtigctrlSpec> {
        Reserved1W::new(self, 2)
    }
    #[doc = "Bit 15 - 15:15\\]
COS: Continue On Suspend. This bit determines if both counters are stopped when the device goes into debug mode or if they continue counting. User and privilege mode (read): 0 = counters are stopped while in debug mode 1 = counters are running while in debug mode Privilege mode (write): 0 = stop counters in debug mode 1 = continue counting in debug mode"]
    #[inline(always)]
    #[must_use]
    pub fn cos(&mut self) -> CosW<RtigctrlSpec> {
        CosW::new(self, 15)
    }
    #[doc = "Bits 16:19 - 19:16\\]
NTUSEL: Select NTU signal. These bits determine which NTU input signal is used as external timebase. There are up to four inputs supported with four valid selection combinations. Any invalid selection value written to the NTUSEL bit-field will result in a TIED LOW being used as the NTU signal. The NTU signal will also be TIED LOW in case of a single-bit flip as it will result in an invalid combination of NTUSEL. User and privilege mode (read): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to ΓÇÿ0ΓÇÖ Privilege mode (write): 0000 = NTU0 0101 = NTU1 1010 = NTU2 1111 = NTU3 other = tied to ΓÇÿ0ΓÇÖ"]
    #[inline(always)]
    #[must_use]
    pub fn ntusel(&mut self) -> NtuselW<RtigctrlSpec> {
        NtuselW::new(self, 16)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<RtigctrlSpec> {
        Reserved2W::new(self, 20)
    }
}
#[doc = "Global Control Register starts / stops the counters\n\nYou can [`read`](crate::Reg::read) this register and get [`rtigctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtigctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtigctrlSpec;
impl crate::RegisterSpec for RtigctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtigctrl::R`](R) reader structure"]
impl crate::Readable for RtigctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtigctrl::W`](W) writer structure"]
impl crate::Writable for RtigctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIGCTRL to value 0"]
impl crate::Resettable for RtigctrlSpec {
    const RESET_VALUE: u32 = 0;
}
