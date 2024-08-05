#[doc = "Register `TCSTAT` reader"]
pub type R = crate::R<TcstatSpec>;
#[doc = "Register `TCSTAT` writer"]
pub type W = crate::W<TcstatSpec>;
#[doc = "Field `PROGRAM_REGISTER_SET` reader - 0:0\\]
Program Register Set Busy#br#PROGBUSY = 0 : Prog set idle and is available for programming.#br#PROGBUSY = 1 : Prog set busy. User should poll for PROGBUSY equal to '0' prior to re-programming the Program Register set."]
pub type ProgramRegisterSetR = crate::BitReader;
#[doc = "Field `PROGRAM_REGISTER_SET` writer - 0:0\\]
Program Register Set Busy#br#PROGBUSY = 0 : Prog set idle and is available for programming.#br#PROGBUSY = 1 : Prog set busy. User should poll for PROGBUSY equal to '0' prior to re-programming the Program Register set."]
pub type ProgramRegisterSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOURCE_ACTIVE_STATE` reader - 1:1\\]
Source Active State#br#SRCACTV = 0 : Source Active set is idle. Any TR written to Prog Set will immediately transition to Source Active set as long as the Dst FIFO Set is not full \\[DSTFULL == 1\\].#br#SRCACTV = 1 : Source Active set is busy either performing read transfers or waiting to perform read transfers for current Transfer Request."]
pub type SourceActiveStateR = crate::BitReader;
#[doc = "Field `SOURCE_ACTIVE_STATE` writer - 1:1\\]
Source Active State#br#SRCACTV = 0 : Source Active set is idle. Any TR written to Prog Set will immediately transition to Source Active set as long as the Dst FIFO Set is not full \\[DSTFULL == 1\\].#br#SRCACTV = 1 : Source Active set is busy either performing read transfers or waiting to perform read transfers for current Transfer Request."]
pub type SourceActiveStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_STATUS_ACTIVE` reader - 2:2\\]
Write Status Active#br#WSACTV = 0 : Write status is not pending. Write status has been received for all previously issued write commands.#br#WSACTV = 1 : Write Status is pending. Write status has not been received for all previously issued write commands."]
pub type WriteStatusActiveR = crate::BitReader;
#[doc = "Field `WRITE_STATUS_ACTIVE` writer - 2:2\\]
Write Status Active#br#WSACTV = 0 : Write status is not pending. Write status has been received for all previously issued write commands.#br#WSACTV = 1 : Write Status is pending. Write status has not been received for all previously issued write commands."]
pub type WriteStatusActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESTINATION_ACTIVE_STATE` reader - 6:4\\]
Destination Active State#br#Specifies the number of TRs that are resident in the Dst Register FIFO at a given instant.#br#Legal values are constrained by the DSTREGDEPTH parameter."]
pub type DestinationActiveStateR = crate::FieldReader;
#[doc = "Field `DESTINATION_ACTIVE_STATE` writer - 6:4\\]
Destination Active State#br#Specifies the number of TRs that are resident in the Dst Register FIFO at a given instant.#br#Legal values are constrained by the DSTREGDEPTH parameter."]
pub type DestinationActiveStateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CHANNEL_ACTIVE` reader - 8:8\\]
Channel Active#br#Channel Active is a logical-OR of each of the BUSY/ACTV signals. The ACTV bit must remain high through the life of a TR.#br#ACTV = 0 : Channel is idle.#br#ACTV = 1 : Channel is busy."]
pub type ChannelActiveR = crate::BitReader;
#[doc = "Field `CHANNEL_ACTIVE` writer - 8:8\\]
Channel Active#br#Channel Active is a logical-OR of each of the BUSY/ACTV signals. The ACTV bit must remain high through the life of a TR.#br#ACTV = 0 : Channel is idle.#br#ACTV = 1 : Channel is busy."]
pub type ChannelActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_FIFO_START` reader - 13:12\\]
Dst FIFO Start Pointer#br#Represents the offset to the head entry of Dst Register FIFO in units of entries. Legal values = 0x0 to 0x3"]
pub type DstFifoStartR = crate::FieldReader;
#[doc = "Field `DST_FIFO_START` writer - 13:12\\]
Dst FIFO Start Pointer#br#Represents the offset to the head entry of Dst Register FIFO in units of entries. Legal values = 0x0 to 0x3"]
pub type DstFifoStartW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Program Register Set Busy#br#PROGBUSY = 0 : Prog set idle and is available for programming.#br#PROGBUSY = 1 : Prog set busy. User should poll for PROGBUSY equal to '0' prior to re-programming the Program Register set."]
    #[inline(always)]
    pub fn program_register_set(&self) -> ProgramRegisterSetR {
        ProgramRegisterSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Source Active State#br#SRCACTV = 0 : Source Active set is idle. Any TR written to Prog Set will immediately transition to Source Active set as long as the Dst FIFO Set is not full \\[DSTFULL == 1\\].#br#SRCACTV = 1 : Source Active set is busy either performing read transfers or waiting to perform read transfers for current Transfer Request."]
    #[inline(always)]
    pub fn source_active_state(&self) -> SourceActiveStateR {
        SourceActiveStateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Write Status Active#br#WSACTV = 0 : Write status is not pending. Write status has been received for all previously issued write commands.#br#WSACTV = 1 : Write Status is pending. Write status has not been received for all previously issued write commands."]
    #[inline(always)]
    pub fn write_status_active(&self) -> WriteStatusActiveR {
        WriteStatusActiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Destination Active State#br#Specifies the number of TRs that are resident in the Dst Register FIFO at a given instant.#br#Legal values are constrained by the DSTREGDEPTH parameter."]
    #[inline(always)]
    pub fn destination_active_state(&self) -> DestinationActiveStateR {
        DestinationActiveStateR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel Active#br#Channel Active is a logical-OR of each of the BUSY/ACTV signals. The ACTV bit must remain high through the life of a TR.#br#ACTV = 0 : Channel is idle.#br#ACTV = 1 : Channel is busy."]
    #[inline(always)]
    pub fn channel_active(&self) -> ChannelActiveR {
        ChannelActiveR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Dst FIFO Start Pointer#br#Represents the offset to the head entry of Dst Register FIFO in units of entries. Legal values = 0x0 to 0x3"]
    #[inline(always)]
    pub fn dst_fifo_start(&self) -> DstFifoStartR {
        DstFifoStartR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Program Register Set Busy#br#PROGBUSY = 0 : Prog set idle and is available for programming.#br#PROGBUSY = 1 : Prog set busy. User should poll for PROGBUSY equal to '0' prior to re-programming the Program Register set."]
    #[inline(always)]
    #[must_use]
    pub fn program_register_set(&mut self) -> ProgramRegisterSetW<TcstatSpec> {
        ProgramRegisterSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Source Active State#br#SRCACTV = 0 : Source Active set is idle. Any TR written to Prog Set will immediately transition to Source Active set as long as the Dst FIFO Set is not full \\[DSTFULL == 1\\].#br#SRCACTV = 1 : Source Active set is busy either performing read transfers or waiting to perform read transfers for current Transfer Request."]
    #[inline(always)]
    #[must_use]
    pub fn source_active_state(&mut self) -> SourceActiveStateW<TcstatSpec> {
        SourceActiveStateW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Write Status Active#br#WSACTV = 0 : Write status is not pending. Write status has been received for all previously issued write commands.#br#WSACTV = 1 : Write Status is pending. Write status has not been received for all previously issued write commands."]
    #[inline(always)]
    #[must_use]
    pub fn write_status_active(&mut self) -> WriteStatusActiveW<TcstatSpec> {
        WriteStatusActiveW::new(self, 2)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Destination Active State#br#Specifies the number of TRs that are resident in the Dst Register FIFO at a given instant.#br#Legal values are constrained by the DSTREGDEPTH parameter."]
    #[inline(always)]
    #[must_use]
    pub fn destination_active_state(&mut self) -> DestinationActiveStateW<TcstatSpec> {
        DestinationActiveStateW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel Active#br#Channel Active is a logical-OR of each of the BUSY/ACTV signals. The ACTV bit must remain high through the life of a TR.#br#ACTV = 0 : Channel is idle.#br#ACTV = 1 : Channel is busy."]
    #[inline(always)]
    #[must_use]
    pub fn channel_active(&mut self) -> ChannelActiveW<TcstatSpec> {
        ChannelActiveW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Dst FIFO Start Pointer#br#Represents the offset to the head entry of Dst Register FIFO in units of entries. Legal values = 0x0 to 0x3"]
    #[inline(always)]
    #[must_use]
    pub fn dst_fifo_start(&mut self) -> DstFifoStartW<TcstatSpec> {
        DstFifoStartW::new(self, 12)
    }
}
#[doc = "TC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcstatSpec;
impl crate::RegisterSpec for TcstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcstat::R`](R) reader structure"]
impl crate::Readable for TcstatSpec {}
#[doc = "`write(|w| ..)` method takes [`tcstat::W`](W) writer structure"]
impl crate::Writable for TcstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCSTAT to value 0"]
impl crate::Resettable for TcstatSpec {
    const RESET_VALUE: u32 = 0;
}
