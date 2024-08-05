#[doc = "Register `SCIGCR2` reader"]
pub type R = crate::R<Scigcr2Spec>;
#[doc = "Register `SCIGCR2` writer"]
pub type W = crate::W<Scigcr2Spec>;
#[doc = "Field `POWERDOWN` reader - 0:0\\]
Power down. This bit is effective in LIN or SCI-compatible mode. When the powerdown bit is set, the SCI/LIN module attempts to enter local low-power mode. If the POWERDOWN bit is set while the receiver is actively receiving data and the wakeup interrupt is disabled, then the SCI/LIN will delay low-power mode from being entered until completion of reception. In LIN mode the user may set the POWERDOWN bit on Sleep Command reception or on idle bus detection (more than 4 seconds, i.e. 80,000 cycles at 20kHz)"]
pub type PowerdownR = crate::BitReader;
#[doc = "Field `POWERDOWN` writer - 0:0\\]
Power down. This bit is effective in LIN or SCI-compatible mode. When the powerdown bit is set, the SCI/LIN module attempts to enter local low-power mode. If the POWERDOWN bit is set while the receiver is actively receiving data and the wakeup interrupt is disabled, then the SCI/LIN will delay low-power mode from being entered until completion of reception. In LIN mode the user may set the POWERDOWN bit on Sleep Command reception or on idle bus detection (more than 4 seconds, i.e. 80,000 cycles at 20kHz)"]
pub type PowerdownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENWU` reader - 8:8\\]
Generate wakeup signal. This bit controls the generation of a wakeup signal, by transmitting the TDO buffer value. This bit is cleared on reception of a valid sync break."]
pub type GenwuR = crate::BitReader;
#[doc = "Field `GENWU` writer - 8:8\\]
Generate wakeup signal. This bit controls the generation of a wakeup signal, by transmitting the TDO buffer value. This bit is cleared on reception of a valid sync break."]
pub type GenwuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 15:9\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 15:9\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SC` reader - 16:16\\]
Send Checksum This mode is effective in LIN mode only. This bit is used by the transmitter with extended frames to send a checkbyte. In non multibuffer mode the checkbyte will be sent after the current byte transmission. In multibuffer mode the checkbyte will be sent after the last byte count, indicated by the SCIFORMAT\\[18:16\\]). This field is writable in LIN mode only."]
pub type ScR = crate::BitReader;
#[doc = "Field `SC` writer - 16:16\\]
Send Checksum This mode is effective in LIN mode only. This bit is used by the transmitter with extended frames to send a checkbyte. In non multibuffer mode the checkbyte will be sent after the current byte transmission. In multibuffer mode the checkbyte will be sent after the last byte count, indicated by the SCIFORMAT\\[18:16\\]). This field is writable in LIN mode only."]
pub type ScW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC` reader - 17:17\\]
Compare Checksum. This mode is effective in LIN mode only. This bit is used by the receiver for extended frames to trigger a checksum compare. The user will initiate this transaction by writing a one to this bit. In non multibuffer mode, once the CC bit is set, the checksum will be compared on the byte that is currently being received, expected to be the checkbyte. During Multi-buffer mode, following are the scenarios associated with the CC bit : - If CC bit is set during the reception of the data, then the byte that is received after the reception of the programmed no. of data bytes indicated by SCIFORMAT\\[18:16\\], is treated as a checksum byte. - If CC bit is set during the IDLE period (i.e. during inter-frame space), then the next immediate byte will be treated as a checksum byte. A CE will immediatly be flagged if there is a checksum error. This bit is automatically cleared once the checksum is successfully compared."]
pub type CcR = crate::BitReader;
#[doc = "Field `CC` writer - 17:17\\]
Compare Checksum. This mode is effective in LIN mode only. This bit is used by the receiver for extended frames to trigger a checksum compare. The user will initiate this transaction by writing a one to this bit. In non multibuffer mode, once the CC bit is set, the checksum will be compared on the byte that is currently being received, expected to be the checkbyte. During Multi-buffer mode, following are the scenarios associated with the CC bit : - If CC bit is set during the reception of the data, then the byte that is received after the reception of the programmed no. of data bytes indicated by SCIFORMAT\\[18:16\\], is treated as a checksum byte. - If CC bit is set during the IDLE period (i.e. during inter-frame space), then the next immediate byte will be treated as a checksum byte. A CE will immediatly be flagged if there is a checksum error. This bit is automatically cleared once the checksum is successfully compared."]
pub type CcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 31:18\\]
Reserved"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `Reserved2` writer - 31:18\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Power down. This bit is effective in LIN or SCI-compatible mode. When the powerdown bit is set, the SCI/LIN module attempts to enter local low-power mode. If the POWERDOWN bit is set while the receiver is actively receiving data and the wakeup interrupt is disabled, then the SCI/LIN will delay low-power mode from being entered until completion of reception. In LIN mode the user may set the POWERDOWN bit on Sleep Command reception or on idle bus detection (more than 4 seconds, i.e. 80,000 cycles at 20kHz)"]
    #[inline(always)]
    pub fn powerdown(&self) -> PowerdownR {
        PowerdownR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Generate wakeup signal. This bit controls the generation of a wakeup signal, by transmitting the TDO buffer value. This bit is cleared on reception of a valid sync break."]
    #[inline(always)]
    pub fn genwu(&self) -> GenwuR {
        GenwuR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Send Checksum This mode is effective in LIN mode only. This bit is used by the transmitter with extended frames to send a checkbyte. In non multibuffer mode the checkbyte will be sent after the current byte transmission. In multibuffer mode the checkbyte will be sent after the last byte count, indicated by the SCIFORMAT\\[18:16\\]). This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn sc(&self) -> ScR {
        ScR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Compare Checksum. This mode is effective in LIN mode only. This bit is used by the receiver for extended frames to trigger a checksum compare. The user will initiate this transaction by writing a one to this bit. In non multibuffer mode, once the CC bit is set, the checksum will be compared on the byte that is currently being received, expected to be the checkbyte. During Multi-buffer mode, following are the scenarios associated with the CC bit : - If CC bit is set during the reception of the data, then the byte that is received after the reception of the programmed no. of data bytes indicated by SCIFORMAT\\[18:16\\], is treated as a checksum byte. - If CC bit is set during the IDLE period (i.e. during inter-frame space), then the next immediate byte will be treated as a checksum byte. A CE will immediatly be flagged if there is a checksum error. This bit is automatically cleared once the checksum is successfully compared."]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power down. This bit is effective in LIN or SCI-compatible mode. When the powerdown bit is set, the SCI/LIN module attempts to enter local low-power mode. If the POWERDOWN bit is set while the receiver is actively receiving data and the wakeup interrupt is disabled, then the SCI/LIN will delay low-power mode from being entered until completion of reception. In LIN mode the user may set the POWERDOWN bit on Sleep Command reception or on idle bus detection (more than 4 seconds, i.e. 80,000 cycles at 20kHz)"]
    #[inline(always)]
    #[must_use]
    pub fn powerdown(&mut self) -> PowerdownW<Scigcr2Spec> {
        PowerdownW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Generate wakeup signal. This bit controls the generation of a wakeup signal, by transmitting the TDO buffer value. This bit is cleared on reception of a valid sync break."]
    #[inline(always)]
    #[must_use]
    pub fn genwu(&mut self) -> GenwuW<Scigcr2Spec> {
        GenwuW::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Scigcr2Spec> {
        Reserved1W::new(self, 9)
    }
    #[doc = "Bit 16 - 16:16\\]
Send Checksum This mode is effective in LIN mode only. This bit is used by the transmitter with extended frames to send a checkbyte. In non multibuffer mode the checkbyte will be sent after the current byte transmission. In multibuffer mode the checkbyte will be sent after the last byte count, indicated by the SCIFORMAT\\[18:16\\]). This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn sc(&mut self) -> ScW<Scigcr2Spec> {
        ScW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Compare Checksum. This mode is effective in LIN mode only. This bit is used by the receiver for extended frames to trigger a checksum compare. The user will initiate this transaction by writing a one to this bit. In non multibuffer mode, once the CC bit is set, the checksum will be compared on the byte that is currently being received, expected to be the checkbyte. During Multi-buffer mode, following are the scenarios associated with the CC bit : - If CC bit is set during the reception of the data, then the byte that is received after the reception of the programmed no. of data bytes indicated by SCIFORMAT\\[18:16\\], is treated as a checksum byte. - If CC bit is set during the IDLE period (i.e. during inter-frame space), then the next immediate byte will be treated as a checksum byte. A CE will immediatly be flagged if there is a checksum error. This bit is automatically cleared once the checksum is successfully compared."]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CcW<Scigcr2Spec> {
        CcW::new(self, 17)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Scigcr2Spec> {
        Reserved2W::new(self, 18)
    }
}
#[doc = "The SCIGCR2 register is used to send or compare a checksum byte during extended frames, to generate a wakeup and for low-power mode control of the LIN module.\n\nYou can [`read`](crate::Reg::read) this register and get [`scigcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scigcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scigcr2Spec;
impl crate::RegisterSpec for Scigcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scigcr2::R`](R) reader structure"]
impl crate::Readable for Scigcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`scigcr2::W`](W) writer structure"]
impl crate::Writable for Scigcr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIGCR2 to value 0"]
impl crate::Resettable for Scigcr2Spec {
    const RESET_VALUE: u32 = 0;
}
