#[doc = "Register `IODFTCTRL` reader"]
pub type R = crate::R<IodftctrlSpec>;
#[doc = "Register `IODFTCTRL` writer"]
pub type W = crate::W<IodftctrlSpec>;
#[doc = "Field `RXPENA` reader - 0:0\\]
Module Analog loopback through receive pin enable. This bit defines whether the I/O buffers for the transmit or the receive pin are included in the communication path in analog loopback mode only. Writable Only in privilege mode"]
pub type RxpenaR = crate::BitReader;
#[doc = "Field `RXPENA` writer - 0:0\\]
Module Analog loopback through receive pin enable. This bit defines whether the I/O buffers for the transmit or the receive pin are included in the communication path in analog loopback mode only. Writable Only in privilege mode"]
pub type RxpenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPBENA` reader - 1:1\\]
Module loopback enable. In analog loopback mode the complete communication path through the I/Os can be tested, whereas in digital loopback mode the I/O buffers are excluded from this path. Writable Only in privilege mode"]
pub type LpbenaR = crate::BitReader;
#[doc = "Field `LPBENA` writer - 1:1\\]
Module loopback enable. In analog loopback mode the complete communication path through the I/Os can be tested, whereas in digital loopback mode the I/O buffers are excluded from this path. Writable Only in privilege mode"]
pub type LpbenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IODFTENA` reader - 11:8\\]
IO DFT Enable Key This field is used to enable the IODFT mode of the SCI/LIN module for testing. Writable Only in privilege mode"]
pub type IodftenaR = crate::FieldReader;
#[doc = "Field `IODFTENA` writer - 11:8\\]
IO DFT Enable Key This field is used to enable the IODFT mode of the SCI/LIN module for testing. Writable Only in privilege mode"]
pub type IodftenaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Reserved1` reader - 15:12\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 15:12\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXSHIFT` reader - 18:16\\]
Transmit shift. These bits define the delay by which the value on LINTX is delayed so that the value on LINRX is asynchronous. (Not applicable to Start Bit)"]
pub type TxshiftR = crate::FieldReader;
#[doc = "Field `TXSHIFT` writer - 18:16\\]
Transmit shift. These bits define the delay by which the value on LINTX is delayed so that the value on LINRX is asynchronous. (Not applicable to Start Bit)"]
pub type TxshiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PINSAMPLEMASK` reader - 20:19\\]
Pin sample mask. These bits define the sample number at which the TX Pin value that is being transmitted will be inverted to verify the receive pin samples correctly with the majority detection circuitry. Note: During IODFT mode testing for the pin sample mask, the prescalar P must be programmed to be greater than 2."]
pub type PinsamplemaskR = crate::FieldReader;
#[doc = "Field `PINSAMPLEMASK` writer - 20:19\\]
Pin sample mask. These bits define the sample number at which the TX Pin value that is being transmitted will be inverted to verify the receive pin samples correctly with the majority detection circuitry. Note: During IODFT mode testing for the pin sample mask, the prescalar P must be programmed to be greater than 2."]
pub type PinsamplemaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - 23:21\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 23:21\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BRKDTERRENA` reader - 24:24\\]
Compatible Mode only This bit is effective in SCI-compatible mode only. This bit is used to create BRKDT error (SCI mode only). When this bit is set, the stop bit of the frame is ANDed with ΓÇÖ0ΓÇÖ and passed to the RSM so that a frame error occurs. Then the RX Pin is forced to continuous low for 10 Tbits so that a BRKDT error occurs."]
pub type BrkdterrenaR = crate::BitReader;
#[doc = "Field `BRKDTERRENA` writer - 24:24\\]
Compatible Mode only This bit is effective in SCI-compatible mode only. This bit is used to create BRKDT error (SCI mode only). When this bit is set, the stop bit of the frame is ANDed with ΓÇÖ0ΓÇÖ and passed to the RSM so that a frame error occurs. Then the RX Pin is forced to continuous low for 10 Tbits so that a BRKDT error occurs."]
pub type BrkdterrenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRENA` reader - 25:25\\]
Compatible Mode only This bit is effective in SCI-compatible mode only. This bit is used to create a Parity Error. When this bit is set, in compatible mode, the parity bit received is toggled so that a parity error occurs."]
pub type PerrenaR = crate::BitReader;
#[doc = "Field `PERRENA` writer - 25:25\\]
Compatible Mode only This bit is effective in SCI-compatible mode only. This bit is used to create a Parity Error. When this bit is set, in compatible mode, the parity bit received is toggled so that a parity error occurs."]
pub type PerrenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERRENA` reader - 26:26\\]
This bit is used to create a Frame Error. This bit is effective in SCI-compatible mode only. When this bit is set, the stop bit received is ANDed with ΓÇÖ0ΓÇÖ and passed to the stop bit check circuitry."]
pub type FerrenaR = crate::BitReader;
#[doc = "Field `FERRENA` writer - 26:26\\]
This bit is used to create a Frame Error. This bit is effective in SCI-compatible mode only. When this bit is set, the stop bit received is ANDed with ΓÇÖ0ΓÇÖ and passed to the stop bit check circuitry."]
pub type FerrenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - 27:27\\]
Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - 27:27\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISFERRENA` reader - 28:28\\]
Inconsistent Sync Field Error Enable bit. This bit is effective in LIN mode only. This bit is used to create an ISF error. When this bit is set, the bit widths in the sync field are varied so that the ISF check fails and the error flag is set."]
pub type IsferrenaR = crate::BitReader;
#[doc = "Field `ISFERRENA` writer - 28:28\\]
Inconsistent Sync Field Error Enable bit. This bit is effective in LIN mode only. This bit is used to create an ISF error. When this bit is set, the bit widths in the sync field are varied so that the ISF check fails and the error flag is set."]
pub type IsferrenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERRENA` reader - 29:29\\]
Checksum Error Enable bit. This bit is effective in LIN mode only. This bit is used to create a checksum error. When this bit is set, the polarity of the CTYPE (checksum type) in the receive checksum calculator is changed so that a checksum error is generated."]
pub type CerrenaR = crate::BitReader;
#[doc = "Field `CERRENA` writer - 29:29\\]
Checksum Error Enable bit. This bit is effective in LIN mode only. This bit is used to create a checksum error. When this bit is set, the polarity of the CTYPE (checksum type) in the receive checksum calculator is changed so that a checksum error is generated."]
pub type CerrenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBERRENA` reader - 30:30\\]
Physical Bus Error Enable bit. This bit is effective in LIN mode only. This bit is used to create a Physical Bus Error. When this bit is set, the bit received during Sync Break field transmission is ORed with 1 and passed to the Bit monitor circuitry"]
pub type PberrenaR = crate::BitReader;
#[doc = "Field `PBERRENA` writer - 30:30\\]
Physical Bus Error Enable bit. This bit is effective in LIN mode only. This bit is used to create a Physical Bus Error. When this bit is set, the bit received during Sync Break field transmission is ORed with 1 and passed to the Bit monitor circuitry"]
pub type PberrenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERRENA` reader - 31:31\\]
Bit Errror Enable bit. This bit is effective in LIN mode only. This bit is used to create a Bit error. When this bit is set, the bit received is ORed with 1 and passed to the Bit monitor circuitry."]
pub type BerrenaR = crate::BitReader;
#[doc = "Field `BERRENA` writer - 31:31\\]
Bit Errror Enable bit. This bit is effective in LIN mode only. This bit is used to create a Bit error. When this bit is set, the bit received is ORed with 1 and passed to the Bit monitor circuitry."]
pub type BerrenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Module Analog loopback through receive pin enable. This bit defines whether the I/O buffers for the transmit or the receive pin are included in the communication path in analog loopback mode only. Writable Only in privilege mode"]
    #[inline(always)]
    pub fn rxpena(&self) -> RxpenaR {
        RxpenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Module loopback enable. In analog loopback mode the complete communication path through the I/Os can be tested, whereas in digital loopback mode the I/O buffers are excluded from this path. Writable Only in privilege mode"]
    #[inline(always)]
    pub fn lpbena(&self) -> LpbenaR {
        LpbenaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
IO DFT Enable Key This field is used to enable the IODFT mode of the SCI/LIN module for testing. Writable Only in privilege mode"]
    #[inline(always)]
    pub fn iodftena(&self) -> IodftenaR {
        IodftenaR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Transmit shift. These bits define the delay by which the value on LINTX is delayed so that the value on LINRX is asynchronous. (Not applicable to Start Bit)"]
    #[inline(always)]
    pub fn txshift(&self) -> TxshiftR {
        TxshiftR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Pin sample mask. These bits define the sample number at which the TX Pin value that is being transmitted will be inverted to verify the receive pin samples correctly with the majority detection circuitry. Note: During IODFT mode testing for the pin sample mask, the prescalar P must be programmed to be greater than 2."]
    #[inline(always)]
    pub fn pinsamplemask(&self) -> PinsamplemaskR {
        PinsamplemaskR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Compatible Mode only This bit is effective in SCI-compatible mode only. This bit is used to create BRKDT error (SCI mode only). When this bit is set, the stop bit of the frame is ANDed with ΓÇÖ0ΓÇÖ and passed to the RSM so that a frame error occurs. Then the RX Pin is forced to continuous low for 10 Tbits so that a BRKDT error occurs."]
    #[inline(always)]
    pub fn brkdterrena(&self) -> BrkdterrenaR {
        BrkdterrenaR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Compatible Mode only This bit is effective in SCI-compatible mode only. This bit is used to create a Parity Error. When this bit is set, in compatible mode, the parity bit received is toggled so that a parity error occurs."]
    #[inline(always)]
    pub fn perrena(&self) -> PerrenaR {
        PerrenaR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
This bit is used to create a Frame Error. This bit is effective in SCI-compatible mode only. When this bit is set, the stop bit received is ANDed with ΓÇÖ0ΓÇÖ and passed to the stop bit check circuitry."]
    #[inline(always)]
    pub fn ferrena(&self) -> FerrenaR {
        FerrenaR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Inconsistent Sync Field Error Enable bit. This bit is effective in LIN mode only. This bit is used to create an ISF error. When this bit is set, the bit widths in the sync field are varied so that the ISF check fails and the error flag is set."]
    #[inline(always)]
    pub fn isferrena(&self) -> IsferrenaR {
        IsferrenaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Checksum Error Enable bit. This bit is effective in LIN mode only. This bit is used to create a checksum error. When this bit is set, the polarity of the CTYPE (checksum type) in the receive checksum calculator is changed so that a checksum error is generated."]
    #[inline(always)]
    pub fn cerrena(&self) -> CerrenaR {
        CerrenaR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Physical Bus Error Enable bit. This bit is effective in LIN mode only. This bit is used to create a Physical Bus Error. When this bit is set, the bit received during Sync Break field transmission is ORed with 1 and passed to the Bit monitor circuitry"]
    #[inline(always)]
    pub fn pberrena(&self) -> PberrenaR {
        PberrenaR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Bit Errror Enable bit. This bit is effective in LIN mode only. This bit is used to create a Bit error. When this bit is set, the bit received is ORed with 1 and passed to the Bit monitor circuitry."]
    #[inline(always)]
    pub fn berrena(&self) -> BerrenaR {
        BerrenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Module Analog loopback through receive pin enable. This bit defines whether the I/O buffers for the transmit or the receive pin are included in the communication path in analog loopback mode only. Writable Only in privilege mode"]
    #[inline(always)]
    #[must_use]
    pub fn rxpena(&mut self) -> RxpenaW<IodftctrlSpec> {
        RxpenaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Module loopback enable. In analog loopback mode the complete communication path through the I/Os can be tested, whereas in digital loopback mode the I/O buffers are excluded from this path. Writable Only in privilege mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpbena(&mut self) -> LpbenaW<IodftctrlSpec> {
        LpbenaW::new(self, 1)
    }
    #[doc = "Bits 8:11 - 11:8\\]
IO DFT Enable Key This field is used to enable the IODFT mode of the SCI/LIN module for testing. Writable Only in privilege mode"]
    #[inline(always)]
    #[must_use]
    pub fn iodftena(&mut self) -> IodftenaW<IodftctrlSpec> {
        IodftenaW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<IodftctrlSpec> {
        Reserved1W::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Transmit shift. These bits define the delay by which the value on LINTX is delayed so that the value on LINRX is asynchronous. (Not applicable to Start Bit)"]
    #[inline(always)]
    #[must_use]
    pub fn txshift(&mut self) -> TxshiftW<IodftctrlSpec> {
        TxshiftW::new(self, 16)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Pin sample mask. These bits define the sample number at which the TX Pin value that is being transmitted will be inverted to verify the receive pin samples correctly with the majority detection circuitry. Note: During IODFT mode testing for the pin sample mask, the prescalar P must be programmed to be greater than 2."]
    #[inline(always)]
    #[must_use]
    pub fn pinsamplemask(&mut self) -> PinsamplemaskW<IodftctrlSpec> {
        PinsamplemaskW::new(self, 19)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<IodftctrlSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
Compatible Mode only This bit is effective in SCI-compatible mode only. This bit is used to create BRKDT error (SCI mode only). When this bit is set, the stop bit of the frame is ANDed with ΓÇÖ0ΓÇÖ and passed to the RSM so that a frame error occurs. Then the RX Pin is forced to continuous low for 10 Tbits so that a BRKDT error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn brkdterrena(&mut self) -> BrkdterrenaW<IodftctrlSpec> {
        BrkdterrenaW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Compatible Mode only This bit is effective in SCI-compatible mode only. This bit is used to create a Parity Error. When this bit is set, in compatible mode, the parity bit received is toggled so that a parity error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn perrena(&mut self) -> PerrenaW<IodftctrlSpec> {
        PerrenaW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
This bit is used to create a Frame Error. This bit is effective in SCI-compatible mode only. When this bit is set, the stop bit received is ANDed with ΓÇÖ0ΓÇÖ and passed to the stop bit check circuitry."]
    #[inline(always)]
    #[must_use]
    pub fn ferrena(&mut self) -> FerrenaW<IodftctrlSpec> {
        FerrenaW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<IodftctrlSpec> {
        Reserved3W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Inconsistent Sync Field Error Enable bit. This bit is effective in LIN mode only. This bit is used to create an ISF error. When this bit is set, the bit widths in the sync field are varied so that the ISF check fails and the error flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn isferrena(&mut self) -> IsferrenaW<IodftctrlSpec> {
        IsferrenaW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Checksum Error Enable bit. This bit is effective in LIN mode only. This bit is used to create a checksum error. When this bit is set, the polarity of the CTYPE (checksum type) in the receive checksum calculator is changed so that a checksum error is generated."]
    #[inline(always)]
    #[must_use]
    pub fn cerrena(&mut self) -> CerrenaW<IodftctrlSpec> {
        CerrenaW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Physical Bus Error Enable bit. This bit is effective in LIN mode only. This bit is used to create a Physical Bus Error. When this bit is set, the bit received during Sync Break field transmission is ORed with 1 and passed to the Bit monitor circuitry"]
    #[inline(always)]
    #[must_use]
    pub fn pberrena(&mut self) -> PberrenaW<IodftctrlSpec> {
        PberrenaW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Bit Errror Enable bit. This bit is effective in LIN mode only. This bit is used to create a Bit error. When this bit is set, the bit received is ORed with 1 and passed to the Bit monitor circuitry."]
    #[inline(always)]
    #[must_use]
    pub fn berrena(&mut self) -> BerrenaW<IodftctrlSpec> {
        BerrenaW::new(self, 31)
    }
}
#[doc = "The IODFTCTRL register is used to emulate various error and test conditions.\n\nYou can [`read`](crate::Reg::read) this register and get [`iodftctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodftctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IodftctrlSpec;
impl crate::RegisterSpec for IodftctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iodftctrl::R`](R) reader structure"]
impl crate::Readable for IodftctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`iodftctrl::W`](W) writer structure"]
impl crate::Writable for IodftctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IODFTCTRL to value 0"]
impl crate::Resettable for IodftctrlSpec {
    const RESET_VALUE: u32 = 0;
}
