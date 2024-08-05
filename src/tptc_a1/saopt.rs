#[doc = "Register `SAOPT` reader"]
pub type R = crate::R<SaoptSpec>;
#[doc = "Register `SAOPT` writer"]
pub type W = crate::W<SaoptSpec>;
#[doc = "Field `SOURCE_ADDRESS_MODE` reader - 0:0\\]
Source Address Mode within an array:#br#0: INCR Src addressing within an array increments.#br#1: FIFO Src addressing within an array wraps around upon reaching FIFO width."]
pub type SourceAddressModeR = crate::BitReader;
#[doc = "Field `SOURCE_ADDRESS_MODE` writer - 0:0\\]
Source Address Mode within an array:#br#0: INCR Src addressing within an array increments.#br#1: FIFO Src addressing within an array wraps around upon reaching FIFO width."]
pub type SourceAddressModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESTINATION_ADDRESS_MODE` reader - 1:1\\]
Destination Address Mode within an array:#br#0: INCR Dst addressing within an array increments.#br#1: FIFO Dst addressing within an array wraps around upon reaching FIFO width."]
pub type DestinationAddressModeR = crate::BitReader;
#[doc = "Field `DESTINATION_ADDRESS_MODE` writer - 1:1\\]
Destination Address Mode within an array:#br#0: INCR Dst addressing within an array increments.#br#1: FIFO Dst addressing within an array wraps around upon reaching FIFO width."]
pub type DestinationAddressModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFER_PRIORITY` reader - 6:4\\]
Transfer Priority:#br#0: Priority 0 - Highest priority#br#1: Priority 1 ...#br#7: Priority 7 - Lowest priority"]
pub type TransferPriorityR = crate::FieldReader;
#[doc = "Field `TRANSFER_PRIORITY` writer - 6:4\\]
Transfer Priority:#br#0: Priority 0 - Highest priority#br#1: Priority 1 ...#br#7: Priority 7 - Lowest priority"]
pub type TransferPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FIFO_WIDTH_CONTROL` reader - 10:8\\]
FIFO width control: Applies if either SAM or DAM is set to FIFO mode."]
pub type FifoWidthControlR = crate::FieldReader;
#[doc = "Field `FIFO_WIDTH_CONTROL` writer - 10:8\\]
FIFO width control: Applies if either SAM or DAM is set to FIFO mode."]
pub type FifoWidthControlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRANSFER_COMPLETE_CODE` reader - 17:12\\]
Transfer Complete Code: The 6-bit code is used to set the relevant bit in CER or IPR of the TPCC module."]
pub type TransferCompleteCodeR = crate::FieldReader;
#[doc = "Field `TRANSFER_COMPLETE_CODE` writer - 17:12\\]
Transfer Complete Code: The 6-bit code is used to set the relevant bit in CER or IPR of the TPCC module."]
pub type TransferCompleteCodeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRANSFER_COMPLETE_INTERRUPT` reader - 20:20\\]
Transfer complete interrupt enable:#br#0: Transfer complete interrupt is disabled.#br#1: Transfer complete interrupt is enabled."]
pub type TransferCompleteInterruptR = crate::BitReader;
#[doc = "Field `TRANSFER_COMPLETE_INTERRUPT` writer - 20:20\\]
Transfer complete interrupt enable:#br#0: Transfer complete interrupt is disabled.#br#1: Transfer complete interrupt is enabled."]
pub type TransferCompleteInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFER_COMPLETE_CHAINING` reader - 22:22\\]
Transfer complete chaining enable:#br#0: Transfer complete chaining is disabled.#br#1: Transfer complete chaining is enabled."]
pub type TransferCompleteChainingR = crate::BitReader;
#[doc = "Field `TRANSFER_COMPLETE_CHAINING` writer - 22:22\\]
Transfer complete chaining enable:#br#0: Transfer complete chaining is disabled.#br#1: Transfer complete chaining is enabled."]
pub type TransferCompleteChainingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUG_ID` reader - 29:28\\]
Debug ID#br#Value driven on the read (tptc_r_dbg_channel_id) and write (tptc_w_dbg_channel_id) command bus.#br#Used at system level for trace/profiling of user selected transfers in systems that include this feature."]
pub type DebugIdR = crate::FieldReader;
#[doc = "Field `DEBUG_ID` writer - 29:28\\]
Debug ID#br#Value driven on the read (tptc_r_dbg_channel_id) and write (tptc_w_dbg_channel_id) command bus.#br#Used at system level for trace/profiling of user selected transfers in systems that include this feature."]
pub type DebugIdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Source Address Mode within an array:#br#0: INCR Src addressing within an array increments.#br#1: FIFO Src addressing within an array wraps around upon reaching FIFO width."]
    #[inline(always)]
    pub fn source_address_mode(&self) -> SourceAddressModeR {
        SourceAddressModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Destination Address Mode within an array:#br#0: INCR Dst addressing within an array increments.#br#1: FIFO Dst addressing within an array wraps around upon reaching FIFO width."]
    #[inline(always)]
    pub fn destination_address_mode(&self) -> DestinationAddressModeR {
        DestinationAddressModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Transfer Priority:#br#0: Priority 0 - Highest priority#br#1: Priority 1 ...#br#7: Priority 7 - Lowest priority"]
    #[inline(always)]
    pub fn transfer_priority(&self) -> TransferPriorityR {
        TransferPriorityR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
FIFO width control: Applies if either SAM or DAM is set to FIFO mode."]
    #[inline(always)]
    pub fn fifo_width_control(&self) -> FifoWidthControlR {
        FifoWidthControlR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:17 - 17:12\\]
Transfer Complete Code: The 6-bit code is used to set the relevant bit in CER or IPR of the TPCC module."]
    #[inline(always)]
    pub fn transfer_complete_code(&self) -> TransferCompleteCodeR {
        TransferCompleteCodeR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Transfer complete interrupt enable:#br#0: Transfer complete interrupt is disabled.#br#1: Transfer complete interrupt is enabled."]
    #[inline(always)]
    pub fn transfer_complete_interrupt(&self) -> TransferCompleteInterruptR {
        TransferCompleteInterruptR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Transfer complete chaining enable:#br#0: Transfer complete chaining is disabled.#br#1: Transfer complete chaining is enabled."]
    #[inline(always)]
    pub fn transfer_complete_chaining(&self) -> TransferCompleteChainingR {
        TransferCompleteChainingR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Debug ID#br#Value driven on the read (tptc_r_dbg_channel_id) and write (tptc_w_dbg_channel_id) command bus.#br#Used at system level for trace/profiling of user selected transfers in systems that include this feature."]
    #[inline(always)]
    pub fn debug_id(&self) -> DebugIdR {
        DebugIdR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Source Address Mode within an array:#br#0: INCR Src addressing within an array increments.#br#1: FIFO Src addressing within an array wraps around upon reaching FIFO width."]
    #[inline(always)]
    #[must_use]
    pub fn source_address_mode(&mut self) -> SourceAddressModeW<SaoptSpec> {
        SourceAddressModeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Destination Address Mode within an array:#br#0: INCR Dst addressing within an array increments.#br#1: FIFO Dst addressing within an array wraps around upon reaching FIFO width."]
    #[inline(always)]
    #[must_use]
    pub fn destination_address_mode(&mut self) -> DestinationAddressModeW<SaoptSpec> {
        DestinationAddressModeW::new(self, 1)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Transfer Priority:#br#0: Priority 0 - Highest priority#br#1: Priority 1 ...#br#7: Priority 7 - Lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_priority(&mut self) -> TransferPriorityW<SaoptSpec> {
        TransferPriorityW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
FIFO width control: Applies if either SAM or DAM is set to FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_width_control(&mut self) -> FifoWidthControlW<SaoptSpec> {
        FifoWidthControlW::new(self, 8)
    }
    #[doc = "Bits 12:17 - 17:12\\]
Transfer Complete Code: The 6-bit code is used to set the relevant bit in CER or IPR of the TPCC module."]
    #[inline(always)]
    #[must_use]
    pub fn transfer_complete_code(&mut self) -> TransferCompleteCodeW<SaoptSpec> {
        TransferCompleteCodeW::new(self, 12)
    }
    #[doc = "Bit 20 - 20:20\\]
Transfer complete interrupt enable:#br#0: Transfer complete interrupt is disabled.#br#1: Transfer complete interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn transfer_complete_interrupt(&mut self) -> TransferCompleteInterruptW<SaoptSpec> {
        TransferCompleteInterruptW::new(self, 20)
    }
    #[doc = "Bit 22 - 22:22\\]
Transfer complete chaining enable:#br#0: Transfer complete chaining is disabled.#br#1: Transfer complete chaining is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn transfer_complete_chaining(&mut self) -> TransferCompleteChainingW<SaoptSpec> {
        TransferCompleteChainingW::new(self, 22)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Debug ID#br#Value driven on the read (tptc_r_dbg_channel_id) and write (tptc_w_dbg_channel_id) command bus.#br#Used at system level for trace/profiling of user selected transfers in systems that include this feature."]
    #[inline(always)]
    #[must_use]
    pub fn debug_id(&mut self) -> DebugIdW<SaoptSpec> {
        DebugIdW::new(self, 28)
    }
}
#[doc = "Src Actv Set Options\n\nYou can [`read`](crate::Reg::read) this register and get [`saopt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saopt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaoptSpec;
impl crate::RegisterSpec for SaoptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saopt::R`](R) reader structure"]
impl crate::Readable for SaoptSpec {}
#[doc = "`write(|w| ..)` method takes [`saopt::W`](W) writer structure"]
impl crate::Writable for SaoptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAOPT to value 0"]
impl crate::Resettable for SaoptSpec {
    const RESET_VALUE: u32 = 0;
}
