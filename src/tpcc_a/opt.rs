#[doc = "Register `OPT` reader"]
pub type R = crate::R<OptSpec>;
#[doc = "Register `OPT` writer"]
pub type W = crate::W<OptSpec>;
#[doc = "Field `SAM` reader - 0:0\\]
Source Address Mode: Source Address Mode within an array. Pass-thru to TC. 0: INCR Src addressing within an array increments. Source is not a FIFO. 1: FIFO Src addressing within an array wraps around upon reaching FIFO width."]
pub type SamR = crate::BitReader;
#[doc = "Field `SAM` writer - 0:0\\]
Source Address Mode: Source Address Mode within an array. Pass-thru to TC. 0: INCR Src addressing within an array increments. Source is not a FIFO. 1: FIFO Src addressing within an array wraps around upon reaching FIFO width."]
pub type SamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAM` reader - 1:1\\]
Destination Address Mode: Destination Address Mode within an array. Pass-thru to TC. 0: INCR Dst addressing within an array increments. Dst is not a FIFO. 1: FIFO Dst addressing within an array wraps around upon reaching FIFO width."]
pub type DamR = crate::BitReader;
#[doc = "Field `DAM` writer - 1:1\\]
Destination Address Mode: Destination Address Mode within an array. Pass-thru to TC. 0: INCR Dst addressing within an array increments. Dst is not a FIFO. 1: FIFO Dst addressing within an array wraps around upon reaching FIFO width."]
pub type DamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDIM` reader - 2:2\\]
Transfer Synchronization Dimension: 0: A-Sync Each event triggers the transfer of ACNT elements. 1: AB-Sync Each event triggers the transfer of BCNT arrays of ACNT elements"]
pub type SyncdimR = crate::BitReader;
#[doc = "Field `SYNCDIM` writer - 2:2\\]
Transfer Synchronization Dimension: 0: A-Sync Each event triggers the transfer of ACNT elements. 1: AB-Sync Each event triggers the transfer of BCNT arrays of ACNT elements"]
pub type SyncdimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATIC` reader - 3:3\\]
Static Entry: 0: Entry is updated as normal 1: Entry is static Count and Address updates are not updated after TRP is submitted. Linking is not performed."]
pub type StaticR = crate::BitReader;
#[doc = "Field `STATIC` writer - 3:3\\]
Static Entry: 0: Entry is updated as normal 1: Entry is static Count and Address updates are not updated after TRP is submitted. Linking is not performed."]
pub type StaticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES85` reader - 7:4\\]
RESERVE FIELD"]
pub type Res85R = crate::FieldReader;
#[doc = "Field `RES85` writer - 7:4\\]
RESERVE FIELD"]
pub type Res85W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FWID` reader - 10:8\\]
FIFO width: Applies if either SAM or DAM is set to FIFO mode. Pass-thru to TC."]
pub type FwidR = crate::FieldReader;
#[doc = "Field `FWID` writer - 10:8\\]
FIFO width: Applies if either SAM or DAM is set to FIFO mode. Pass-thru to TC."]
pub type FwidW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TCCMODE` reader - 11:11\\]
Transfer complete code mode: Indicates the point at which a transfer is considered completed. Applies to both chaining and interrupt. 0: Normal Completion A transfer is considered completed after the transfer parameters are returned to the CC from the TC (which was returned from the peripheral). 1: Early Completion A transfer is considered completed after the CC submits a TR to the TC. CC generates completion code internally ."]
pub type TccmodeR = crate::BitReader;
#[doc = "Field `TCCMODE` writer - 11:11\\]
Transfer complete code mode: Indicates the point at which a transfer is considered completed. Applies to both chaining and interrupt. 0: Normal Completion A transfer is considered completed after the transfer parameters are returned to the CC from the TC (which was returned from the peripheral). 1: Early Completion A transfer is considered completed after the CC submits a TR to the TC. CC generates completion code internally ."]
pub type TccmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` reader - 17:12\\]
Transfer Complete Code: The 6-bit code is used to set the relevant bit in CER (bit CER\\[TCC\\]) for chaining or in IER (bit IER\\[TCC\\]) for interrupts."]
pub type TccR = crate::FieldReader;
#[doc = "Field `TCC` writer - 17:12\\]
Transfer Complete Code: The 6-bit code is used to set the relevant bit in CER (bit CER\\[TCC\\]) for chaining or in IER (bit IER\\[TCC\\]) for interrupts."]
pub type TccW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RES84` reader - 18:18\\]
RESERVE FIELD"]
pub type Res84R = crate::BitReader;
#[doc = "Field `RES84` writer - 18:18\\]
RESERVE FIELD"]
pub type Res84W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIMODE` reader - 19:19\\]
Backward compatibility mode: 0: Normal operation 1 : WI Backwards Compatibility mode forces BCNT to be adjusted by '1' upon TR submission (0 means 1 1 means 2 ... ) and forces ACNT to be treated as a word-count (left shifted by 2 by hardware to create byte cnt for TR submission)"]
pub type WimodeR = crate::BitReader;
#[doc = "Field `WIMODE` writer - 19:19\\]
Backward compatibility mode: 0: Normal operation 1 : WI Backwards Compatibility mode forces BCNT to be adjusted by '1' upon TR submission (0 means 1 1 means 2 ... ) and forces ACNT to be treated as a word-count (left shifted by 2 by hardware to create byte cnt for TR submission)"]
pub type WimodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCINTEN` reader - 20:20\\]
Transfer complete interrupt enable: 0: Transfer complete interrupt is disabled. 1: Transfer complete interrupt is enabled (corresponding IER\\[TCC\\]
bit must be set to 1 to generate interrupt)"]
pub type TcintenR = crate::BitReader;
#[doc = "Field `TCINTEN` writer - 20:20\\]
Transfer complete interrupt enable: 0: Transfer complete interrupt is disabled. 1: Transfer complete interrupt is enabled (corresponding IER\\[TCC\\]
bit must be set to 1 to generate interrupt)"]
pub type TcintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITCINTEN` reader - 21:21\\]
Intermediate transfer completion interrupt enable: 0: Intermediate transfer complete interrupt is disabled. 1: Intermediate transfer complete interrupt is enabled (corresponding IER\\[TCC\\]
bit must be set to 1 to generate interrupt)"]
pub type ItcintenR = crate::BitReader;
#[doc = "Field `ITCINTEN` writer - 21:21\\]
Intermediate transfer completion interrupt enable: 0: Intermediate transfer complete interrupt is disabled. 1: Intermediate transfer complete interrupt is enabled (corresponding IER\\[TCC\\]
bit must be set to 1 to generate interrupt)"]
pub type ItcintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCHEN` reader - 22:22\\]
Transfer complete chaining enable: 0: Transfer complete chaining is disabled. 1: Transfer complete chaining is enabled."]
pub type TcchenR = crate::BitReader;
#[doc = "Field `TCCHEN` writer - 22:22\\]
Transfer complete chaining enable: 0: Transfer complete chaining is disabled. 1: Transfer complete chaining is enabled."]
pub type TcchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITCCHEN` reader - 23:23\\]
Intermediate transfer completion chaining enable: 0: Intermediate transfer complete chaining is disabled. 1: Intermediate transfer complete chaining is enabled."]
pub type ItcchenR = crate::BitReader;
#[doc = "Field `ITCCHEN` writer - 23:23\\]
Intermediate transfer completion chaining enable: 0: Intermediate transfer complete chaining is disabled. 1: Intermediate transfer complete chaining is enabled."]
pub type ItcchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIVID` reader - 27:24\\]
Privilege ID: Privilege ID for the external host/cpu/dma that programmed this PaRAM Entry. This value is set with the vbus privid value when any part of the PaRAM Entry is written. Not writeable via vbus wdata bus. Is readable via VBus rdata bus."]
pub type PrividR = crate::FieldReader;
#[doc = "Field `PRIVID` writer - 27:24\\]
Privilege ID: Privilege ID for the external host/cpu/dma that programmed this PaRAM Entry. This value is set with the vbus privid value when any part of the PaRAM Entry is written. Not writeable via vbus wdata bus. Is readable via VBus rdata bus."]
pub type PrividW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES83` reader - 30:28\\]
RESERVE FIELD"]
pub type Res83R = crate::FieldReader;
#[doc = "Field `RES83` writer - 30:28\\]
RESERVE FIELD"]
pub type Res83W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIV` reader - 31:31\\]
Privilege level: privilege level (supervisor vs. user) for the host/cpu/dma that programmed this PaRAM Entry. Value is set with the vbus priv value when any part of the PaRAM Entry is written. Not writeable via vbus wdata bus. Is readable via VBus rdata bus. PRIV = 0 : User level privilege PRIV = 1 : Supervisor level privilege"]
pub type PrivR = crate::BitReader;
#[doc = "Field `PRIV` writer - 31:31\\]
Privilege level: privilege level (supervisor vs. user) for the host/cpu/dma that programmed this PaRAM Entry. Value is set with the vbus priv value when any part of the PaRAM Entry is written. Not writeable via vbus wdata bus. Is readable via VBus rdata bus. PRIV = 0 : User level privilege PRIV = 1 : Supervisor level privilege"]
pub type PrivW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Source Address Mode: Source Address Mode within an array. Pass-thru to TC. 0: INCR Src addressing within an array increments. Source is not a FIFO. 1: FIFO Src addressing within an array wraps around upon reaching FIFO width."]
    #[inline(always)]
    pub fn sam(&self) -> SamR {
        SamR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Destination Address Mode: Destination Address Mode within an array. Pass-thru to TC. 0: INCR Dst addressing within an array increments. Dst is not a FIFO. 1: FIFO Dst addressing within an array wraps around upon reaching FIFO width."]
    #[inline(always)]
    pub fn dam(&self) -> DamR {
        DamR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Transfer Synchronization Dimension: 0: A-Sync Each event triggers the transfer of ACNT elements. 1: AB-Sync Each event triggers the transfer of BCNT arrays of ACNT elements"]
    #[inline(always)]
    pub fn syncdim(&self) -> SyncdimR {
        SyncdimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Static Entry: 0: Entry is updated as normal 1: Entry is static Count and Address updates are not updated after TRP is submitted. Linking is not performed."]
    #[inline(always)]
    pub fn static_(&self) -> StaticR {
        StaticR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res85(&self) -> Res85R {
        Res85R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
FIFO width: Applies if either SAM or DAM is set to FIFO mode. Pass-thru to TC."]
    #[inline(always)]
    pub fn fwid(&self) -> FwidR {
        FwidR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Transfer complete code mode: Indicates the point at which a transfer is considered completed. Applies to both chaining and interrupt. 0: Normal Completion A transfer is considered completed after the transfer parameters are returned to the CC from the TC (which was returned from the peripheral). 1: Early Completion A transfer is considered completed after the CC submits a TR to the TC. CC generates completion code internally ."]
    #[inline(always)]
    pub fn tccmode(&self) -> TccmodeR {
        TccmodeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:17 - 17:12\\]
Transfer Complete Code: The 6-bit code is used to set the relevant bit in CER (bit CER\\[TCC\\]) for chaining or in IER (bit IER\\[TCC\\]) for interrupts."]
    #[inline(always)]
    pub fn tcc(&self) -> TccR {
        TccR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res84(&self) -> Res84R {
        Res84R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Backward compatibility mode: 0: Normal operation 1 : WI Backwards Compatibility mode forces BCNT to be adjusted by '1' upon TR submission (0 means 1 1 means 2 ... ) and forces ACNT to be treated as a word-count (left shifted by 2 by hardware to create byte cnt for TR submission)"]
    #[inline(always)]
    pub fn wimode(&self) -> WimodeR {
        WimodeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Transfer complete interrupt enable: 0: Transfer complete interrupt is disabled. 1: Transfer complete interrupt is enabled (corresponding IER\\[TCC\\]
bit must be set to 1 to generate interrupt)"]
    #[inline(always)]
    pub fn tcinten(&self) -> TcintenR {
        TcintenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Intermediate transfer completion interrupt enable: 0: Intermediate transfer complete interrupt is disabled. 1: Intermediate transfer complete interrupt is enabled (corresponding IER\\[TCC\\]
bit must be set to 1 to generate interrupt)"]
    #[inline(always)]
    pub fn itcinten(&self) -> ItcintenR {
        ItcintenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Transfer complete chaining enable: 0: Transfer complete chaining is disabled. 1: Transfer complete chaining is enabled."]
    #[inline(always)]
    pub fn tcchen(&self) -> TcchenR {
        TcchenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Intermediate transfer completion chaining enable: 0: Intermediate transfer complete chaining is disabled. 1: Intermediate transfer complete chaining is enabled."]
    #[inline(always)]
    pub fn itcchen(&self) -> ItcchenR {
        ItcchenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Privilege ID: Privilege ID for the external host/cpu/dma that programmed this PaRAM Entry. This value is set with the vbus privid value when any part of the PaRAM Entry is written. Not writeable via vbus wdata bus. Is readable via VBus rdata bus."]
    #[inline(always)]
    pub fn privid(&self) -> PrividR {
        PrividR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res83(&self) -> Res83R {
        Res83R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Privilege level: privilege level (supervisor vs. user) for the host/cpu/dma that programmed this PaRAM Entry. Value is set with the vbus priv value when any part of the PaRAM Entry is written. Not writeable via vbus wdata bus. Is readable via VBus rdata bus. PRIV = 0 : User level privilege PRIV = 1 : Supervisor level privilege"]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Source Address Mode: Source Address Mode within an array. Pass-thru to TC. 0: INCR Src addressing within an array increments. Source is not a FIFO. 1: FIFO Src addressing within an array wraps around upon reaching FIFO width."]
    #[inline(always)]
    #[must_use]
    pub fn sam(&mut self) -> SamW<OptSpec> {
        SamW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Destination Address Mode: Destination Address Mode within an array. Pass-thru to TC. 0: INCR Dst addressing within an array increments. Dst is not a FIFO. 1: FIFO Dst addressing within an array wraps around upon reaching FIFO width."]
    #[inline(always)]
    #[must_use]
    pub fn dam(&mut self) -> DamW<OptSpec> {
        DamW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Transfer Synchronization Dimension: 0: A-Sync Each event triggers the transfer of ACNT elements. 1: AB-Sync Each event triggers the transfer of BCNT arrays of ACNT elements"]
    #[inline(always)]
    #[must_use]
    pub fn syncdim(&mut self) -> SyncdimW<OptSpec> {
        SyncdimW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Static Entry: 0: Entry is updated as normal 1: Entry is static Count and Address updates are not updated after TRP is submitted. Linking is not performed."]
    #[inline(always)]
    #[must_use]
    pub fn static_(&mut self) -> StaticW<OptSpec> {
        StaticW::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res85(&mut self) -> Res85W<OptSpec> {
        Res85W::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
FIFO width: Applies if either SAM or DAM is set to FIFO mode. Pass-thru to TC."]
    #[inline(always)]
    #[must_use]
    pub fn fwid(&mut self) -> FwidW<OptSpec> {
        FwidW::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
Transfer complete code mode: Indicates the point at which a transfer is considered completed. Applies to both chaining and interrupt. 0: Normal Completion A transfer is considered completed after the transfer parameters are returned to the CC from the TC (which was returned from the peripheral). 1: Early Completion A transfer is considered completed after the CC submits a TR to the TC. CC generates completion code internally ."]
    #[inline(always)]
    #[must_use]
    pub fn tccmode(&mut self) -> TccmodeW<OptSpec> {
        TccmodeW::new(self, 11)
    }
    #[doc = "Bits 12:17 - 17:12\\]
Transfer Complete Code: The 6-bit code is used to set the relevant bit in CER (bit CER\\[TCC\\]) for chaining or in IER (bit IER\\[TCC\\]) for interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TccW<OptSpec> {
        TccW::new(self, 12)
    }
    #[doc = "Bit 18 - 18:18\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res84(&mut self) -> Res84W<OptSpec> {
        Res84W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Backward compatibility mode: 0: Normal operation 1 : WI Backwards Compatibility mode forces BCNT to be adjusted by '1' upon TR submission (0 means 1 1 means 2 ... ) and forces ACNT to be treated as a word-count (left shifted by 2 by hardware to create byte cnt for TR submission)"]
    #[inline(always)]
    #[must_use]
    pub fn wimode(&mut self) -> WimodeW<OptSpec> {
        WimodeW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Transfer complete interrupt enable: 0: Transfer complete interrupt is disabled. 1: Transfer complete interrupt is enabled (corresponding IER\\[TCC\\]
bit must be set to 1 to generate interrupt)"]
    #[inline(always)]
    #[must_use]
    pub fn tcinten(&mut self) -> TcintenW<OptSpec> {
        TcintenW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Intermediate transfer completion interrupt enable: 0: Intermediate transfer complete interrupt is disabled. 1: Intermediate transfer complete interrupt is enabled (corresponding IER\\[TCC\\]
bit must be set to 1 to generate interrupt)"]
    #[inline(always)]
    #[must_use]
    pub fn itcinten(&mut self) -> ItcintenW<OptSpec> {
        ItcintenW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Transfer complete chaining enable: 0: Transfer complete chaining is disabled. 1: Transfer complete chaining is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tcchen(&mut self) -> TcchenW<OptSpec> {
        TcchenW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Intermediate transfer completion chaining enable: 0: Intermediate transfer complete chaining is disabled. 1: Intermediate transfer complete chaining is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn itcchen(&mut self) -> ItcchenW<OptSpec> {
        ItcchenW::new(self, 23)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Privilege ID: Privilege ID for the external host/cpu/dma that programmed this PaRAM Entry. This value is set with the vbus privid value when any part of the PaRAM Entry is written. Not writeable via vbus wdata bus. Is readable via VBus rdata bus."]
    #[inline(always)]
    #[must_use]
    pub fn privid(&mut self) -> PrividW<OptSpec> {
        PrividW::new(self, 24)
    }
    #[doc = "Bits 28:30 - 30:28\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res83(&mut self) -> Res83W<OptSpec> {
        Res83W::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
Privilege level: privilege level (supervisor vs. user) for the host/cpu/dma that programmed this PaRAM Entry. Value is set with the vbus priv value when any part of the PaRAM Entry is written. Not writeable via vbus wdata bus. Is readable via VBus rdata bus. PRIV = 0 : User level privilege PRIV = 1 : Supervisor level privilege"]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PrivW<OptSpec> {
        PrivW::new(self, 31)
    }
}
#[doc = "Options Parameter\n\nYou can [`read`](crate::Reg::read) this register and get [`opt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptSpec;
impl crate::RegisterSpec for OptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opt::R`](R) reader structure"]
impl crate::Readable for OptSpec {}
#[doc = "`write(|w| ..)` method takes [`opt::W`](W) writer structure"]
impl crate::Writable for OptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPT to value 0"]
impl crate::Resettable for OptSpec {
    const RESET_VALUE: u32 = 0;
}
