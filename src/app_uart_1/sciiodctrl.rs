#[doc = "Register `SCIIODCTRL` reader"]
pub type R = crate::R<SciiodctrlSpec>;
#[doc = "Register `SCIIODCTRL` writer"]
pub type W = crate::W<SciiodctrlSpec>;
#[doc = "Field `RXP_ENA` reader - 0:0\\]
Module Analog loopback through receive pin enable. user and privileged mode reads: Write only in privileged mode: write/read : 1=Analog loopback through receive pin. 0=Analog loopback through transmit pin."]
pub type RxpEnaR = crate::BitReader;
#[doc = "Field `RXP_ENA` writer - 0:0\\]
Module Analog loopback through receive pin enable. user and privileged mode reads: Write only in privileged mode: write/read : 1=Analog loopback through receive pin. 0=Analog loopback through transmit pin."]
pub type RxpEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBP_ENA` reader - 1:1\\]
Module loopback enable. user and privileged mode reads: Write only in privileged mode: write/read : 1=Analog loopback is enabled in module I/O DFT mode(when IODFTENA = 1010) 0=Digital loopback is enabled."]
pub type LbpEnaR = crate::BitReader;
#[doc = "Field `LBP_ENA` writer - 1:1\\]
Module loopback enable. user and privileged mode reads: Write only in privileged mode: write/read : 1=Analog loopback is enabled in module I/O DFT mode(when IODFTENA = 1010) 0=Digital loopback is enabled."]
pub type LbpEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:2\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:2\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IODFTENA` reader - 11:8\\]
These bits define the delay by which the value on TX pin is delayed so that the value on RX Pin is asynchronous. (Not applicable to Start Bit) TX SHIFT: 000 -- No Delay, 001 -- Delay by 1 SCLK, 010 -- Delay by 2 SCLKs, 011 -- Delay by 3 SCLKs, 100 -- Delay by 4 SCLKs, 101 -- Delay by 5 SCLKs, 110 -- Delay by 6 SCLKs, 111 -- No Delay."]
pub type IodftenaR = crate::FieldReader;
#[doc = "Field `IODFTENA` writer - 11:8\\]
These bits define the delay by which the value on TX pin is delayed so that the value on RX Pin is asynchronous. (Not applicable to Start Bit) TX SHIFT: 000 -- No Delay, 001 -- Delay by 1 SCLK, 010 -- Delay by 2 SCLKs, 011 -- Delay by 3 SCLKs, 100 -- Delay by 4 SCLKs, 101 -- Delay by 5 SCLKs, 110 -- Delay by 6 SCLKs, 111 -- No Delay."]
pub type IodftenaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED2` reader - 15:12\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 15:12\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_SHIFT` reader - 18:16\\]
These bits define the delay by which the value on TX pin is delayed so that the value on RX Pin is asynchronous. (Not applicable to Start Bit) TX SHIFT: 000 -- No Delay, 001 -- Delay by 1 SCLK, 010 -- Delay by 2 SCLKs, 011 -- Delay by 3 SCLKs, 100 -- Delay by 4 SCLKs, 101 -- Delay by 5 SCLKs, 110 -- Delay by 6 SCLKs, 111 -- No Delay."]
pub type TxShiftR = crate::FieldReader;
#[doc = "Field `TX_SHIFT` writer - 18:16\\]
These bits define the delay by which the value on TX pin is delayed so that the value on RX Pin is asynchronous. (Not applicable to Start Bit) TX SHIFT: 000 -- No Delay, 001 -- Delay by 1 SCLK, 010 -- Delay by 2 SCLKs, 011 -- Delay by 3 SCLKs, 100 -- Delay by 4 SCLKs, 101 -- Delay by 5 SCLKs, 110 -- Delay by 6 SCLKs, 111 -- No Delay."]
pub type TxShiftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PIN_SAMPLE_MASK` reader - 20:19\\]
PIN SAMPLE MASK These bits define the sample number at which the TX Pin value that is being transmitted will be inverted to verify the receive pin samples majority detection circuitry. PIN SAMPLE MASK: 00 -- No Mask, 01 -- Invert the TX Pin value at 7th SCLK, 10 -- Invert the TX Pin value at 8th SCLK, 11 -- Invert the TX Pin value at 9th SCLK."]
pub type PinSampleMaskR = crate::FieldReader;
#[doc = "Field `PIN_SAMPLE_MASK` writer - 20:19\\]
PIN SAMPLE MASK These bits define the sample number at which the TX Pin value that is being transmitted will be inverted to verify the receive pin samples majority detection circuitry. PIN SAMPLE MASK: 00 -- No Mask, 01 -- Invert the TX Pin value at 7th SCLK, 10 -- Invert the TX Pin value at 8th SCLK, 11 -- Invert the TX Pin value at 9th SCLK."]
pub type PinSampleMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED3` reader - 23:21\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 23:21\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BRKDT_ENA` reader - 24:24\\]
Break Detect Error Enable. User and Privileged Mode Reads and Writes: 1 = This bit is used to create BRKDT Error. The stop bit of the frame is ANDed with ΓÇÿ0ΓÇÖ and passed to the RSM so that a frame error occurs. Then the RX pin is forced to continuous low for 10 TBITS so that a BRKDT error occurs. 0 = No effect."]
pub type BrkdtEnaR = crate::BitReader;
#[doc = "Field `BRKDT_ENA` writer - 24:24\\]
Break Detect Error Enable. User and Privileged Mode Reads and Writes: 1 = This bit is used to create BRKDT Error. The stop bit of the frame is ANDed with ΓÇÿ0ΓÇÖ and passed to the RSM so that a frame error occurs. Then the RX pin is forced to continuous low for 10 TBITS so that a BRKDT error occurs. 0 = No effect."]
pub type BrkdtEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - 25:25\\]
Parity Error Enable. User and Privileged Mode Reads and Writes: 1 = This bit is used to create a Parity Error. The parity bit received is toggled so that a parity error occurs. 0 = No effect"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - 25:25\\]
Parity Error Enable. User and Privileged Mode Reads and Writes: 1 = This bit is used to create a Parity Error. The parity bit received is toggled so that a parity error occurs. 0 = No effect"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN` reader - 26:26\\]
Frame Error Enable. User and Privileged Mode Reads and Writes: 1 = This bit is used to create a Frame Error. The stop bit received is ANDed with ΓÇÖ0ΓÇÖ and passed to the stop bit check circuitry. 0 = No effect."]
pub type FenR = crate::BitReader;
#[doc = "Field `FEN` writer - 26:26\\]
Frame Error Enable. User and Privileged Mode Reads and Writes: 1 = This bit is used to create a Frame Error. The stop bit received is ANDed with ΓÇÖ0ΓÇÖ and passed to the stop bit check circuitry. 0 = No effect."]
pub type FenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 31:27\\]
Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 31:27\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Module Analog loopback through receive pin enable. user and privileged mode reads: Write only in privileged mode: write/read : 1=Analog loopback through receive pin. 0=Analog loopback through transmit pin."]
    #[inline(always)]
    pub fn rxp_ena(&self) -> RxpEnaR {
        RxpEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Module loopback enable. user and privileged mode reads: Write only in privileged mode: write/read : 1=Analog loopback is enabled in module I/O DFT mode(when IODFTENA = 1010) 0=Digital loopback is enabled."]
    #[inline(always)]
    pub fn lbp_ena(&self) -> LbpEnaR {
        LbpEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
These bits define the delay by which the value on TX pin is delayed so that the value on RX Pin is asynchronous. (Not applicable to Start Bit) TX SHIFT: 000 -- No Delay, 001 -- Delay by 1 SCLK, 010 -- Delay by 2 SCLKs, 011 -- Delay by 3 SCLKs, 100 -- Delay by 4 SCLKs, 101 -- Delay by 5 SCLKs, 110 -- Delay by 6 SCLKs, 111 -- No Delay."]
    #[inline(always)]
    pub fn iodftena(&self) -> IodftenaR {
        IodftenaR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
These bits define the delay by which the value on TX pin is delayed so that the value on RX Pin is asynchronous. (Not applicable to Start Bit) TX SHIFT: 000 -- No Delay, 001 -- Delay by 1 SCLK, 010 -- Delay by 2 SCLKs, 011 -- Delay by 3 SCLKs, 100 -- Delay by 4 SCLKs, 101 -- Delay by 5 SCLKs, 110 -- Delay by 6 SCLKs, 111 -- No Delay."]
    #[inline(always)]
    pub fn tx_shift(&self) -> TxShiftR {
        TxShiftR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - 20:19\\]
PIN SAMPLE MASK These bits define the sample number at which the TX Pin value that is being transmitted will be inverted to verify the receive pin samples majority detection circuitry. PIN SAMPLE MASK: 00 -- No Mask, 01 -- Invert the TX Pin value at 7th SCLK, 10 -- Invert the TX Pin value at 8th SCLK, 11 -- Invert the TX Pin value at 9th SCLK."]
    #[inline(always)]
    pub fn pin_sample_mask(&self) -> PinSampleMaskR {
        PinSampleMaskR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Break Detect Error Enable. User and Privileged Mode Reads and Writes: 1 = This bit is used to create BRKDT Error. The stop bit of the frame is ANDed with ΓÇÿ0ΓÇÖ and passed to the RSM so that a frame error occurs. Then the RX pin is forced to continuous low for 10 TBITS so that a BRKDT error occurs. 0 = No effect."]
    #[inline(always)]
    pub fn brkdt_ena(&self) -> BrkdtEnaR {
        BrkdtEnaR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Parity Error Enable. User and Privileged Mode Reads and Writes: 1 = This bit is used to create a Parity Error. The parity bit received is toggled so that a parity error occurs. 0 = No effect"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Frame Error Enable. User and Privileged Mode Reads and Writes: 1 = This bit is used to create a Frame Error. The stop bit received is ANDed with ΓÇÖ0ΓÇÖ and passed to the stop bit check circuitry. 0 = No effect."]
    #[inline(always)]
    pub fn fen(&self) -> FenR {
        FenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Module Analog loopback through receive pin enable. user and privileged mode reads: Write only in privileged mode: write/read : 1=Analog loopback through receive pin. 0=Analog loopback through transmit pin."]
    #[inline(always)]
    #[must_use]
    pub fn rxp_ena(&mut self) -> RxpEnaW<SciiodctrlSpec> {
        RxpEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Module loopback enable. user and privileged mode reads: Write only in privileged mode: write/read : 1=Analog loopback is enabled in module I/O DFT mode(when IODFTENA = 1010) 0=Digital loopback is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn lbp_ena(&mut self) -> LbpEnaW<SciiodctrlSpec> {
        LbpEnaW::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SciiodctrlSpec> {
        Reserved1W::new(self, 2)
    }
    #[doc = "Bits 8:11 - 11:8\\]
These bits define the delay by which the value on TX pin is delayed so that the value on RX Pin is asynchronous. (Not applicable to Start Bit) TX SHIFT: 000 -- No Delay, 001 -- Delay by 1 SCLK, 010 -- Delay by 2 SCLKs, 011 -- Delay by 3 SCLKs, 100 -- Delay by 4 SCLKs, 101 -- Delay by 5 SCLKs, 110 -- Delay by 6 SCLKs, 111 -- No Delay."]
    #[inline(always)]
    #[must_use]
    pub fn iodftena(&mut self) -> IodftenaW<SciiodctrlSpec> {
        IodftenaW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<SciiodctrlSpec> {
        Reserved2W::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
These bits define the delay by which the value on TX pin is delayed so that the value on RX Pin is asynchronous. (Not applicable to Start Bit) TX SHIFT: 000 -- No Delay, 001 -- Delay by 1 SCLK, 010 -- Delay by 2 SCLKs, 011 -- Delay by 3 SCLKs, 100 -- Delay by 4 SCLKs, 101 -- Delay by 5 SCLKs, 110 -- Delay by 6 SCLKs, 111 -- No Delay."]
    #[inline(always)]
    #[must_use]
    pub fn tx_shift(&mut self) -> TxShiftW<SciiodctrlSpec> {
        TxShiftW::new(self, 16)
    }
    #[doc = "Bits 19:20 - 20:19\\]
PIN SAMPLE MASK These bits define the sample number at which the TX Pin value that is being transmitted will be inverted to verify the receive pin samples majority detection circuitry. PIN SAMPLE MASK: 00 -- No Mask, 01 -- Invert the TX Pin value at 7th SCLK, 10 -- Invert the TX Pin value at 8th SCLK, 11 -- Invert the TX Pin value at 9th SCLK."]
    #[inline(always)]
    #[must_use]
    pub fn pin_sample_mask(&mut self) -> PinSampleMaskW<SciiodctrlSpec> {
        PinSampleMaskW::new(self, 19)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SciiodctrlSpec> {
        Reserved3W::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
Break Detect Error Enable. User and Privileged Mode Reads and Writes: 1 = This bit is used to create BRKDT Error. The stop bit of the frame is ANDed with ΓÇÿ0ΓÇÖ and passed to the RSM so that a frame error occurs. Then the RX pin is forced to continuous low for 10 TBITS so that a BRKDT error occurs. 0 = No effect."]
    #[inline(always)]
    #[must_use]
    pub fn brkdt_ena(&mut self) -> BrkdtEnaW<SciiodctrlSpec> {
        BrkdtEnaW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Parity Error Enable. User and Privileged Mode Reads and Writes: 1 = This bit is used to create a Parity Error. The parity bit received is toggled so that a parity error occurs. 0 = No effect"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<SciiodctrlSpec> {
        PenW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Frame Error Enable. User and Privileged Mode Reads and Writes: 1 = This bit is used to create a Frame Error. The stop bit received is ANDed with ΓÇÖ0ΓÇÖ and passed to the stop bit check circuitry. 0 = No effect."]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FenW<SciiodctrlSpec> {
        FenW::new(self, 26)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<SciiodctrlSpec> {
        Reserved4W::new(self, 27)
    }
}
#[doc = "SCI IO DFT Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sciiodctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciiodctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SciiodctrlSpec;
impl crate::RegisterSpec for SciiodctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sciiodctrl::R`](R) reader structure"]
impl crate::Readable for SciiodctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sciiodctrl::W`](W) writer structure"]
impl crate::Writable for SciiodctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIIODCTRL to value 0"]
impl crate::Resettable for SciiodctrlSpec {
    const RESET_VALUE: u32 = 0;
}
